// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use ibis::Ibis;
use pretty_assertions::assert_eq;

#[test]
fn demo_json_round_trips() {
    let data = include_str!("../examples/demo.json");
    let ibis: Ibis = serde_json::from_str(data).expect("JSON Error?");

    let serialized = serde_json::to_string(&ibis).unwrap();
    let deserialized: Ibis = serde_json::from_str(&serialized).unwrap();

    // TODO: clear out the 'meta' section
    assert_eq!(ibis, deserialized);
}
