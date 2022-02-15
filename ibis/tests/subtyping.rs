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
  "subtypes": [
    ["plato", "man"],
    ["socretes", "man"],
    ["man", "mortal"]
  ],
  "recipies": [
    {
      "nodes": [
        ["p_a", "socretes", "socretes"],
        ["p_b", "plato", "plato"],
        ["p_c", "man", "man"],
        ["p_out", "mortal", "mortal"]
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
