// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

mod utils;
use pretty_assertions::assert_eq;
use utils::{all_solutions, map_all_solutions};

#[test]
fn create_combinations() {
    let solutions = map_all_solutions(
        r#"
{
  "flags": {
    "planning": true
  },
  "capabilities": [
    ["write", "read"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_a", "a", "write", "Unit"],
        ["p_b", "b", "write", "Unit"],
        ["p_c", "c", "write", "Unit"],
        ["p_out", "out", "read", "Unit"]
      ]
    }
  ]
}"#,
        &|recipe| {
            let mut in_nodes: Vec<String> = (&recipe.edges)
                .iter()
                .map(|(from, _to)| from.name())
                .collect();
            in_nodes.sort();
            in_nodes.join("")
        },
    );
    let expected: Vec<String> = vec!["", "a", "ab", "abc", "ac", "b", "bc", "c"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(solutions, expected);
}

#[test]
fn create_edges() {
    let solutions = all_solutions(
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
        ["p_a", "a", "any", "Char"],
        ["p_b", "b", "any", "Char"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec!["", "a -> b", "a -> b, b -> a", "b -> a"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(solutions, expected);
}

#[test]
fn create_typed_edges() {
    let solutions = all_solutions(
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
        ["p_a", "a", "any", "Char"],
        ["p_b", "b", "any", "Char"],
        ["p_c", "c", "any", "Int"]
      ]
    }
  ]
}"#,
    );
    let expected: Vec<String> = vec!["", "a -> b", "a -> b, b -> a", "b -> a"]
        .iter()
        .map(|s| s.to_string())
        .collect();
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
    let solutions = all_solutions(
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
        ["p_a", "a", "any", "Char"],
        ["p_b", "b", "any", "Char"],
        ["p_c", "c", "any", "Char"],
        ["p_d", "d", "any", "Char"]
      ]
    }
  ]
}"#,
    );
    assert_eq!(solutions.len(), 4096);
}
