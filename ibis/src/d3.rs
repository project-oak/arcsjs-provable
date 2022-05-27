// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
use crate::type_struct::WITH_CAPABILITY;
use crate::ent::EntityIdBackingType;
use crate::recipes::{Ibis, Node, Recipe};
use crate::{Ent, Sol};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde()]
pub struct D3Node {
    id: EntityIdBackingType,
    name: String,
    group: EntityIdBackingType,
    kind: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde()]
pub struct Link {
    source: EntityIdBackingType,
    target: EntityIdBackingType,
    kind: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde()]
pub struct D3Graph {
    nodes: Vec<D3Node>,
    links: Vec<Link>,
}

pub trait ToD3 {
    fn to_d3(&self) -> D3Graph;
}

impl D3Graph {
    pub fn add_node(&mut self, node: D3Node) {
        self.nodes.push(node);
    }

    pub fn add_link(&mut self, link: Link) {
        self.links.push(link);
    }

    // TODO: Might not need this but it seems useful.
    pub fn map_ids<F: Fn(u64) -> u64>(&mut self, f: &F) {
        for node in &mut self.nodes {
            node.id = f(node.id);
        }
        for link in &mut self.links {
            link.target = f(link.target);
            link.source = f(link.source);
        }
    }

    // Usage: to merge disconnected subgraphs, use
    // let mut result = D3Graph::default();
    // for (i, graph) of graphs.drain(0..).ennumerate() {
    //      graph.scale_and_offset_ids(graphs.len(), i);
    //      result.merge(graph);
    // }
    pub fn scale_and_offset_ids(&mut self, mul: u64, offset: u64) {
        let f = |x| mul*x + offset;
        self.map_ids(&f);
    }

    pub fn merge(&mut self, other: Self) {
        self.nodes.extend(other.nodes);
        self.links.extend(other.links);
    }
}

impl ToD3 for (&Ibis, &Recipe) {
    fn to_d3(&self) -> D3Graph {
        let (ibis, recipe) = &self;
        let mut d3 = D3Graph::default();

        let mut particles: HashSet<Ent> = HashSet::new();

        for Node ( particle, node, ty ) in &ibis.shared.nodes {
            particles.insert(*particle);
            d3.add_node(D3Node {
                id: node.id,
                name: format!("{}: {}", node, ty),
                group: particle.id,
                kind: "handle".to_string(),
            });
            // if ty.is_a(WITH_CAPABILITY) && ty.args()[0].is_a("write") {
            d3.add_link(Link {
                source: particle.id,
                target: node.id,
                kind: "handle_in_particle".to_string(),
            });
            // }
            // if ty.is_a(WITH_CAPABILITY) && ty.args()[0].is_a("read") {
                // d3.add_link(Link {
                    // source: node.id,
                    // target: particle.id,
                // });
            // }
        }
        for particle in particles {
            d3.add_node(D3Node {
                id: particle.id,
                name: format!("{}", particle),
                group: particle.id,
                kind: "particle".to_string(),
            });
        }
        let sol = &recipe.id.unwrap_or_else(Sol::empty).solution();
        for (source, target) in &sol.edges {
            d3.add_link(Link {
                source: source.id,
                target: target.id,
                kind: "connection".to_string(),
            });
        }

        use crate::recipes::TypeError;
        for TypeError(_sol, source, _expected_ty, target, _found_ty) in &recipe.feedback.type_errors {
            d3.add_link(Link {
                source: source.id,
                target: target.id,
                kind: "type_error".to_string(),
            });
        }

        use crate::recipes::Leak;
        for Leak(_sol, source, _expected_tag, target, _found_tag) in &recipe.feedback.leaks {
            d3.add_link(Link {
                source: source.id,
                target: target.id,
                kind: "leak".to_string(),
            });
        }

        d3
    }
}

impl ToD3 for Ibis {
    fn to_d3(&self) -> D3Graph {
        let mut d3 = D3Graph::default();

        for recipe in &self.recipes {
            let recipe_d3 = (self, recipe).to_d3();
            d3.merge(recipe_d3);
        }
        d3
    }
}
