// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

mod utils;
use pretty_assertions::assert_eq;
use utils::all_solutions;

#[test]
fn create_tagged_type_checked_graphs() {
    let solutions = all_solutions(
        r#"
{
  "capabilities": [
    ["any", "any"]
  ],
  "subtypes": [
    ["Int", "Number"],
    ["Int", "Serializable"],
    ["String", "Serializable"],
    ["Number", "Or(Number, String)"],
    ["String", "Or(Number, String)"]
  ],
  "less_private_than": [
    ["public", "private"]
  ],
  "recipies": [
    {
      "trusted_to_remove_tag": [
        ["b", "private"]
      ],
      "claims": [
        ["a", "private"]
      ],
      "checks": [
        ["e", "public"],
        ["d", "public"]
      ],
      "nodes": [
        ["p_a", "a", "any", "Int"],
        ["p_b", "b", "any", "Number"],
        ["p_c", "c", "any", "String"],
        ["p_de", "d", "any", "Serializable"],
        ["p_de", "e", "any", "Or(Number, String)"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec![
        "",
        "a -> b",
        "a -> b, b -> e",
        "a -> b, b -> e, c -> d",
        "a -> b, b -> e, c -> d, c -> e",
        "a -> b, b -> e, c -> e",
        "a -> b, c -> d",
        "a -> b, c -> d, c -> e",
        "a -> b, c -> e",
        "b -> e",
        "b -> e, c -> d",
        "b -> e, c -> d, c -> e",
        "b -> e, c -> e",
        "c -> d",
        "c -> d, c -> e",
        "c -> e",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    assert_eq!(solutions, expected);
}
