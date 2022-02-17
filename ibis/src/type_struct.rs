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

impl <'a> Type<'a> {
    pub fn new(name: &'a str, args: Vec<Type<'a>>) -> Self {
        Self { name, args }
    }

    pub fn named(name: &'a str) -> Self {
        Self::new(name, vec![])
    }
}

impl<'a> std::fmt::Display for Type<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = write!(f, "{}", self.name)?;
        if self.args.is_empty() {
            Ok(res)
        } else {
            write!(f, "(")?;
            let mut first = true;
            for arg in &self.args {
                if first {
                    first = false;
                } else {
                    write!(f, ", ")?;
                }
                write!(f, "{}", arg)?;
            }
            write!(f, ")")
        }
    }
}
