// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use std::collections::BTreeSet;

use crate::ent::*;

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct Solution {
    pub edges: BTreeSet<(Ent, Ent)>,
}

impl Solution {
    pub fn has_edge(&self, from: Ent, to: Ent) -> bool {
        self.edges.contains(&(from, to))
    }

    pub fn add_edge(&self, from: Ent, to: Ent) -> Solution {
        let mut edges = self.edges.clone();
        edges.insert((from, to));
        Solution { edges }
    }
}
