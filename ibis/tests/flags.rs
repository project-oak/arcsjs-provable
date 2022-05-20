// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use ibis::{get_solutions, Ibis};
use pretty_assertions::assert_eq;

#[test]
fn unknown_flag_generates_warning() {
    let data = r#"
{
    "flags": {
        "unknown_and_unexpected_flag": true
    }
}
"#;
    let results: Ibis = get_solutions(data, None);
    assert_eq!(results.shared.warnings.len(), 1);
    let warning = results
        .shared
        .warnings
        .get(0)
        .expect("Should have a single value");
    let expected = r#"Unknown flag 'unknown_and_unexpected_flag' set to: true"#;
    assert!(
        warning.starts_with(expected),
        "unexpected warning:\n'{}'\n'{}'",
        warning,
        expected
    );
}

#[test]
fn known_flags_round_trip_some_true() {
    let data = r#"
{
    "flags": {
        "planning": true
    }
}
"#;
    let results: Ibis = get_solutions(data, None);
    assert_eq!(results.config.flags.get("planning"), Some(&true));
    assert_eq!(results.shared.warnings, Vec::<&str>::new());
}

#[test]
fn known_flags_round_trip_some_false() {
    let data = r#"
{
    "flags": {
        "planning": false
    }
}
"#;
    let results: Ibis = get_solutions(data, None);
    assert_eq!(results.config.flags.get("planning"), Some(&false));
    assert_eq!(results.shared.warnings, Vec::<&str>::new());
}

#[test]
fn known_flags_round_trip_none() {
    let data = r#"
{
    "flags": {
    }
}
"#;
    let results: Ibis = get_solutions(data, None);
    assert_eq!(results.config.flags.get("planning"), None);
    assert_eq!(results.shared.warnings, Vec::<&str>::new());
}
