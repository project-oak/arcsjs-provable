// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use ibis::run_ibis;
use ibis::IbisError;
use std::io::Read;

fn main() -> Result<(), IbisError> {
    eprintln!("{}", ibis::version_info());
    let mut data = String::new();
    std::io::stdin()
        .read_to_string(&mut data)
        .expect("IO Error, reading stdin");
    eprintln!("Preparing graph...");
    let solutions = run_ibis(&data);
    println!(
        "{}",
        serde_json::to_string(&solutions).expect("Couldn't serialize Ibis output")
    );
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
