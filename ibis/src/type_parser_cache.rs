// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::Mutex;
use std::borrow::BorrowMut;
use crate::type_parser::read_type_uncached;
use crate::type_struct::Type;

lazy_static! {
    pub static ref PARSE_CACHE: Mutex<RefCell<HashMap<String, Type>>> = Mutex::new(RefCell::new(HashMap::new()));
}
pub fn read_type(input: &str) -> Type {
    let guard = PARSE_CACHE.lock().expect("Shouldn't fail");
    let mut ctx = (*guard).borrow_mut();
    ctx.borrow_mut()
        .entry(input.to_string())
        .or_insert_with(|| read_type_uncached(input))
        .clone()
}

