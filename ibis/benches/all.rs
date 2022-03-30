// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use criterion::{criterion_group, criterion_main, Criterion};

mod checking_and_planning;
mod checking_only;
mod demo;
mod utils;

use checking_and_planning::*;
use checking_only::*;
use demo::*;
use utils::*;

criterion_group!(
    name = micro_benches;
    config = Criterion::default().sample_size(10000);
    targets = criterion_benchmark_new_vec_push
);
criterion_group!(
    benches,
    criterion_benchmark_checking_only,
    criterion_benchmark_noop_planning,
    criterion_benchmark_solve_demo
);
criterion_main!(benches, micro_benches);
