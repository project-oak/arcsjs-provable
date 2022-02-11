// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use ibis::Ibis;
use pretty_assertions::assert_eq;

#[test]
fn create_tagged_type_checked_graphs() {
    let mut runtime = Ibis::new();

    let data = r#"
{
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
        ["p_a", "a", "Int"],
        ["p_b", "b", "Number"],
        ["p_c", "c", "String"],
        ["p_de", "d", "Serializable"],
        ["p_de", "e", "Or(Number, String)"]
      ]
    }
  ]
}"#;
    let recipies: Ibis = serde_json::from_str(data).expect("JSON Error?");

    runtime.add_recipies(recipies);

    let mut solutions: Vec<String> = runtime.extract_solutions().recipies.iter().map(|recipe| {
        let mut in_nodes: Vec<String> = (&recipe.edges).iter().map(|(from, to)| format!("{} -> {}", from, to)).collect();
        in_nodes.sort();
        in_nodes.join(", ")
    }).collect();
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
        "c -> e"
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    solutions.sort();
    assert_eq!(solutions, expected);
}
