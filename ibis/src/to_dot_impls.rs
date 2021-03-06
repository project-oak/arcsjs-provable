// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use crate::dot::{DotGraph, ToDot};
use crate::recipes::{
    Check, Claim, HasTag, Ibis, Leak, Node, Recipe, TrustedToRemoveTag, TrustedToRemoveTagFromNode,
    TypeError,
};
use crate::Sol;
use std::collections::{HashMap, HashSet};

impl ToDot for Ibis {
    fn to_dot_repr(&self) -> DotGraph {
        let mut g = DotGraph::default();

        let solutions = if true {
            self.recipes.iter().collect()
        } else {
            let mut max = 0;
            let mut best = None;
            for s in &self.recipes {
                let l = s.edges.len();
                if l > max {
                    best = Some(s);
                    max = l;
                }
            }
            vec![best.expect("Expected a 'best' solution")]
        };
        for recipe in solutions.iter() {
            let sol = &recipe.id.unwrap_or_else(Sol::empty);
            let s_id = sol_id(sol);
            #[allow(unused_mut)]
            let mut sol_graph = (self, *recipe).to_dot_repr();
            #[cfg(feature = "ancestors")]
            {
                let s = Sol::from(*recipe);
                let solution_head = |sol| format!("{}_head", sol_id(sol));
                sol_graph.add_node(format!(
                    "{}[style=invis height = 0 width = 0 label=\"\"]",
                    solution_head(&s)
                ));
                for ancestor in &s.ancestors() {
                    g.add_edge(
                        solution_head(&s),
                        solution_head(ancestor),
                        vec![format!(
                            "ltail=cluster_{} lhead=cluster_{}",
                            &s_id,
                            sol_id(ancestor)
                        )],
                    );
                }
            }
            g.add_child(s_id.clone(), format!("Solution {}", &sol.id), sol_graph);
        }
        g
    }
}

fn sol_id(sol: &Sol) -> String {
    format!("sol_{}", &sol.id)
}

impl ToDot for (&Ibis, &Recipe) {
    fn to_dot_repr(&self) -> DotGraph {
        let (ibis, recipe) = &self;
        let sol = &recipe.id.unwrap_or_else(Sol::empty);
        let s_id = sol_id(sol);
        let particle_id = |particle| format!("{}_p_{}", &s_id, particle);
        let node_id = |node| format!("{}_h_{}", &s_id, node).replace('.', "_");
        let mut sol_graph = DotGraph::default();
        let mut particles = HashMap::new();
        for Node(particle, node, ty) in &ibis.shared.nodes {
            let mut extras: HashSet<String> = HashSet::new();
            let mut tags: HashMap<String, Vec<String>> = HashMap::new();
            for HasTag(_hts, source, sink, tag) in &recipe.feedback.has_tags {
                if sink == node && source != node {
                    tags.entry(tag.to_string())
                        .or_insert(vec![])
                        .push(source.to_string());
                }
            }
            for TrustedToRemoveTag(trusted_n, tag) in &ibis.shared.trusted_to_remove_tag {
                if trusted_n == node {
                    extras.insert(format!(
                        "<font color=\"red\">trusted to drop tag '{}'</font>",
                        tag
                    ));
                }
            }
            for TrustedToRemoveTagFromNode(trusted_n, source_node) in
                &ibis.shared.trusted_to_remove_tag_from_node
            {
                if trusted_n == node {
                    extras.insert(format!(
                        "<font color=\"red\">trusted to drop tags from '{}'</font>",
                        source_node
                    ));
                }
            }
            for Claim(claim_node, tag) in &ibis.shared.claims {
                if claim_node == node {
                    extras.insert(format!(
                        "<font color=\"orange\">claims to be '{}'</font>",
                        tag
                    ));
                }
            }
            for Check(check_node, tag) in &ibis.shared.checks {
                if check_node == node {
                    extras.insert(format!(
                        "<font color=\"blue\">checked to be '{}'</font>",
                        tag
                    ));
                }
            }
            for (tag, sources) in &tags {
                extras.insert(format!(
                    "<font color=\"purple\">'{}' from {}</font>",
                    tag,
                    sources.join(", ")
                ));
            }
            let extras: Vec<String> = extras
                .iter()
                .map(|ex| format!("<tr><td>{}</td></tr>", ex))
                .collect();
            let particle_g = particles.entry(particle).or_insert_with(DotGraph::default);
            particle_g.add_node(format!("{node_id} [shape=record label=< <table border=\"0\"><tr><td>{node} : {ty}</td></tr>{extras}</table>>]", node_id=node_id(node), node=node, ty=ty, extras=extras.join("")));
        }
        for (particle, particle_g) in particles {
            sol_graph.add_child(particle_id(particle), format!("{}", particle), particle_g);
        }

        for Leak(_leak_s, node, expected, source, tag) in &recipe.feedback.leaks {
            sol_graph.add_edge(node_id(source), node_id(node), vec![format!("style=dotted color=red label=<<font color=\"red\">expected '{}', found '{}'</font>>", expected, tag)]);
        }

        for TypeError(_error_s, from, from_ty, to, to_ty) in &recipe.feedback.type_errors {
            sol_graph.add_edge(node_id(from), node_id(to), vec![format!("style=dotted color=red label=<<font color=\"red\">expected '{}', found '{}'</font>>", to_ty, from_ty)]);
        }

        let sol = &recipe.id.unwrap_or_else(Sol::empty).solution();
        for (from_id, to_id) in &sol.edges {
            let from = node_id(from_id).to_string();
            let to = node_id(to_id).to_string();
            sol_graph.add_edge(from, to, vec![]);
        }
        sol_graph
    }
}
