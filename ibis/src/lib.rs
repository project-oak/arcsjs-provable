// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

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
pub mod recipies;
extern crate ibis_macros;

pub use ent::Ent;
pub use error::IbisError;
pub use ibis_macros::*;
pub use recipies::*;
pub use solution_data::SolutionData;
pub use solution_id::Sol;
pub use util::*;

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
    ($type: expr, $parent: expr) => {
        {
            use crate::type_parser::read_type;
            let name = $type.name();
            let ty = read_type(&name);
            ty.name == $parent.name()
        }
    };
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
