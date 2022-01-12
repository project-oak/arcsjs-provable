pub type EntId = u64;

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Hash)]
pub struct Ent {
    pub id: EntId,
}

pub type SolId = u32;

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Hash)]
pub struct Sol {
    pub id: SolId,
}
