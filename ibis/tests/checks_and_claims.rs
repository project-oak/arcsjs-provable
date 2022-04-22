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
  "flags": {
    "planning": true
  },
  "capabilities": [
    ["write", "read"]
  ],
  "subtypes": [
    ["any", "read"],
    ["any", "write"],
    ["Int", "Number"],
    ["Int", "Serializable"],
    ["String", "Serializable"],
    ["Number", "Or(Number, String)"],
    ["String", "Or(Number, String)"]
  ],
  "less_private_than": [
    ["public", "private"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_a", "a", "write Int +private"],
        ["p_b", "b", "any Number -private"],
        ["p_c", "c", "any String"],
        ["p_de", "d", "write Serializable +public"],
        ["p_de", "e", "read Or(Number, String) +public"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec![
        "",
        "a -> b",
        "a -> b, b -> e",
        "a -> b, b -> e, c -> e",
        "a -> b, c -> e",
        "b -> e",
        "b -> e, c -> e",
        "c -> e",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    assert_eq!(solutions, expected);
}
