// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Type<'a> {
    pub name: &'a str,
    pub args: Vec<Type<'a>>,
}

impl<'a> Type<'a> {
    pub fn with_args(name: &'a str, args: Vec<Type<'a>>) -> Self {
        Self { name, args }
    }
    pub fn new(name: &'a str) -> Self {
        Self::with_args(name, vec![])
    }
    pub fn with_arg(mut self, arg: Type<'a>) -> Self {
        self.args.push(arg);
        self
    }
    pub fn with_capability(self, cap: &'a str) -> Self {
        Type::new("ibis.WithCapability")
            .with_arg(Type::new(cap))
            .with_arg(self)
    }
}

fn format_arg_set<'a>(
    f: &mut std::fmt::Formatter<'_>,
    joiner: &str,
    args: &[Type<'a>],
) -> std::fmt::Result {
    if let Some(first) = args.first() {
        write!(f, "{}", first)?;
    }
    for arg in args[1..].iter() {
        write!(f, "{}{}", joiner, arg)?;
    }
    Ok(())
}

impl<'a> std::fmt::Display for Type<'a> {
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
                    self.name
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
