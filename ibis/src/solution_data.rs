// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use crate::ent::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SolutionData {
    pub edges: BTreeSet<(Ent, Ent)>, // from, to

    // Starting data
    pub nodes: BTreeSet<Ent>,
    pub node_types: BTreeMap<Ent, Ent>,              // node, type
    pub node_to_particle: BTreeMap<Ent, Ent>,        // node, particle
    pub claims: BTreeSet<(Ent, Ent)>,                // node, tag
    pub checks: BTreeSet<(Ent, Ent)>,                // node, tag
    pub trusted_to_remove_tag: BTreeSet<(Ent, Ent)>, // node, tag
}

impl SolutionData {
    pub fn has_edge(&self, from: Ent, to: Ent) -> bool {
        self.edges.contains(&(from, to))
    }

    pub fn add_edge(&self, from: Ent, to: Ent) -> SolutionData {
        let mut n = SolutionData { ..self.clone() };
        n.edges.insert((from, to));
        n
    }

    pub fn add_node(&self, particle: Ent, node: Ent, ty: Ent) -> SolutionData {
        let mut n = SolutionData { ..self.clone() };
        n.nodes.insert(node);
        n.node_to_particle.insert(node, particle);
        n.node_types.insert(node, ty);
        n
    }

    pub fn is_trusted_to_remove_tag(&self, node: Ent, tag: Ent) -> bool {
        self.trusted_to_remove_tag.contains(&(node, tag))
    }
}
