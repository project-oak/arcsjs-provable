// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use crate::ent::*;
use std::collections::BTreeSet;

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SolutionData {
    // from, to
    pub edges: BTreeSet<(Ent, Ent)>,
    // TODO: Instances of: pub particle_instance: BTreeSet<(Ent, Ent)>, // from, to
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
}
