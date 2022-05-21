// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
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
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde()]
pub struct Link {
    source: EntityIdBackingType,
    target: EntityIdBackingType,
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
            });
        }
        for particle in particles {
            d3.add_node(D3Node {
                id: particle.id,
                name: format!("{}", particle),
            });
        }
        let sol = &recipe.id.unwrap_or_else(Sol::empty).solution();
        for (source, target) in &sol.edges {
            d3.add_link(Link {
                source: source.id,
                target: target.id,
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
