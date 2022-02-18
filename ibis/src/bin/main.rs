// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#[cfg(feature = "dot")]
use ibis::best_solutions_to_dot;
use ibis::IbisError;
use std::io::Read;

fn main() -> Result<(), IbisError> {
    let mut data = String::new();
    std::io::stdin()
        .read_to_string(&mut data)
        .expect("IO Error, reading stdin");
    println!("{}", best_solutions_to_dot(&data));
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
