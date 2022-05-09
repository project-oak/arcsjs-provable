// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
#![allow(clippy::unused_unit)] // rustwasm/wasm-bindgen#2774 awaiting next `wasm-bindgen` release

mod context;
mod ent;
mod error;
mod solution_data;
mod solution_id;
mod type_parser;
mod type_parser_cache;
mod type_struct;
#[macro_use]
mod util;
#[cfg(feature = "dot")]
pub mod dot;
pub mod recipes;
#[cfg(feature = "dot")]
pub mod to_dot_impls;

pub use ent::Ent;
pub use error::IbisError;
pub use recipes::*;
pub use solution_data::SolutionData;
pub use solution_id::Sol;
pub use util::*;

use shadow_rs::shadow;
shadow!(build);

#[macro_export]
macro_rules! ent {
    ($fmt: expr) => {
        Ent::by_type(crate::type_parser_cache::read_type($fmt))
    };
}

#[macro_export]
macro_rules! apply {
    ($type: expr) => {
        crate::ent!($type)
    };
    ($type: expr, $($arg: expr),*) => {
        {{
             // TODO: Types should be made of ents to avoid cloning work.
            let mut ty = (*$type.get_type()).clone();
            let args = vec![$($arg.get_type(), )*];
            ty.args.extend(args);
            Ent::by_type(std::sync::Arc::new(ty))
        }}
    };
}

#[macro_export]
macro_rules! is_a {
    ($type: expr, $parent: expr) => {{
        let ty = $type.get_type();
        ty.name == $parent && !ty.args.is_empty()
    }};
}

#[macro_export]
macro_rules! name {
    ($type: expr) => {
        // TODO: remove this
        ent!(&$type.get_type().name)
    };
}

#[macro_export]
macro_rules! arg {
    ($type: expr, $ind: expr) => {{
        let ty = $type.get_type();
        let ind = $ind;
        if ind >= ty.args.len() {
            panic!("Cannot access argument {} of {}", ind, ty);
        }
        // Clones an Arc
        Ent::by_type(ty.args[ind].clone())
    }};
}

#[macro_export]
macro_rules! args {
    ($type: expr) => {{
        $type
            .get_type()
            .args
            .iter()
            .map(|arg| Ent::by_type(arg.clone()))
    }};
}

pub fn get_solutions(data: &str, loss: Option<usize>) -> Ibis {
    let mut runtime = Ibis::new();

    // TODO: Use ibis::Error and https://serde.rs/error-handling.html instead of expect.
    let recipes: Ibis = serde_json::from_str(data)
        .map_err(|e| {
            eprintln!("{}", data);
            e
        })
        .unwrap_or_else(|_| panic!("JSON Error in {}", data));
    runtime.add_recipes(recipes);

    runtime.extract_solutions_with_loss(loss)
}

pub fn version_info() -> String {
    build::version()
}

#[cfg(feature = "wasm")]
pub mod wasm {
    use wasm_bindgen::prelude::*;

    fn set_panic_hook() {
        console_error_panic_hook::set_once();
    }

    fn setup() {
        set_panic_hook();
    }

    #[wasm_bindgen]
    pub fn version_info() -> String {
        setup();
        super::version_info()
    }

    #[wasm_bindgen]
    pub fn best_solutions_to_json(data: &str) -> String {
        setup();
        super::best_solutions_to_json(data)
    }

    #[wasm_bindgen]
    pub fn all_solutions_to_json(data: &str) -> String {
        setup();
        super::all_solutions_to_json(data)
    }

    #[cfg(feature = "dot")]
    #[wasm_bindgen]
    pub fn best_solutions_to_dot(data: &str) -> String {
        setup();
        super::best_solutions_to_dot(data)
    }

    #[cfg(feature = "dot")]
    #[wasm_bindgen]
    pub fn all_solutions_to_dot(data: &str) -> String {
        setup();
        super::all_solutions_to_dot(data)
    }
}

pub fn best_solutions_to_json(data: &str) -> String {
    let solutions = get_solutions(data, Some(0));
    serde_json::to_string(&solutions).expect("Couldn't serialize Ibis output")
}

pub fn all_solutions_to_json(data: &str) -> String {
    let solutions = get_solutions(data, None);
    serde_json::to_string(&solutions).expect("Couldn't serialize Ibis output")
}

#[cfg(feature = "dot")]
pub fn best_solutions_to_dot(data: &str) -> String {
    use dot::ToDot;
    let solutions = get_solutions(data, Some(0));
    solutions.to_dot()
}

#[cfg(feature = "dot")]
pub fn all_solutions_to_dot(data: &str) -> String {
    use dot::ToDot;
    let solutions = get_solutions(data, None);
    solutions.to_dot()
}
