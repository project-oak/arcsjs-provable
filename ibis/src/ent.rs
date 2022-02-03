// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use super::context::{Ctx, CTX};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

pub type EntityIdBackingType = u64;

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub struct Ent {
    pub id: EntityIdBackingType,
}

impl From<Ent> for String {
    fn from(ent: Ent) -> Self {
        ent.name()
    }
}

impl From<String> for Ent {
    fn from(id: String) -> Self {
        Self::by_name(&id)
    }
}

impl Ent {
    fn new(ctx: &mut Ctx, name: &str) -> Self {
        let id = ctx.last_id;
        ctx.last_id += 1;
        let ent = Ent { id };
        ctx.id_to_name.insert(ent, name.to_string());
        ent
    }

    pub fn name(&self) -> String {
        let guard = CTX.lock().expect("Shouldn't fail");
        let ctx = (*guard).borrow();
        ctx.borrow()
            .id_to_name
            .get(self)
            .cloned()
            .expect("All entities should have a name")
    }

    fn get_by_name(ctx: &mut Ctx, name: &str) -> Option<Ent> {
        ctx.id_to_name.get_back(name).cloned()
    }

    pub fn by_name(name: &str) -> Ent {
        let guard = CTX.lock().expect("Shouldn't fail");
        let mut ctx = (*guard).borrow_mut();
        Ent::get_by_name(&mut ctx, name).unwrap_or_else(|| Ent::new(&mut ctx, name))
    }
}

impl std::fmt::Display for Ent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::fmt::Debug for Ent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ent")
            .field("id", &self.id)
            .field("{name}", &self.name())
            .finish()
    }
}
