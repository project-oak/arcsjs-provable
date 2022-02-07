// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum IbisError {}

impl fmt::Display for IbisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IbisError")?;
        todo!();
    }
}

impl Error for IbisError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!()
    }
}
