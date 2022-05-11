// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

mod utils;
use pretty_assertions::assert_eq;
use utils::all_edges;

#[test]
fn a_union_is_a_subtype_of_its_arguments() {
    let solutions = all_edges(
        r#"
{
  "flags": {
    "planning": true
  },
  "capabilities": [
    ["any", "any"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_a", "a", "any ibis.UnionType(Man, Dog)"],
        ["p_b", "b", "any Dog"],
        ["p_c", "c", "any Man"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec!["b -> a, c -> a".to_string()];
    assert_eq!(solutions, expected);
}

#[test]
fn a_union_is_not_a_subtype_of_its_arguments_with_unshared_super_types() {
    let solutions = all_edges(
        r#"
{
  "flags": {
    "planning": true
  },
  "capabilities": [
    ["any", "any"]
  ],
  "subtypes": [
    ["TallMan", "Man"],
    ["ShortMan", "Man"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_a", "a", "any ibis.UnionType(TallMan, ShortMan, Dog)"],
        ["p_b", "b", "any Man"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec!["b -> a".to_string()];
    assert_eq!(solutions, expected);
}

#[test]
fn a_union_is_a_subtype_of_its_arguments_shared_super_types() {
    let solutions = all_edges(
        r#"
{
  "flags": {
    "planning": true
  },
  "capabilities": [
    ["any", "any"]
  ],
  "subtypes": [
    ["TallMan", "Man"],
    ["ShortMan", "Man"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_a", "a", "any ibis.UnionType(TallMan, ShortMan)"],
        ["p_b", "b", "any Man"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec!["a -> b".to_string()];
    assert_eq!(solutions, expected);
}

#[test]
fn a_type_is_equal_to_the_union_of_it_and_its_super_types() {
    let solutions = all_edges(
        r#"
{
  "flags": {
    "planning": true
  },
  "capabilities": [
    ["any", "any"]
  ],
  "subtypes": [
    ["Man", "Mortal"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_a", "a", "any ibis.UnionType(Man, Mortal)"],
        ["p_b", "b", "any Man"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec!["b -> a".to_string()];
    assert_eq!(solutions, expected);
}

#[test]
fn union_of_unions() {
    let solutions = all_edges(
        r#"
{
  "flags": {
    "planning": true
  },
  "capabilities": [
    ["any", "any"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_abc", "abc", "any ibis.UnionType(A, ibis.UnionType(B, C))"],
        ["p_acb", "acb", "any ibis.UnionType(ibis.UnionType(A, C), B)"],
        ["p_a", "a", "any A"],
        ["p_b", "b", "any B"],
        ["p_c", "c", "any C"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec![
        "a -> abc, a -> acb, abc -> acb, acb -> abc, b -> abc, b -> acb, c -> abc, c -> acb"
            .to_string(),
    ];
    assert_eq!(solutions, expected);
}

#[test]
fn union_of_unions_inlined() {
    let solutions = all_edges(
        r#"
{
  "flags": {
    "planning": true
  },
  "capabilities": [
    ["any", "any"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_abc", "abc", "any ibis.UnionType(A, B, C)"],
        ["p_acb", "acb", "any ibis.UnionType(A, C, B)"],
        ["p_a", "a", "any A"],
        ["p_b", "b", "any B"],
        ["p_c", "c", "any C"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec![
        "a -> abc, a -> acb, abc -> acb, acb -> abc, b -> abc, b -> acb, c -> abc, c -> acb"
            .to_string(),
    ];
    assert_eq!(solutions, expected);
}
