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
#[cfg(feature = "d3")]
pub mod d3;
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
macro_rules! name {
    ($type: expr) => {
        // TODO: remove this
        ent!(&$type.get_type().name)
    };
}

pub fn run_ibis(data: &str) -> Ibis {
    get_solutions(data, Some(0))
}

pub fn all_solutions(data: &str) -> Ibis {
    get_solutions(data, None)
}

pub fn get_solutions(data: &str, loss: Option<usize>) -> Ibis {
    let mut runtime = Ibis::new();

    // TODO: Use ibis::Error and https://serde.rs/error-handling.html instead of expect.
    let recipes: Ibis = serde_json::from_str(data)
        .map_err(|e| {
            eprintln!("{}", data);
            e
        })
        .unwrap_or_else(|e| panic!("JSON Error: {}. In {}", e, data));
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
    pub fn run_ibis(data: &str) -> String {
        setup();
        let solutions = super::run_ibis(data);
        serde_json::to_string(&solutions).expect("Couldn't serialize Ibis output")
    }

    #[wasm_bindgen]
    pub fn all_solutions(data: &str) -> String {
        setup();
        let solutions = super::all_solutions(data);
        serde_json::to_string(&solutions).expect("Couldn't serialize Ibis output")
    }
}
