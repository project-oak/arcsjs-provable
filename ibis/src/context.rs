// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use super::ent::*;
use super::solution::*;
use super::solution_id::*;
use super::util::BiMap;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::sync::Mutex;

pub struct Ctx {
    pub last_id: EntityIdBackingType,
    pub solution_id: SolutionIdBackingType,
    // TODO: Consider using https://docs.rs/bimap/latest/bimap/
    pub id_to_name: BiMap<Ent, String>,
    pub id_to_solution: BiMap<Sol, Solution>,
    #[cfg(feature = "ancestors")]
    pub ancestors: HashMap<Sol, BTreeSet<Sol>>,
}

impl Ctx {
    fn new() -> Self {
        Self {
            last_id: 0,
            solution_id: 0, // zero is never used except for the 'empty' solution
            id_to_name: BiMap::new(),
            id_to_solution: BiMap::new(),
            #[cfg(feature = "ancestors")]
            ancestors: HashMap::new(),
        }
    }
}

lazy_static! {
    pub static ref CTX: Mutex<RefCell<Ctx>> = Mutex::new(RefCell::new(Ctx::new()));
}
