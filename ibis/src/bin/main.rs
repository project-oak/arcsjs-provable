// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use ibis::recipies::Ibis;
use ibis::IbisError;
use std::io::Read;

fn main() -> Result<(), IbisError> {
    let mut runtime = Ibis::new();

    let mut data = String::new();
    std::io::stdin()
        .read_to_string(&mut data)
        .expect("IO Error, reading stdin");
    // TODO: Use ibis::Error and https://serde.rs/error-handling.html instead of expect.
    let recipies: Ibis = serde_json::from_str(&data).expect("JSON Error?");

    runtime.add_recipies(recipies);

    eprintln!("Preparing graph...");
    let solutions = runtime.extract_solutions();
    eprintln!("Done");
    println!("{}", solutions.to_dot());
    Ok(())
}

/*
#[test]
fn demo_json_round_trips() {
    let data = include_str!("../../demo.json");
    let recipe: Recipe = serde_json::from_str(data).expect("JSON Error?");

    let serialized = serde_json::to_string(&recipe).unwrap();
    let deserialized: Recipe = serde_json::from_str(&serialized).unwrap();

}*/
