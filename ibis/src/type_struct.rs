// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Type {
    pub name: String,
    pub args: Vec<Type>,
}

impl Type {
    pub fn with_args(mut name: &str, args: Vec<Type>) -> Self {
        if name == "*" {
            name = "ibis.UniversalType";
        }
        Self { name: name.to_string(), args }
    }
    pub fn new(name: &str) -> Self {
        Self::with_args(name, vec![])
    }
    pub fn with_arg(mut self, arg: Type) -> Self {
        self.args.push(arg);
        self
    }
    pub fn with_capability(self, cap: &str) -> Self {
        Type::new("ibis.WithCapability")
            .with_arg(Type::new(cap))
            .with_arg(self)
    }
}

fn format_arg_set(
    f: &mut std::fmt::Formatter<'_>,
    joiner: &str,
    args: &[Type],
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
        if self.name == "ibis.WithCapability" && self.args.len() > 1 {
            write!(f, "{} ", self.args[0])?;
            if self.args.len() > 2 {
                write!(f, "(")?;
            }
            format_arg_set(f, ", ", &self.args[1..])?;
            if self.args.len() > 2 {
                write!(f, ")")?;
            }
            Ok(())
        } else if self.name == "ibis.Labelled" && self.args.len() > 1 {
            write!(f, "{}: ", self.args[0])?;
            if self.args.len() > 2 {
                write!(f, "(")?;
            }
            format_arg_set(f, ", ", &self.args[1..])?;
            if self.args.len() > 2 {
                write!(f, ")")?;
            }
            Ok(())
        } else if self.name == "ibis.ProductType" && !self.args.is_empty() {
            write!(f, "{{")?;
            format_arg_set(f, ", ", &self.args)?;
            write!(f, "}}")
        } else {
            let res = write!(
                f,
                "{}",
                if self.name == "ibis.UniversalType" {
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
