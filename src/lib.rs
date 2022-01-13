#[cfg(feature = "ancestors")]
use std::collections::BTreeSet;

mod context;
mod ent;
mod solution;
mod solution_id;
mod util;
extern crate ibis_macros;

pub use ent::Ent;
pub use ibis_macros::*;
pub use solution_id::Sol;
pub use util::*;

#[macro_export]
macro_rules! facts {
    ($runtime: expr $(, $name: ident ($($arg: expr ),*) )* $(,)? ) => {
        {
            use paste::paste;
            $(
                $runtime.extend(&[
                    paste!( [< $name Claim >]) (
                        $($arg, )*
                    ),
                ]);
            )*
        }
    }
}

#[macro_export]
macro_rules! ent {
    ($fmt: expr) => {
        Ent::by_name($fmt)
    };
    ($fmt: expr, $($names: expr),*) => {
        Ent::by_name(&format!($fmt, $( $names.name(), )*))
    }
}

#[macro_export]
macro_rules! apply {
    ($type: expr, $arg: expr) => {
        Ent::by_name(&format!("{}({})", $type.name(), $arg.name()))
    };
}

#[macro_export]
macro_rules! is_a {
    ($type: expr, $parent: expr) => {
        ($type.name().starts_with(&($parent.name() + "(")) && $type.name().ends_with(")"))
    };
}
