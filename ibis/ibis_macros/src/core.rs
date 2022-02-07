// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use ibis::Sol;
use std::collections::HashMap;

type Ibis=Crepe;

pub trait ToInput {
    type U;
    fn to_claim(self) -> Self::U;
}

impl <T: ToInput + Clone> ToInput for &T {
    type U = T::U;

    fn to_claim(self) -> Self::U {
        self.clone().to_claim()
    }
}

#[derive(Default)]
struct DotGraph {
    nodes: Vec<String>,
    edges: Vec<(String, String, Vec<String>)>,
    children: Vec<(String, String, DotGraph)>,
}

impl DotGraph {
    fn add_node(&mut self, node: String) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, from: String, to: String, attrs: Vec<String>) {
        self.edges.push((from, to, attrs));
    }

    fn add_child(&mut self, name: String, label: String, child: DotGraph) {
        self.children.push((name, label, child));
    }

    fn to_dot(self) -> String {
        format!("digraph solutions {{compound=true; {} }}", self.to_dot_items())
    }

    fn to_dot_items(self) -> String {
        let mut items: Vec<String> = vec![];

        for node in self.nodes {
            items.push(node+";");
        }

        for edge in self.edges {
            items.push(format!("{} -> {}[{}];", edge.0, edge.1, edge.2.join(" ")));
        }
        for (name, label, child) in self.children {
            items.push(format!("subgraph cluster_{name} {{ {} color=\"#00000070\"; label=\"{label}\"}}", child.to_dot_items(), name=name, label=label));
        }
        items.join("")
    }
}

fn sol_id(sol: &Sol) -> String {
    format!("sol_{}", &sol.id)
}

