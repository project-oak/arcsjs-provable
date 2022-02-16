// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

extern crate nom;
use nom::{
    bytes::complete::{tag, take_while},
    combinator::opt,
    multi::separated_list0,
    Finish, IResult,
};

use crate::type_struct::Type;

fn is_name_char(c: char) -> bool {
    c != '(' && c != ')' && c != ','
}

fn name(input: &str) -> IResult<&str, &str> {
    take_while(is_name_char)(input)
}

fn type_args(input: &str) -> IResult<&str, Vec<Type>> {
    let (input, _) = tag("(")(input)?;
    let (input, args) = separated_list0(tag(", "), type_parser)(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, args))
}

fn type_parser(input: &str) -> IResult<&str, Type> {
    let (input, name) = name(input)?;
    let (input, args) = opt(type_args)(input)?;
    let args = args.unwrap_or_default();

    Ok((input, Type { name, args }))
}

pub fn read_type(input: &str) -> Type {
    // TODO: return errors instead of panics
    let (input, ty) = type_parser(input).finish().expect("Could not parse type");
    if !input.is_empty() {
        todo!(
            "Did not reach end of input. Read {:?}. Left over {}",
            ty,
            input
        );
    }
    ty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_a_simple_type_name() {
        assert_eq!(
            read_type("Type"),
            Type {
                name: "Type",
                args: vec![]
            }
        );
    }

    #[test]
    fn read_a_type_with_arguments() {
        assert_eq!(
            read_type("Type(a, b)"),
            Type {
                name: "Type",
                args: vec![
                    Type {
                        name: "a",
                        args: vec![]
                    },
                    Type {
                        name: "b",
                        args: vec![]
                    },
                ]
            }
        );
    }

    #[test]
    fn read_a_type_with_nested_arguments() {
        assert_eq!(
            read_type("Type(a(c), b)"),
            Type {
                name: "Type",
                args: vec![
                    Type {
                        name: "a",
                        args: vec![Type {
                            name: "c",
                            args: vec![]
                        }]
                    },
                    Type {
                        name: "b",
                        args: vec![]
                    },
                ]
            }
        );
    }

    // TODO: tests for error messages
}
