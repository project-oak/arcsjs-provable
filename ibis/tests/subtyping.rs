// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

mod utils;
use pretty_assertions::assert_eq;
use utils::all_edges;

#[test]
fn static_subtyping_socretes_is_mortal() {
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
    ["plato", "man"],
    ["socretes", "man"],
    ["man", "mortal"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_a", "socretes", "any", "socretes"],
        ["p_b", "plato", "any", "plato"],
        ["p_c", "man", "any", "man"],
        ["p_out", "mortal", "any", "mortal"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec![
        "man -> mortal, plato -> man, plato -> mortal, socretes -> man, socretes -> mortal"
            .to_string(),
    ];
    assert_eq!(solutions, expected);
}
