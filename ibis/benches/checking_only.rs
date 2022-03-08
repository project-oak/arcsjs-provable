// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ibis::best_solutions_to_json;

fn solve_demo(data: &str) {
    let _result = best_solutions_to_json(&data);
    // TODO: use the result to ensure it is correct
}

fn criterion_benchmark_solve_demo(c: &mut Criterion) {
    let data = r#"
{
  "flags": {
    "planning": false
  },
  "capabilities": [
    ["write", "read"],
    ["any", "read"],
    ["write", "any"]
  ],
  "subtypes": [
    ["Int", "Number"],
    ["Int", "Serializable"],
    ["String", "Serializable"]
  ],
  "less_private_than": [
    ["public", "private"]
  ],
  "recipes": [
    {
      "nodes": [
        ["p_a", "a", "write", "Int"],
        ["p_b", "b", "any", "Number"],
        ["p_c", "c", "write", "String"],
        ["p_de", "d", "read", "Serializable"],
        ["p_de", "e", "read", "ibis.UnionType(Number, String)"],
        ["p_f", "f", "write", "ibis.ProductType(name: String, age: Int)"],
        ["p_g", "g", "read", "name: *"],
        ["p_h", "h", "read", "ibis.ProductType(name: String, age: Int)"],
        ["p_i", "i", "read", "name: String"],
        ["p_j", "j", "read", "age: Int"]
      ],
      "claims": [
        ["a", "private"]
      ],
      "checks": [
        ["e", "public"]
      ],
      "trusted_to_remove_tag": [
        ["b", "private"]
      ],
      "edges": [
        ["a", "b"],
        ["b", "e"],
        ["c", "d"],
        ["c", "e"],
        ["f", "b"],
        ["f", "d"],
        ["f", "e"],
        ["f", "g"],
        ["f", "h"],
        ["f", "i"],
        ["f", "j"]
      ]
    }
  ]
}
"#;
    c.bench_function("checking_only", |b| b.iter(|| solve_demo(black_box(data))));
}

criterion_group!(benches, criterion_benchmark_solve_demo);
criterion_main!(benches);