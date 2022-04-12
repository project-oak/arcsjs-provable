// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

extern crate nom;
use nom::{
    bytes::complete::{tag as simple_tag, take_while1},
    character::complete::{space0, space1},
    combinator::{cut, opt},
    multi::{separated_list0, separated_list1},
    sequence::tuple,
    Finish, IResult,
};

use crate::type_struct::Type;

fn is_name_char(c: char) -> bool {
    !matches!(
        c,
        '(' | ')' | '{' | '}' | ',' | ':' | ' ' | '\n' | '\r' | '\t'
    )
}

fn tag(exp: &'static str) -> impl Fn(&str) -> IResult<&str, &str> {
    move |input| {
        let (input, (_, s)) = tuple((space0, simple_tag(exp)))(input)?;
        Ok((input, s))
    }
}

fn is_lower_char(c: char) -> bool {
    matches!(c, 'a'..='z' | '_')
}

fn name(input: &str) -> IResult<&str, &str> {
    let (input, _) = space0(input)?;
    take_while1(is_name_char)(input)
}

fn capability(input: &str) -> IResult<&str, &str> {
    let (input, (_, cap, _)) = tuple((space0, take_while1(is_lower_char), space1))(input)?;
    Ok((input, cap))
}

fn label(input: &str) -> IResult<&str, &str> {
    let (input, (name, _)) = tuple((name, tag(":")))(input)?;
    Ok((input, name))
}

fn type_args(input: &str) -> IResult<&str, Vec<Type>> {
    let (input, (_, args, _)) = tuple((
        tag("("),
        cut(separated_list0(tag(","), type_parser)),
        tag(")"),
    ))(input)?;
    Ok((input, args))
}

fn parenthesized(input: &str) -> IResult<&str, Type> {
    let (input, (_, ty, _)) = tuple((tag("("), cut(type_parser), tag(")")))(input)?;
    Ok((input, ty))
}

fn simple_structure(input: &str) -> IResult<&str, Type> {
    let (input, (name, args)) = tuple((name, opt(type_args)))(input)?;
    Ok((input, Type::with_args(name, args.unwrap_or_default())))
}

fn labelled_type(input: &str) -> IResult<&str, Type> {
    let (input, (label, ty)) = tuple((label, cut(type_parser)))(input)?;
    Ok((
        input,
        Type::new("ibis.Labelled")
            .with_arg(Type::new(label))
            .with_arg(ty),
    ))
}

fn product_type(input: &str) -> IResult<&str, Type> {
    let (input, (_, mut types, _)) = tuple((
        tag("{"),
        cut(separated_list1(tag(","), type_parser)),
        tag("}"),
    ))(input)?;
    let mut types: Vec<Type> = types.drain(0..).rev().collect();
    let mut ty = types
        .pop()
        .expect("A product type requires at least one type");
    for new_ty in types {
        ty = Type::with_args("ibis.ProductType", vec![ty, new_ty]);
    }
    Ok((input, ty))
}

fn structure_with_capability(input: &str) -> IResult<&str, Type> {
    let (input, (cap, ty)) = tuple((capability, cut(type_parser)))(input)?;
    Ok((input, ty.with_capability(cap)))
}

fn type_parser(input: &str) -> IResult<&str, Type> {
    let (input, res) = parenthesized(input)
        .or_else(|_| product_type(input))
        .or_else(|_| labelled_type(input))
        .or_else(|_| structure_with_capability(input))
        .or_else(|_| simple_structure(input))?;
    let (input, _) = space0(input)?; // drop any following whitespace.
    Ok((input, res))
}

pub fn read_type(og_input: &str) -> Type {
    // TODO: return errors instead of panics
    let (input, ty) = type_parser(og_input)
        .finish()
        .expect("Could not parse type");
    if !input.is_empty() {
        todo!(
            "Did not reach end of input. Read {:?}. Left over '{}' from '{}'",
            ty,
            input,
            og_input
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
        parse_and_round_trip("Type", Type::new("Type"));
    }

    #[test]
    fn read_a_type_with_a_single_capabilities() {
        parse_and_round_trip("read Type", Type::new("Type").with_capability("read"));
    }

    #[test]
    fn read_a_type_with_multiple_capabilities() {
        parse_and_round_trip(
            "read write Type",
            Type::new("Type")
                .with_capability("write")
                .with_capability("read"),
        );
    }

    #[test]
    fn read_a_product_type_using_syntactic_sugar() {
        let name_string = read_type("{name: String}");
        let age_number = read_type("{age: Number}");
        parse_and_round_trip(
            "{name: String, age: Number}",
            Type::new("ibis.ProductType")
                .with_arg(name_string)
                .with_arg(age_number),
        );
    }

    #[test]
    fn read_nested_type() {
        let json = read_type("JSON");
        let age_number = read_type("{age: Number}");
        parse_and_round_trip(
            "name: {JSON, age: Number}",
            Type::new("ibis.Labelled")
                .with_arg(Type::new("name"))
                .with_arg(
                    Type::new("ibis.ProductType")
                        .with_arg(json)
                        .with_arg(age_number),
                ),
        );
    }

    #[test]
    fn read_a_type_with_arguments() {
        parse_and_round_trip(
            "Type(a, b)",
            Type::new("Type")
                .with_arg(Type::new("a"))
                .with_arg(Type::new("b")),
        );
    }

    #[test]
    fn read_a_type_with_nested_arguments() {
        parse_and_round_trip(
            "Type(a(c), b)",
            Type::new("Type")
                .with_arg(Type::new("a").with_arg(Type::new("c")))
                .with_arg(Type::new("b")),
        );
    }

    #[test]
    fn read_type_with_label() {
        parse_and_round_trip(
            "name: Type",
            Type::new("ibis.Labelled")
                .with_arg(Type::new("name"))
                .with_arg(Type::new("Type")),
        );
    }

    // TODO: tests for error messages
}
