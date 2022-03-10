// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use criterion::{criterion_group, criterion_main};

mod checking_and_planning;
mod checking_only;
mod demo;

use checking_and_planning::*;
use checking_only::*;
use demo::*;

criterion_group!(
    benches,
    criterion_benchmark_checking_only,
    criterion_benchmark_noop_planning,
    criterion_benchmark_solve_demo
);
criterion_main!(benches);
