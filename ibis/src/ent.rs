// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use crate::{type_parser_cache::read_type, type_struct::Type};

use super::context::{Ctx, CTX};
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, sync::Arc};

pub type EntityIdBackingType = u64;

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub struct Ent {
    pub id: EntityIdBackingType,
}

impl From<Ent> for String {
    fn from(ent: Ent) -> Self {
        format!("{}", ent)
    }
}

impl From<String> for Ent {
    fn from(id: String) -> Self {
        let ty = read_type(&id);
        Self::by_type(ty)
    }
}

impl Ent {
    fn new(ctx: &mut Ctx, ty: Arc<Type>) -> Self {
        let id = ctx.last_id;
        ctx.last_id += 1;
        let ent = Ent { id };
        ctx.id_to_type.insert(ent, ty);
        ent
    }

    pub fn get_type(&self) -> Arc<Type> {
        let guard = CTX.lock().expect("Shouldn't fail");
        let ctx = (*guard).borrow();
        ctx.borrow()
            .id_to_type
            .get(self)
            .cloned()
            .expect("All entities should have a type")
    }

    fn get_by_type(ctx: &mut Ctx, ty: &Type) -> Option<Ent> {
        ctx.id_to_type.get_back(ty).cloned()
    }

    pub fn by_type(ty: Arc<Type>) -> Ent {
        let guard = CTX.lock().expect("Shouldn't fail");
        let mut ctx = (*guard).borrow_mut();
        Ent::get_by_type(&mut ctx, &ty).unwrap_or_else(|| Ent::new(&mut ctx, ty))
    }
}

impl std::fmt::Display for Ent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_type())
    }
}

impl std::fmt::Debug for Ent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ent")
            .field("id", &self.id)
            .field("{repr}", &self.get_type())
            .finish()
    }
}
