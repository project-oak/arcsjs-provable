// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use ibis::Ibis;
use pretty_assertions::assert_eq;

#[test]
fn static_subtyping_socretes_is_mortal() {
    let mut runtime = Ibis::new();

    let data = r#"
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
}"#;
    let recipies: Ibis = serde_json::from_str(data).expect("JSON Error?");
    runtime.add_recipies(recipies);

    let solutions = runtime.extract_solutions_with_loss(Some(0));
    let solutions: Vec<String> = solutions.recipies.iter().map(|recipe| {
        let mut in_nodes: Vec<String> = (&recipe.edges).iter().map(|(from, to)| format!("{} is {}", &from, &to)).collect();
        in_nodes.sort();
        in_nodes.join(", ")
    }).collect();
    let expected: Vec<String> = vec!["man is mortal, plato is man, plato is mortal, socretes is man, socretes is mortal".to_string()];

    assert_eq!(
        solutions,
        expected
    );
}
