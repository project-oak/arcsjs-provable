// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Type<'a> {
    pub capabilities: Vec<&'a str>,
    pub name: &'a str,
    pub args: Vec<Type<'a>>,
}

impl<'a> Type<'a> {
    pub fn with_args(name: &'a str, args: Vec<Type<'a>>) -> Self {
        Self { capabilities: vec![], name, args }
    }
    pub fn new(name: &'a str) -> Self {
        Self::with_args(name, vec![])
    }
    pub fn with_capability(mut self, cap: &'a str) -> Self {
        self.capabilities.push(cap);
        self
    }
    pub fn with_arg(mut self, arg: Type<'a>) -> Self {
        self.args.push(arg);
        self
    }
}

fn format_arg_set<'a>(
    f: &mut std::fmt::Formatter<'_>,
    joiner: &str,
    args: &[Type<'a>],
) -> std::fmt::Result {
    let mut first = true;
    for arg in args {
        if first {
            first = false;
        } else {
            write!(f, "{}", joiner)?;
        }
        write!(f, "{}", arg)?;
    }
    Ok(())
}

impl<'a> std::fmt::Display for Type<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cap in &self.capabilities {
            write!(f, "{} ", cap)?;
        }
        if self.name == "ibis.Labelled" && self.args.len() > 1 {
            write!(f, "{}: ", self.args[0])?;
            let is_product = self.args[1].name == "ibis.ProductType";
            if is_product {
                write!(f, "(")?;
            }
            format_arg_set(f, ", ", &self.args[1..])?;
            if is_product {
                write!(f, ")")?;
            }
            Ok(())
        } else if self.name == "ibis.ProductType" && self.args.len() > 0 {
            format_arg_set(f, " ", &self.args)
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
