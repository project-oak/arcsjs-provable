// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use ibis::Ibis;
use pretty_assertions::assert_eq;

#[test]
fn create_combinations() {
    let mut runtime = Ibis::new();

    let data = r#"
{
  "subtypes": [
    ["Char_a", "Unit"],
    ["Char_b", "Unit"],
    ["Char_c", "Unit"]
  ],
  "recipies": [
    {
      "nodes": [
        ["p_a", "a", "Char_a"],
        ["p_b", "b", "Char_b"],
        ["p_c", "c", "Char_c"],
        ["p_out", "out", "Unit"]
      ]
    }
  ]
}"#;
    let recipies: Ibis = serde_json::from_str(data).expect("JSON Error?");

    runtime.add_recipies(recipies);

    let mut solutions: Vec<String> = runtime
        .extract_solutions()
        .recipies
        .iter()
        .map(|recipe| {
            let mut in_nodes: Vec<String> = (&recipe.edges)
                .iter()
                .map(|(from, _to)| from.name().clone())
                .collect();
            in_nodes.sort();
            in_nodes.join("")
        })
        .collect();
    let mut expected: Vec<String> = vec!["", "a", "b", "c", "ab", "bc", "ac", "abc"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    solutions.sort();
    expected.sort();
    assert_eq!(solutions, expected);
}

#[test]
fn create_edges() {
    let mut runtime = Ibis::new();

    let data = r#"
{
  "recipies": [
    {
      "nodes": [
        ["p_a", "a", "Char"],
        ["p_b", "b", "Char"]
      ]
    }
  ]
}"#;
    let recipies: Ibis = serde_json::from_str(data).expect("JSON Error?");

    runtime.add_recipies(recipies);

    let mut solutions: Vec<String> = runtime
        .extract_solutions()
        .recipies
        .iter()
        .map(|recipe| {
            let mut in_nodes: Vec<String> = (&recipe.edges)
                .iter()
                .map(|(from, to)| format!("{} -> {}", from, to))
                .collect();
            in_nodes.sort();
            in_nodes.join(", ")
        })
        .collect();
    let mut expected: Vec<String> = vec!["", "a -> b", "b -> a", "a -> b, b -> a"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    solutions.sort();
    expected.sort();
    assert_eq!(solutions, expected);
}

#[test]
fn create_typed_edges() {
    let mut runtime = Ibis::new();

    let data = r#"
{
  "recipies": [
    {
      "nodes": [
        ["p_a", "a", "Char"],
        ["p_b", "b", "Char"],
        ["p_c", "c", "Int"]
      ]
    }
  ]
}"#;
    let recipies: Ibis = serde_json::from_str(data).expect("JSON Error?");

    runtime.add_recipies(recipies);

    let mut solutions: Vec<String> = runtime
        .extract_solutions()
        .recipies
        .iter()
        .map(|recipe| {
            let mut in_nodes: Vec<String> = (&recipe.edges)
                .iter()
                .map(|(from, to)| format!("{} -> {}", from, to))
                .collect();
            in_nodes.sort();
            in_nodes.join(", ")
        })
        .collect();
    let mut expected: Vec<String> = vec!["", "a -> b", "b -> a", "a -> b, b -> a"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    solutions.sort();
    expected.sort();
    assert_eq!(solutions, expected);
}

#[test]
#[ignore] // This test (unoptimized) can take 2 seconds and so will (by default) only run on CI.
fn create_all_directed_graphs_with_4_nodes() {
    // Useful for performance estimations, not a proper bench mark.
    // Calculates the number of different graphs with 4 nodes
    // 4*3 = 12 possible directed edges (excluding self edges)
    // 2^12 = 4096 graphs
    // Note: Do not try with larger graphs, the |results| are O(2^(n*(n-1))).
    // e.g. for 5 nodes theres over a trillion results to calculate.
    let mut runtime = Ibis::new();

    let data = r#"
{
  "recipies": [
    {
      "nodes": [
        ["p_a", "a", "Char"],
        ["p_b", "b", "Char"],
        ["p_c", "c", "Char"],
        ["p_d", "d", "Char"]
      ]
    }
  ]
}"#;
    let recipies: Ibis = serde_json::from_str(data).expect("JSON Error?");
    runtime.add_recipies(recipies);
    let solutions = runtime.extract_solutions().recipies;
    assert_eq!(solutions.len(), 4096);
}
