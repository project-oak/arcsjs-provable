use std::collections::BTreeSet;

use crate::ids::*;

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SolData {
    pub edges: BTreeSet<(Ent, Ent)>,
}

impl SolData {
    pub fn has_edge(&self, from: Ent, to: Ent) -> bool {
        self.edges.contains(&(from, to))
    }

    pub fn add_edge(&self, from: Ent, to: Ent) -> SolData {
        let mut edges = self.edges.clone();
        edges.insert((from, to));
        SolData { edges }
    }
}
