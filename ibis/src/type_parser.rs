// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

extern crate nom;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::space0,
    combinator::opt,
    multi::separated_list0,
    sequence::tuple,
    Finish, IResult,
};

use crate::type_struct::Type;

fn is_name_char(c: char) -> bool {
    c != '(' && c != ')' && c != ',' && c != ':'
}

fn name(input: &str) -> IResult<&str, &str> {
    take_while(is_name_char)(input)
}

fn label(input: &str) -> IResult<&str, &str> {
    let (input, (name, _, _)) = tuple((name, tag(":"), space0))(input)?;
    Ok((input, name))
}

fn type_args(input: &str) -> IResult<&str, Vec<Type>> {
    let (input, (_, args, _)) =
        tuple((tag("("), separated_list0(tag(", "), type_parser), tag(")")))(input)?;
    Ok((input, args))
}

fn type_structure(input: &str) -> IResult<&str, Type> {
    let (input, (mut name, args)) = tuple((name, opt(type_args)))(input)?;
    if name == "*" {
        name = "ibis.UniversalType";
    }
    Ok((input, Type::new(name, args.unwrap_or_default())))
}

fn type_parser(input: &str) -> IResult<&str, Type> {
    let (input, (label, mut structure)) = tuple((opt(label), type_structure))(input)?;
    if let Some(label) = label {
        structure = Type::new("ibis.Labelled", vec![Type::named(label), structure]);
    }
    Ok((input, structure))
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

    #[test]
    fn read_type_with_label() {
        assert_eq!(
            read_type("name: Type"),
            Type {
                name: "ibis.Labelled",
                args: vec![
                    Type {
                        name: "name",
                        args: vec![],
                    },
                    Type {
                        name: "Type",
                        args: vec![]
                    },
                ]
            }
        );
    }

    // TODO: tests for error messages
}
