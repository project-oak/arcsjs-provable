use lazy_static::lazy_static;
use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::Mutex;
use super::ent::*;
use super::solution_id::*;
use super::solution::*;

pub struct Ctx {
    pub last_id: EntityIdBackingType,
    pub solution_id: SolutionIdBackingType,
    pub name_by_id: HashMap<Ent, String>,
    pub id_by_name: HashMap<String, Ent>,
    pub id_to_solution: HashMap<Sol, Solution>,
    pub solution_to_id: HashMap<Solution, Sol>,
    #[cfg(feature = "ancestors")]
    pub ancestors: HashMap<Sol, BTreeSet<Sol>>,
}

impl Ctx {
    fn new() -> Self {
        Self {
            last_id: 0,
            solution_id: 0, // zero is never used except for the 'empty' solution
            name_by_id: HashMap::new(),
            id_by_name: HashMap::new(),
            id_to_solution: HashMap::new(),
            solution_to_id: HashMap::new(),
            #[cfg(feature = "ancestors")]
            ancestors: HashMap::new(),
        }
    }
}

lazy_static! {
    pub static ref CTX: Mutex<RefCell<Ctx>> = Mutex::new(RefCell::new(Ctx::new()));
}

