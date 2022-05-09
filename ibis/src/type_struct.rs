// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
use std::sync::Arc;

pub const UNIVERSAL: &str = "ibis.UniversalType";
pub const WITH_CAPABILITY: &str = "ibis.WithCapability";
pub const PRODUCT: &str = "ibis.ProductType";
pub const UNION: &str = "ibis.UnionType";
pub const GENERIC: &str = "ibis.GenericType";
pub const INDUCTIVE: &str = "ibis.InductiveType";
pub const LABELLED: &str = "ibis.Labelled";

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Type {
    pub name: String,
    pub args: Vec<Arc<Type>>,
}

impl Type {
    pub fn new(mut name: &str) -> Self {
        if name == "*" {
            name = UNIVERSAL;
        }
        Self {
            name: name.to_string(),
            args: vec![],
        }
    }
    pub fn with_args<T: Into<Arc<Type>>>(mut self, mut args: Vec<T>) -> Self {
        self.args.extend(args.drain(0..).map(|arg| arg.into()));
        self
    }
    pub fn with_arg<T: Into<Arc<Type>>>(mut self, arg: T) -> Self {
        self.args.push(arg.into());
        self
    }
    pub fn with_capability(self, cap: &str) -> Self {
        Self::new(WITH_CAPABILITY).with_args(vec![Type::new(cap), self])
    }
}

fn format_arg_set(
    f: &mut std::fmt::Formatter<'_>,
    joiner: &str,
    args: &[Arc<Type>],
) -> std::fmt::Result {
    if let Some(first) = args.first() {
        write!(f, "{}", first)?;
    }
    for arg in args[1..].iter() {
        write!(f, "{}{}", joiner, arg)?;
    }
    Ok(())
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.name == WITH_CAPABILITY && self.args.len() > 1 {
            write!(f, "{} ", self.args[0])?;
            if self.args.len() > 2 {
                write!(f, "(")?;
            }
            format_arg_set(f, ", ", &self.args[1..])?;
            if self.args.len() > 2 {
                write!(f, ")")?;
            }
            Ok(())
        } else if self.name == LABELLED && self.args.len() > 1 {
            write!(f, "{}: ", self.args[0])?;
            if self.args.len() > 2 {
                write!(f, "(")?;
            }
            format_arg_set(f, ", ", &self.args[1..])?;
            if self.args.len() > 2 {
                write!(f, ")")?;
            }
            Ok(())
        } else if self.name == PRODUCT && !self.args.is_empty() {
            write!(f, "{{")?;
            format_arg_set(f, ", ", &self.args)?;
            write!(f, "}}")
        } else {
            let res = write!(
                f,
                "{}",
                if self.name == UNIVERSAL {
                    "*"
                } else {
                    &self.name
                }
            )?;
            if self.args.is_empty() {
                Ok(res)
            } else {
                write!(f, "(")?;
                format_arg_set(f, ", ", &self.args)?;
                write!(f, ")")
            }
        }
    }
}