fn the_empty_solution() -> Vec<Sol> {
    vec![Sol::empty()]
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
struct Recipe {
    #[serde(default)]
    metadata: serde_json::Value,
    #[serde(default = "the_empty_solution")]
    solutions: Vec<Sol>,
    #[serde(default)]
    types: Vec<Type>,
    #[serde(default)]
    subtypes: Vec<Subtype>,
    #[serde(default)]
    nodes: Vec<Node>,
    #[serde(default)]
    claims: Vec<Claim>,
    #[serde(default)]
    checks: Vec<Check>,
    #[serde(default)]
    has_tags: Vec<HasTag>,
    #[serde(default)]
    less_private_than: Vec<LessPrivateThan>,
    #[serde(default)]
    trusted_to_remove_tag: Vec<TrustedToRemoveTag>,
    #[serde(default)]
    leaks: Vec<Leak>,
    #[serde(default)]
    type_errors: Vec<TypeError>,
    #[serde(default)]
    edges: Vec<Edge>,
}

impl Recipe {
    pub fn to_dot(self: &Self) -> String {
        self.to_dot_repr().to_dot()
    }

    fn to_dot_repr(self: &Self) -> DotGraph {
        let mut g = DotGraph::default();

        let mut max = 0;
        let mut best = None;
        for s in &self.solutions {
            let l = s.edges().len();
            if l > max {
                best = Some(*s);
                max = l;
            }
        }
        // let solutions = best;
        for s in &self.solutions {
            let s_id = sol_id(s);
            let particle_id = |particle| format!("{}_p{}", &s_id, particle);
            let node_id = |node| format!("{}_h{}", &s_id, node);
            let mut sol_graph = DotGraph::default();
            let mut particles = HashMap::new();
            for Node(particle, node, ty) in &self.nodes {
                let mut extras: Vec<String> = vec![];
                for HasTag(hts, source, sink, tag) in &self.has_tags {
                    if hts == s && sink == node && source != node {
                        extras.push(format!("'{}' from {}", tag, source));
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
                        extras.push(format!("<font color=\"blue\">checked to be '{}'</font>", tag));
                    }
                }
                let extras: Vec<String> = extras.iter().map(|ex| format!("<tr><td>{}</td></tr>", ex)).collect();
                let mut particle_g = particles.entry(particle).or_insert(DotGraph::default());
                particle_g.add_node(format!("{node_id} [shape=record label=< <table border=\"0\"><tr><td>{node} : {ty}</td></tr>{extras}</table>>]", node_id=node_id(node), node=node, ty=ty, extras=extras.join("")));
            }
            for (particle, mut particle_g) in particles {
                sol_graph.add_child(particle_id(particle), format!("{} : Particle", particle), particle_g);
            }

            for Leak(leak_s, node, expected, source, tag) in &self.leaks {
                if leak_s == s {
                    sol_graph.add_edge(node_id(source), node_id(node), vec![format!("style=dotted color=red label=<<font color=\"red\">expected '{}', found contradiction '{}'</font>>", expected, tag)]);
                }
            }

            for TypeError(error_s, from, from_ty, to, to_ty) in &self.type_errors {
                if error_s == s {
                    sol_graph.add_edge(node_id(from), node_id(to), vec![format!("style=dotted color=red label=<<font color=\"red\">expected '{}', found incompatible type '{}'</font>>", to_ty, from_ty)]);
                }
            }

            for Edge(es, from_particle, from_id, to_particle, to_id) in &self.edges {
                if es != s {
                    continue;
                }
                let from = format!("{}:s", node_id(from_id));
                let to = format!("{}:n", node_id(to_id));
                sol_graph.add_edge(
                    from.clone(),
                    to.clone(),
                    vec![]
                );
            }
            #[cfg(feature = "ancestors")]
            {
                let solution_head = |sol| format!("{}_head", sol_id(sol));
                sol_graph.add_node(format!("{}[style=invis height = 0 width = 0 label=\"\"]", solution_head(s)));
                for ancestor in &s.ancestors() {
                    g.add_edge(solution_head(&s), solution_head(ancestor), vec![format!("ltail=cluster_{} lhead=cluster_{}", &s_id, sol_id(ancestor))]);
                }
            }
            g.add_child(s_id.clone(), format!("Solution {}", &s.id), sol_graph);
        }
        g
    }
}

impl Crepe {
    pub fn add_data<T: ToInput, Iter: IntoIterator<Item=T>>(&mut self, data: Iter) where Crepe: Extend<T::U> {
        self.extend(data.into_iter().map(|datum|datum.to_claim()));
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        // Note: solutions and edges must be handled differently
        //for solution in recipe.solutions
        //for Edge(sol, from_particle, from, to_particle, to) in self.edges() {
            //sol.add_edge();
        //}
        self.add_data(recipe.solutions.iter().map(|s|Solution(*s)));
        self.add_data(&recipe.types);
        self.add_data(&recipe.subtypes);
        self.add_data(&recipe.nodes);
        self.add_data(&recipe.claims);
        self.add_data(&recipe.checks);
        self.add_data(&recipe.leaks);
        self.add_data(&recipe.type_errors);
        self.add_data(&recipe.has_tags);
        self.add_data(&recipe.less_private_than);
        self.add_data(&recipe.trusted_to_remove_tag);
    }

    pub fn extract_solutions(self) -> Recipe {
        let (mut solutions, mut types, mut nodes, mut claims, mut checks, mut has_tags, mut less_private_than, mut leaks, mut type_errors, mut subtypes, mut trusted_to_remove_tag, mut edges) = self.run();
        Recipe {
            metadata: serde_json::Value::Null,
            solutions: solutions.drain().map(|Solution(s)| s).collect(),
            types: types.drain().collect(),
            nodes: nodes.drain().collect(),
            claims: claims.drain().collect(),
            checks: checks.drain().collect(),
            has_tags: has_tags.drain().collect(),
            less_private_than: less_private_than.drain().collect(),
            leaks: leaks.drain().collect(),
            type_errors: type_errors.drain().collect(),
            subtypes: subtypes.drain().collect(),
            trusted_to_remove_tag: trusted_to_remove_tag.drain().collect(),
            edges: edges.drain().collect(),
        }
    }
}
