// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use crate::type_parser::TypeParser;
use crate::type_struct::Type;
use lazy_static::lazy_static;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    pub static ref PARSE_CACHE: Mutex<RefCell<CachedTP>> =
        Mutex::new(RefCell::new(CachedTP::default()));
}

#[derive(Default)]
pub struct CachedTP {
    cache: HashMap<String, Arc<Type>>,
}

impl TypeParser for CachedTP {
    fn store_type(
        &mut self,
        input: &str,
        get_ty: impl FnOnce(&mut Self) -> Arc<Type>,
    ) -> Arc<Type> {
        if let Some(ty) = self.cache.get(input) {
            return ty.clone();
        }
        let ty = get_ty(self);
        self.cache.insert(input.to_string(), ty.clone());
        ty
    }
}

pub fn read_type(input: &str) -> Arc<Type> {
    let guard = PARSE_CACHE.lock().expect("Shouldn't fail");
    let mut ctx = (*guard).borrow_mut();
    ctx.borrow_mut().read_type(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_struct::*;

    pub fn read_type(input: &str) -> Arc<Type> {
        let mut tp = CachedTP::default();
        tp.read_type(input)
    }

    fn parse_and_round_trip(s: &str, t: Type) {
        let ty = read_type(s);
        assert_eq!(*ty, t);
        assert_eq!(format!("{}", ty), s);
    }

    #[test]
    fn read_a_type_multiple_times() {
        let mut tp = CachedTP::default();
        let a = tp.read_type("Type");
        let b = tp.read_type("Type");
        assert_eq!(a, b);
        // One for the cache, one for 'a' and one for 'b'.
        assert_eq!(Arc::strong_count(&a), 3);
        assert_eq!(Arc::strong_count(&b), 3);
    }

    #[test]
    fn read_a_type_multiple_times_via_nesting() {
        let a = read_type("And(Type, Type)");
        assert_eq!(a.args[0], a.args[1]);
        assert_eq!(Arc::strong_count(&a.args[0]), 2);
        assert_eq!(Arc::strong_count(&a.args[1]), 2);
    }

    #[test]
    fn read_a_simple_type_name() {
        parse_and_round_trip("Type", Type::new("Type"));
    }

    #[test]
    fn read_a_type_with_a_single_capabilities() {
        parse_and_round_trip("read Type", Type::new("Type").with_capability("read"));
    }

    #[test]
    fn read_a_type_with_multiple_capabilities() {
        parse_and_round_trip(
            "read write Type",
            Type::new("Type")
                .with_capability("write")
                .with_capability("read"),
        );
    }

    #[test]
    fn read_a_product_type_using_syntactic_sugar() {
        let name_string = read_type("{name: String}");
        let age_number = read_type("{age: Number}");
        parse_and_round_trip(
            "{name: String, age: Number}",
            Type::new(PRODUCT)
                .with_arg(name_string)
                .with_arg(age_number),
        );
    }

    #[test]
    fn read_nested_type() {
        let json = read_type("JSON");
        let age_number = read_type("{age: Number}");
        parse_and_round_trip(
            "name: {JSON, age: Number}",
            Type::new(LABELLED)
                .with_arg(Type::new("name"))
                .with_arg(Type::new(PRODUCT).with_arg(json).with_arg(age_number)),
        );
    }

    #[test]
    fn read_a_type_with_arguments() {
        parse_and_round_trip(
            "Type(a, b)",
            Type::new("Type")
                .with_arg(Type::new("a"))
                .with_arg(Type::new("b")),
        );
    }

    #[test]
    fn read_a_type_with_nested_arguments() {
        parse_and_round_trip(
            "Type(a(c), b)",
            Type::new("Type")
                .with_arg(Type::new("a").with_arg(Type::new("c")))
                .with_arg(Type::new("b")),
        );
    }

    #[test]
    fn read_type_with_label() {
        parse_and_round_trip(
            "name: Type",
            Type::new(LABELLED)
                .with_arg(Type::new("name"))
                .with_arg(Type::new("Type")),
        );
    }

    // TODO: tests for error messages
}
