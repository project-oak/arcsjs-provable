// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

extern crate nom;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{space0, space1},
    combinator::opt,
    multi::{separated_list0, separated_list1},
    sequence::tuple,
    Finish, IResult,
};

use crate::type_struct::Type;

fn is_name_char(c: char) -> bool {
    match c {
        '(' | ')' | ',' | ':' => false, // Symbols
        ' ' | '\n' | '\r' | '\t' => false, // Whitespace
        _ => true // Name char
    }
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

fn simple_type_structure(input: &str) -> IResult<&str, Type> {
    if let Ok((input, (_, ty, _))) = tuple((tag("("), type_parser, tag(")")))(input) {
        return Ok((input, ty));
    }
    let (input, (mut name, args)) = tuple((name, opt(type_args)))(input)?;
    if name == "*" {
        name = "ibis.UniversalType";
    }
    Ok((input, Type::new(name, args.unwrap_or_default())))
}

fn labelled_simple_type_structure(input: &str) -> IResult<&str, Type> {
    let (input, (label, mut structure)) = tuple((opt(label), simple_type_structure))(input)?;
    if let Some(label) = label {
        structure = Type::new("ibis.Labelled", vec![Type::named(label), structure]);
    }
    Ok((input, structure))
}

fn type_parser(input: &str) -> IResult<&str, Type> {
    let (input, mut types) = separated_list1(space1, labelled_simple_type_structure)(input)?;
    let mut ty = None;
    for new_ty in types.drain(0..).rev() {
        ty = Some(if let Some(ty) = ty {
            Type::new("ibis.ProductType", vec![new_ty, ty])
        } else {
            new_ty
        });
    }
    Ok((input, ty.expect("Should have a type")))
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

    fn parse_and_round_trip(s: &str, t: Type) {
        let ty = read_type(s);
        assert_eq!(ty, t);
        assert_eq!(format!("{}", ty), s);
    }

    #[test]
    fn read_a_simple_type_name() {
        parse_and_round_trip(
            "Type",
            Type::named("Type")
        );
    }

    #[test]
    fn read_a_type_with_a_single_capabilities() {
        parse_and_round_trip(
            "read Type",
            Type {
                name: "ibis.ProductType",
                args: vec![
                    Type::named("read"),
                    Type::named("Type")
                ]
            }
        );
    }

    #[test]
    fn read_a_type_with_multiple_capabilities() {
        parse_and_round_trip(
            "read write Type",
            Type {
                name: "ibis.ProductType",
                args: vec![
                    Type::named("read"),
                    Type {
                        name: "ibis.ProductType",
                        args: vec![
                            Type::named("write"),
                            Type::named("Type")
                        ]
                    }
                ]
            }
        );
    }

    #[test]
    fn read_a_product_type_using_syntactic_sugar() {
        let name_string = read_type("name: String");
        let age_number = read_type("age: Number");
        parse_and_round_trip(
            "name: String age: Number",
            Type {
                name: "ibis.ProductType",
                args: vec![
                    name_string,
                    age_number
                ]
            }
        );
    }

    #[test]
    fn read_nested_type() {
        let json = read_type("JSON");
        let age_number = read_type("age: Number");
        parse_and_round_trip(
            "name: (JSON age: Number)",
            Type {
                name: "ibis.Labelled",
                args: vec![
                    Type::named("name"),
                    Type {
                        name: "ibis.ProductType",
                        args: vec![
                            json,
                            age_number
                        ]
                    }
                ]
            }
        );
    }

    #[test]
    fn read_a_type_with_arguments() {
        parse_and_round_trip(
            "Type(a, b)",
            Type {
                name: "Type",
                args: vec![
                    Type::named("a"),
                    Type::named("b")
                ]
            }
        );
    }

    #[test]
    fn read_a_type_with_nested_arguments() {
        parse_and_round_trip(
            "Type(a(c), b)",
            Type {
                name: "Type",
                args: vec![
                    Type {
                        name: "a",
                        args: vec![Type::named("c")]
                    },
                    Type::named("b")
                ]
            }
        );
    }

    #[test]
    fn read_type_with_label() {
        parse_and_round_trip(
            "name: Type",
            Type {
                name: "ibis.Labelled",
                args: vec![
                    Type::named("name"),
                    Type::named("Type")
                ]
            }
        );
    }

    // TODO: tests for error messages
}
