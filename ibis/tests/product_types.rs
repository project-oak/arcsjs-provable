// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

mod utils;
use pretty_assertions::assert_eq;
use utils::all_edges;

#[test]
fn a_product_is_a_subtype_of_its_arguments() {
    let solutions = all_edges(
        r#"
{
  "capabilities": [
    ["any", "any"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_a", "a", "any", "ibis.ProductType(Man, Mortal)"],
        ["p_b", "b", "any", "Mortal"],
        ["p_c", "c", "any", "Man"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec!["a -> b, a -> c".to_string()];
    assert_eq!(solutions, expected);
}

#[test]
fn a_type_is_a_subtype_of_products_of_its_super_types() {
    let solutions = all_edges(
        r#"
{
  "capabilities": [
    ["any", "any"]
  ],
  "subtypes": [
    ["Man", "Mortal"],
    ["Man", "Human"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_a", "a", "any", "ibis.ProductType(Human, Mortal)"],
        ["p_b", "b", "any", "Man"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec!["b -> a".to_string()];
    assert_eq!(solutions, expected);
}

#[test]
fn a_type_is_equal_to_the_product_of_it_and_its_super_types() {
    let solutions = all_edges(
        r#"
{
  "capabilities": [
    ["any", "any"]
  ],
  "subtypes": [
    ["Man", "Mortal"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_a", "a", "any", "ibis.ProductType(Man, Mortal)"],
        ["p_b", "b", "any", "Man"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec!["a -> b, b -> a".to_string()];
    assert_eq!(solutions, expected);
}

#[test]
fn product_of_products() {
    let solutions = all_edges(
        r#"
{
  "capabilities": [
    ["any", "any"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_abc", "abc", "any", "ibis.ProductType(A, ibis.ProductType(B, C))"],
        ["p_acb", "acb", "any", "ibis.ProductType(ibis.ProductType(A, C), B)"],
        ["p_a", "a", "any", "A"],
        ["p_b", "b", "any", "B"],
        ["p_c", "c", "any", "C"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec![
        "abc -> a, abc -> acb, abc -> b, abc -> c, acb -> a, acb -> abc, acb -> b, acb -> c"
            .to_string(),
    ];
    assert_eq!(solutions, expected);
}
