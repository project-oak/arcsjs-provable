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
mod type_struct;
#[macro_use]
mod util;
#[cfg(feature = "dot")]
pub mod dot;
#[cfg(feature = "dot")]
pub mod recipe_to_dot;
pub mod recipes;
extern crate ibis_macros;

pub use ent::Ent;
pub use error::IbisError;
pub use ibis_macros::*;
pub use recipes::*;
pub use solution_data::SolutionData;
pub use solution_id::Sol;
pub use util::*;

use shadow_rs::shadow;
shadow!(build);

#[macro_export]
macro_rules! ent {
    ($fmt: expr) => {
        Ent::by_name($fmt)
    };
    ($fmt: expr, $($names: expr),*) => {
        Ent::by_name(&format!($fmt, $( $names, )*))
    }
}

#[macro_export]
macro_rules! apply {
    ($type: expr) => {
        crate::ent!($type)
    };
    ($type: expr, $($arg: expr),*) => {
        {
            let args: Vec<String> = vec![$($arg.name(),)*];
            crate::ent!("{}({})", $type, args.join(", "))
        }
    };
}

#[macro_export]
macro_rules! is_a {
    ($type: expr, $parent: expr) => {{
        use crate::type_parser::read_type;
        let name = $type.name();
        let ty = read_type(&name);
        ty.name == $parent.name()
    }};
}

#[macro_export]
macro_rules! arg {
    ($type: expr, $ind: expr) => {{
        use crate::type_parser::read_type;
        let name = $type.name();
        let ty = read_type(&name);
        ent!(&format!("{}", ty.args[$ind]))
    }};
}

pub trait ToInput {
    type U;
    fn to_claim(self) -> Self::U;
}

impl<T: ToInput + Clone> ToInput for &T {
    type U = T::U;

    fn to_claim(self) -> Self::U {
        self.clone().to_claim()
    }
}

pub fn get_solutions(data: &str, loss: Option<usize>) -> Ibis {
    let mut runtime = Ibis::new();

    // TODO: Use ibis::Error and https://serde.rs/error-handling.html instead of expect.
    let recipes: Ibis = serde_json::from_str(data).expect("JSON Error?");
    runtime.add_recipes(recipes);

    eprintln!("Preparing graph...");
    runtime.extract_solutions_with_loss(loss)
}

pub fn print_info() {
    eprintln!("{}", build::version());
}

#[cfg(feature = "wasm")]
pub mod wasm {
    use wasm_bindgen::prelude::*;

    fn set_panic_hook() {
        console_error_panic_hook::set_once();
    }

    fn setup() {
        set_panic_hook();
        super::print_info();
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
