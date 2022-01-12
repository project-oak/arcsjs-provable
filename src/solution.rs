use std::collections::BTreeSet;

use crate::ids::*;

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct SolData {
    pub edges: BTreeSet<(Ent, Ent)>,
}

impl Default for SolData {
    fn default() -> Self {
        Self {
            edges: BTreeSet::new(),
        }
    }
}
