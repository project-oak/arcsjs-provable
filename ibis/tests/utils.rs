// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use ibis::{get_solutions, Ibis, Recipe};

fn map_solutions_with_edge_loss<U>(
    data: &str,
    loss: Option<usize>,
    on_result: &dyn Fn(&Recipe) -> U,
) -> Vec<U> {
    let recipes: Ibis = get_solutions(data, loss);
    recipes
        .recipes
        .iter()
        .map(|recipe| on_result(recipe))
        .collect()
}

fn solutions_with_edge_loss(data: &str, loss: Option<usize>) -> Vec<String> {
    let mut edges = map_solutions_with_edge_loss(data, loss, &|recipe: &Recipe| {
        let mut in_nodes: Vec<String> = (&recipe.edges)
            .iter()
            .map(|(from, to)| format!("{} -> {}", from, to))
            .collect();
        in_nodes.sort();
        in_nodes.join(", ")
    });
    edges.sort();
    edges
}

#[allow(dead_code)]
pub fn map_all_solutions(data: &str, on_result: &dyn Fn(&Recipe) -> String) -> Vec<String> {
    let mut solutions = map_solutions_with_edge_loss(data, None, on_result);
    solutions.sort();
    solutions
}

#[allow(dead_code)]
pub fn all_edges(data: &str) -> Vec<String> {
    solutions_with_edge_loss(data, Some(0))
}

#[allow(dead_code)]
pub fn all_solutions(data: &str) -> Vec<String> {
    solutions_with_edge_loss(data, None)
}
