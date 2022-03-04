// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use crate::dot::{DotGraph, ToDot};
use crate::recipes::{Check, Claim, HasTag, Ibis, Leak, Node, TrustedToRemoveTag, Recipe, TypeError};
use crate::Sol;
use std::collections::HashMap;

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
        for recipe in solutions {
            let sol = &recipe.id.expect("Every recipe should have an id?");
            let s_id = sol_id(sol);
            #[allow(unused_mut)]
            let mut sol_graph = recipe.to_dot_repr();
            #[cfg(feature = "ancestors")]
            {
                let s = Sol::from(recipe);
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

impl ToDot for Recipe {
    fn to_dot_repr(&self) -> DotGraph {
        let sol = &self.id.expect("Every recipe should have an id?");
        let s_id = sol_id(sol);
        let particle_id = |particle| format!("{}_p_{}", &s_id, particle);
        let node_id = |node| format!("{}_h_{}", &s_id, node);
        let mut sol_graph = DotGraph::default();
        let mut particles = HashMap::new();
        for Node(particle, node, cap, ty) in &self.nodes {
            let mut extras: Vec<String> = vec![];
            if let Some(feedback) = &self.feedback {
                for HasTag(_hts, source, sink, tag) in &feedback.has_tags {
                    if sink == node && source != node {
                        extras.push(format!("'{}' from {}", tag, source));
                    }
                }
            }
            for TrustedToRemoveTag(trusted_n, tag) in &self.trusted_to_remove_tag {
                if trusted_n == node {
                    extras.push(format!("trusted to remove tag '{}'", tag));
                }
            }
            for Claim(claim_node, tag) in &self.claims {
                if claim_node == node {
                    extras.push(format!("claims to be '{}'", tag));
                }
            }
            for Check(check_node, tag) in &self.checks {
                if check_node == node {
                    extras.push(format!(
                        "<font color=\"blue\">checked to be '{}'</font>",
                        tag
                    ));
                }
            }
            let extras: Vec<String> = extras
                .iter()
                .map(|ex| format!("<tr><td>{}</td></tr>", ex))
                .collect();
            let particle_g = particles.entry(particle).or_insert_with(DotGraph::default);
            particle_g.add_node(format!("{node_id} [shape=record label=< <table border=\"0\"><tr><td>{cap} {node} : {ty}</td></tr>{extras}</table>>]", node_id=node_id(node), node=node, cap=cap, ty=ty, extras=extras.join("")));
        }
        for (particle, particle_g) in particles {
            sol_graph.add_child(
                particle_id(particle),
                format!("{} : Particle", particle),
                particle_g,
            );
        }

        if let Some(feedback) = &self.feedback {
            for Leak(_leak_s, node, expected, source, tag) in &feedback.leaks {
                sol_graph.add_edge(node_id(source), node_id(node), vec![format!("style=dotted color=red label=<<font color=\"red\">expected '{}', found contradiction '{}'</font>>", expected, tag)]);
            }

            for TypeError(_error_s, from, from_ty, to, to_ty) in &feedback.type_errors {
                sol_graph.add_edge(node_id(from), node_id(to), vec![format!("style=dotted color=red label=<<font color=\"red\">expected '{}', found incompatible type '{}'</font>>", to_ty, from_ty)]);
            }
        }

        for (from_id, to_id) in &self.id.expect("WAT").edges() {
            let from = format!("{}:s", node_id(from_id));
            let to = format!("{}:n", node_id(to_id));
            sol_graph.add_edge(from.clone(), to.clone(), vec![]);
        }
        sol_graph
    }
}
