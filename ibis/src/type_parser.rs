// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
extern crate nom;
use crate::type_struct::*;
use nom::{
    bytes::complete::{tag as simple_tag, take_while1},
    character::complete::{space0, space1},
    combinator::{cut, opt},
    multi::{separated_list0, separated_list1},
    sequence::tuple,
    Finish, IResult,
};
use std::sync::Arc;

fn is_name_char(c: char) -> bool {
    !matches!(
        c,
        '(' | ')' | '{' | '}' | ',' | ':' | ' ' | '\n' | '\r' | '\t'
    )
}

fn tag<'a>(exp: &'static str) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &'a str| {
        let (input, (_, s)) = tuple((space0, simple_tag(exp)))(input)?;
        Ok((input, s))
    }
}

fn is_lower_char(c: char) -> bool {
    matches!(c, 'a'..='z' | '_')
}

fn name<'a>() -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &'a str| {
        let (input, _) = space0(input)?;
        take_while1(|c| is_name_char(c))(input)
    }
}

fn label<'a>() -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &'a str| {
        let (input, (name, _)) = tuple((name(), tag(":")))(input)?;
        Ok((input, name))
    }
}

pub trait TypeParser {
    fn store_type(&mut self, _og_input: &str, get_ty: impl FnOnce(&mut Self) -> Arc<Type>) -> Arc<Type>;

    fn read_type(&mut self, input: &str) -> Arc<Type> {
        self.store_type(input, &|s: &mut Self| Arc::new(s.read_type_core(input)))
    }

    fn capability<'a>(&mut self, input: &'a str) -> IResult<&'a str, &'a str> {
        let (input, (_, cap, _)) =
            tuple((space0, take_while1(|c| is_lower_char(c)), space1))(input)?;
        Ok((input, cap))
    }

    fn type_args<'a>(&mut self, input: &'a str) -> IResult<&'a str, Vec<Type>> {
        let (input, (_, args, _)) = tuple((
            tag("("),
            cut(separated_list0(
                tag(","),
                |i| self.type_parser(i),
            )),
            tag(")"),
        ))(input)?;
        Ok((input, args))
    }

    fn parenthesized<'a>(&mut self, input: &'a str) -> IResult<&'a str, Type> {
        let (input, (_, ty, _)) = tuple((
            tag("("),
            cut(|i| self.type_parser(i)),
            tag(")"),
        ))(input)?;
        Ok((input, ty))
    }

    fn simple_structure<'a>(&mut self, input: &'a str) -> IResult<&'a str, Type> {
        let (input, (name, args)) = tuple((name(), opt(|i| self.type_args(i))))(input)?;
        Ok((input, Type::new(name).with_args(args.unwrap_or_default())))
    }

    fn labelled_type<'a>(&mut self, input: &'a str) -> IResult<&'a str, Type> {
        let (input, (label, ty)) = tuple((label(), cut(|i| self.type_parser(i))))(input)?;
        Ok((
            input,
            Type::new(LABELLED).with_arg(Type::new(label)).with_arg(ty),
        ))
    }

    fn product_type<'a>(&mut self, input: &'a str) -> IResult<&'a str, Type> {
        let (input, _) = tag("{")(input)?;
        let (input, mut types) = cut(separated_list1(tag(","), |i| self.type_parser(i)))(input)?;
        let (input, _) = tag("}")(input)?;
        let mut types: Vec<Type> = types.drain(0..).rev().collect();
        let mut ty = types
            .pop()
            .expect("A product type requires at least one type");
        for new_ty in types {
            ty = Type::new(PRODUCT).with_arg(ty).with_arg(new_ty);
        }
        Ok((input, ty))
    }

    fn structure_with_capability<'a>(&mut self, input: &'a str) -> IResult<&'a str, Type> {
        let (input, cap) = self.capability(input)?;
        let (input, ty) = cut(|i| self.type_parser(i))(input)?;
        Ok((input, ty.with_capability(cap)))
    }

    fn type_parser<'a>(&mut self, input: &'a str) -> IResult<&'a str, Type> {
        let (input, res) = self
            .parenthesized(input)
            .or_else(|_| self.product_type(input))
            .or_else(|_| self.labelled_type(input))
            .or_else(|_| self.structure_with_capability(input))
            .or_else(|_| self.simple_structure(input))?;
        let (input, _) = space0(input)?; // drop any following whitespace.
        Ok((input, res))
    }

    fn read_type_core(&mut self, og_input: &str) -> Type {
        // TODO: return errors instead of panics
        let (input, ty) = self
            .type_parser(og_input)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TP;

    impl TypeParser for TP {
        fn store_type(&mut self, _og_input: &str, get_ty: impl FnOnce(&mut Self) -> Arc<Type>) -> Arc<Type> {
            // This is lazy / lossy, re-parsing a type will make a new copy.
            get_ty(self)
        }
    }

    fn read_type(input: &str) -> Type {
        let mut tp = TP {};
        // This discards the 'arc'. Bad form
        (*tp.read_type(input)).clone()
    }

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

    // TODO: tests for error messages
}
