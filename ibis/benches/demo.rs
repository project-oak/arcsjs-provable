// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use criterion::{black_box, Criterion};
use ibis::best_solutions_to_json;

fn solve_demo(data: &str) {
    let _result = best_solutions_to_json(data);
    // TODO: use the result to ensure it is correct
}

pub fn criterion_benchmark_solve_demo(c: &mut Criterion) {
    let data = include_str!("../demo.json");
    c.bench_function("solve demo.json", |b| {
        b.iter(|| solve_demo(black_box(data)))
    });
}

pub fn criterion_benchmark_solve_chromium_demo(c: &mut Criterion) {
    let data = include_str!("../chromium.json");
    c.bench_function("solve chromium.json", |b| {
        b.iter(|| solve_demo(black_box(data)))
    });
}
