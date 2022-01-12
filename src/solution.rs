use std::collections::BTreeSet;

use crate::ids::*;

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SolData {
    pub edges: BTreeSet<(Ent, Ent)>,
}
