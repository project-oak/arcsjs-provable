// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

mod utils;
use pretty_assertions::assert_eq;
use utils::all_edges;

#[test]
fn precomputed_subtypes() {
    let solutions = all_edges(
        r#"
{
  "capabilities": [
    ["any", "any"]
  ],
  "subtypes": [
    ["Man", "Mortal"],
    ["List(Man)", "List(Mortal)"],
    ["List(Man)", "Iterable(Man)"],
    ["List(Man)", "Iterable(Mortal)"],
    ["List(Mortal)", "Iterable(Mortal)"],
    ["Iterable(Man)", "Iterable(Mortal)"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_a", "a", "any", "List(Man)"],
        ["p_b", "b", "any", "List(Mortal)"],
        ["p_c", "c", "any", "Iterable(Man)"],
        ["p_d", "d", "any", "Iterable(Mortal)"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec!["a -> b, a -> c, a -> d, b -> d, c -> d".to_string()];
    assert_eq!(solutions, expected);
}

#[test]
fn generics_are_not_necessarily_abstractable() {
    // i.e. List(a) is not necessarily able to be used as any List.
    // That has to be decided for the specific type.
    let solutions = all_edges(
        r#"
{
  "capabilities": [
    ["any", "any"]
  ],
  "subtypes": [
    ["Man", "Mortal"],
    ["List", "Iterable"],
    ["Iterable", "ibis.GenericType"],
    ["Iterable", "ibis.InductiveType"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_a", "a", "any", "List(Man)"],
        ["p_b", "b", "any", "List"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec!["".to_string()];
    assert_eq!(solutions, expected);
}

#[test]
fn dynamic_subtypes() {
    let solutions = all_edges(
        r#"
{
  "capabilities": [
    ["any", "any"]
  ],
  "subtypes": [
    ["Man", "Mortal"],
    ["List", "Iterable"],
    ["Iterable", "ibis.GenericType"],
    ["Iterable", "ibis.InductiveType"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_a", "a", "any", "List(Man)"],
        ["p_b", "b", "any", "List(Mortal)"],
        ["p_c", "c", "any", "Iterable(Man)"],
        ["p_d", "d", "any", "Iterable(Mortal)"],
        ["p_e", "e", "any", "List(ibis.UniversalType)"],
        ["p_f", "f", "any", "List"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> =
        vec!["a -> b, a -> c, a -> d, a -> e, b -> d, b -> e, c -> d".to_string()];
    assert_eq!(solutions, expected);
}

#[test]
fn all_subtype_the_universal_type() {
    let solutions = all_edges(
        r#"
{
  "capabilities": [
    ["any", "any"]
  ],
  "subtypes": [
    ["Man", "Mortal"],
    ["List", "Iterable"],
    ["Iterable", "ibis.GenericType"],
    ["Iterable", "ibis.InductiveType"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_a", "a", "any", "List(Man)"],
        ["p_b", "b", "any", "List(ibis.UniversalType)"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec!["a -> b".to_string()];
    assert_eq!(solutions, expected);
}
