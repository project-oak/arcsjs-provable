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

use crate::type_struct::{Type, Structure};

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

fn type_args(input: &str) -> IResult<&str, Vec<Structure>> {
    let (input, (_, args, _)) =
        tuple((tag("("), separated_list0(tag(", "), labelled_simple_type_structure), tag(")")))(input)?;
    Ok((input, args))
}

fn simple_type_structure(input: &str) -> IResult<&str, Structure> {
    if let Ok((input, (_, ty, _))) = tuple((tag("("), labelled_simple_type_structure, tag(")")))(input) {
        return Ok((input, ty));
    }
    let (input, (mut name, args)) = tuple((name, opt(type_args)))(input)?;
    if name == "*" {
        name = "ibis.UniversalType";
    }
    Ok((input, Structure::with_args(name, args.unwrap_or_default())))
}

fn labelled_simple_type_structure(input: &str) -> IResult<&str, Structure> {
    let (input, (label, mut structure)) = tuple((opt(label), simple_type_structure))(input)?;
    if let Some(label) = label {
        structure = Structure::new("ibis.Labelled")
            .with_arg(Structure::new(label))
            .with_arg(structure);
    }
    Ok((input, structure))
}

fn type_parser(input: &str) -> IResult<&str, Type> {
    let (input, (capabilities, _, ty)) = tuple((separated_list1(space1, name), space0, labelled_simple_type_structure))(input)?;
    Ok((input, Type::from_structure(ty, capabilities)))
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
            Type::new("Type")
        );
    }

    #[test]
    fn read_a_type_with_a_single_capabilities() {
        parse_and_round_trip(
            "read Type",
            Type::new("Type")
                .with_capability("read")
        );
    }

    #[test]
    fn read_a_type_with_multiple_capabilities() {
        parse_and_round_trip(
            "read write Type",
            Type::new("Type")
                .with_capability("read")
                .with_capability("write")
        );
    }

    #[test]
    fn read_a_product_type_using_syntactic_sugar() {
        let name_string = read_type("name: String").structure;
        let age_number = read_type("age: Number").structure;
        parse_and_round_trip(
            "name: String age: Number",
            Type::new("ibis.ProductType")
                .with_arg(name_string)
                .with_arg(age_number)
        );
    }

    #[test]
    fn read_nested_type() {
        let json = read_type("JSON").structure;
        let age_number = read_type("age: Number").structure;
        parse_and_round_trip(
            "name: (JSON age: Number)",
            Type::new("ibis.Labelled")
                .with_arg(Structure::new("name"))
                .with_arg(
                    Structure::new("ibis.ProductType")
                        .with_arg(json)
                        .with_arg(age_number)
                )
        );
    }

    #[test]
    fn read_a_type_with_arguments() {
        parse_and_round_trip(
            "Type(a, b)",
            Type::new("Type")
                .with_arg(Structure::new("a"))
                .with_arg(Structure::new("b"))
        );
    }

    #[test]
    fn read_a_type_with_nested_arguments() {
        parse_and_round_trip(
            "Type(a(c), b)",
            Type::new("Type")
                .with_arg(
                    Structure::new("a")
                        .with_arg(Structure::new("c")),
                )
                .with_arg(Structure::new("b"))
        );
    }

    #[test]
    fn read_type_with_label() {
        parse_and_round_trip(
            "name: Type",
            Type::new("ibis.Labelled")
                .with_arg(Structure::new("name"))
                .with_arg(Structure::new("Type"))
        );
    }

    // TODO: tests for error messages
}
