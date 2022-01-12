pub type EntId = u64;

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Hash)]
pub struct Ent {
    pub id: EntId,
}

pub type SolId = u32;

#[derive(Copy, Clone, PartialOrd, Ord, Eq, Hash)]
pub enum Sol {
    Any,
    Id { id: SolId },
}

impl PartialEq for Sol {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Sol::Any, _) => true,
            (_, Sol::Any) => true,
            (Sol::Id { id: self_id }, Sol::Id { id: other_id }) => self_id == other_id,
        }
    }
}
