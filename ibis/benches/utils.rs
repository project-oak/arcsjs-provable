// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use criterion::{black_box, Criterion};

fn mut_push(data1: &[u32], data2: u32) -> Vec<u32> {
    let mut data = data1.to_vec();
    data.push(data2);
    data
}

fn concat(data1: &[u32], data2: u32) -> Vec<u32> {
    [data1, &[data2]].concat()
}

pub fn criterion_benchmark_new_vec_push(c: &mut Criterion) {
    let data1: Vec<u32> = (1..100000).into_iter().collect();
    let data2: u32 = 10001;
    c.bench_function("noop_planning_vec_thing_mut", |b| {
        b.iter(|| mut_push(black_box(&data1), black_box(data2)))
    });
    c.bench_function("noop_planning_vec_thing_concat", |b| {
        b.iter(|| concat(black_box(&data1), black_box(data2)))
    });
}
