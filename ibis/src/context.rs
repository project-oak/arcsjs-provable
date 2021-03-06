// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use crate::type_struct::Type;

use super::ent::*;
use super::solution_data::SolutionData;
use super::solution_id::*;
use super::util::BiMap;
use lazy_static::lazy_static;
#[cfg(feature = "ancestors")]
use std::collections::{BTreeSet, HashMap};
use std::sync::Mutex;
use std::{cell::RefCell, sync::Arc};

pub struct Ctx {
    pub last_id: EntityIdBackingType,
    pub solution_id: SolutionIdBackingType,
    // TODO: Consider using https://docs.rs/bimap/latest/bimap/
    pub id_to_type: BiMap<Ent, Arc<Type>>,
    pub id_to_solution: BiMap<Sol, Arc<SolutionData>>,
    #[cfg(feature = "ancestors")]
    pub ancestors: HashMap<Sol, BTreeSet<Sol>>,
}

impl Ctx {}

impl Ctx {
    fn new() -> Self {
        Self {
            last_id: 0,
            solution_id: 0, // zero is never used except for the 'empty' solution
            id_to_type: BiMap::new(),
            id_to_solution: BiMap::new(),
            #[cfg(feature = "ancestors")]
            ancestors: HashMap::new(),
        }
    }
}

lazy_static! {
    pub static ref CTX: Mutex<RefCell<Ctx>> = Mutex::new(RefCell::new(Ctx::new()));
}
