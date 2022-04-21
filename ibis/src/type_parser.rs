// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

extern crate nom;
use crate::type_struct::*;
use nom::{
    branch::alt,
    bytes::complete::{tag as simple_tag, take_while1},
    character::complete::{space0, space1},
    combinator::{cut, opt},
    multi::{many0, separated_list0, separated_list1},
    sequence::tuple,
    Finish, IResult,
};

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

enum DataTag<'a> {
    Add(&'a str),
    Remove(&'a str)
}

fn data_tag(input: &str) -> IResult<&str, DataTag> {
    let (input, (sign, data_tag)) = tuple((alt((tag("+"), tag("-"))), take_while1(is_lower_char)))(input)?;
    Ok((input,
        match sign {
        "+" => DataTag::Add(data_tag),
        "-" => DataTag::Remove(data_tag),
        _ => todo!(),
        }
    ))
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
    Ok((input, Type::new(name).with_args(args.unwrap_or_default())))
}

fn labelled_type(input: &str) -> IResult<&str, Type> {
    let (input, (label, ty)) = tuple((label, cut(type_parser)))(input)?;
    Ok((
        input,
        Type::new(LABELLED).with_arg(Type::new(label)).with_arg(ty),
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
        ty = Type::new(PRODUCT).with_arg(ty).with_arg(new_ty);
    }
    Ok((input, ty))
}

fn structure_with_capability(input: &str) -> IResult<&str, Type> {
    let (input, (cap, ty)) = tuple((capability, cut(type_parser)))(input)?;
    Ok((input, ty.with_capability(cap)))
}

fn type_parser(input: &str) -> IResult<&str, Type> {
    let (input, mut res) = parenthesized(input)
        .or_else(|_| product_type(input))
        .or_else(|_| labelled_type(input))
        .or_else(|_| structure_with_capability(input))
        .or_else(|_| simple_structure(input))?;
    let (input, tags) = many0(data_tag)(input)?;
    for tag in tags {
        let (op, tag) = match tag {
            DataTag::Add(tag) => (ADD_TAG, tag),
            DataTag::Remove(tag) => (REMOVE_TAG, tag),
        };
        res = Type::new(op).with_arg(res).with_arg(Type::new(tag));
    }
    let (input, _) = space0(input)?; // drop any following whitespace.
    Ok((input, res))
}

pub fn read_type_uncached(og_input: &str) -> Type {
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

    use read_type_uncached as read_type;

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
            Type::new(PRODUCT)
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
            Type::new(LABELLED)
                .with_arg(Type::new("name"))
                .with_arg(Type::new(PRODUCT).with_arg(json).with_arg(age_number)),
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
            Type::new(LABELLED)
                .with_arg(Type::new("name"))
                .with_arg(Type::new("Type")),
        );
    }

    #[test]
    fn read_type_with_a_tag() {
        parse_and_round_trip(
            "String +name",
            Type::new(ADD_TAG)
                .with_arg(Type::new("String"))
                .with_arg(Type::new("name")),
        );
    }

    #[test]
    fn read_type_with_tags() {
        parse_and_round_trip(
            "String +name +fullname",
            Type::new(ADD_TAG)
                .with_arg(
                    Type::new(ADD_TAG)
                        .with_arg(Type::new("String"))
                        .with_arg(Type::new("name")),
                )
                .with_arg(Type::new("fullname")),
        );
    }

    #[test]
    fn read_a_product_type_with_field_tags() {
        parse_and_round_trip(
            "name: String +fullname",
            Type::new(LABELLED)
                .with_arg(Type::new("name"))
                .with_arg(
                    Type::new(ADD_TAG)
                        .with_arg(Type::new("String"))
                        .with_arg(Type::new("fullname")),
                ),
        );
    }

    // TODO: tests for error messages

    #[test]
    fn read_type_with_a_remove_tag() {
        parse_and_round_trip(
            "String -name",
            Type::new(REMOVE_TAG)
                .with_arg(Type::new("String"))
                .with_arg(Type::new("name")),
        );
    }

    #[test]
    fn read_type_with_remove_tags() {
        parse_and_round_trip(
            "String -name -fullname",
            Type::new(REMOVE_TAG)
                .with_arg(
                    Type::new(REMOVE_TAG)
                        .with_arg(Type::new("String"))
                        .with_arg(Type::new("name")),
                )
                .with_arg(Type::new("fullname")),
        );
    }

    #[test]
    fn read_a_product_type_with_field_remove_tags() {
        parse_and_round_trip(
            "name: String -fullname",
            Type::new(LABELLED)
                .with_arg(Type::new("name"))
                .with_arg(
                    Type::new(REMOVE_TAG)
                        .with_arg(Type::new("String"))
                        .with_arg(Type::new("fullname")),
                ),
        );
    }

    #[test]
    fn read_a_product_type_with_field_add_and_remove_tags() {
        parse_and_round_trip(
            "name: String +fullname -private",
            Type::new(LABELLED)
                .with_arg(Type::new("name"))
                .with_arg(
                    Type::new(REMOVE_TAG)
                        .with_arg(
                            Type::new(ADD_TAG)
                                .with_arg(Type::new("String"))
                                .with_arg(Type::new("fullname"))
                                )
                        .with_arg(Type::new("private")),
                ),
        );
    }

    // TODO: tests for error messages
}
