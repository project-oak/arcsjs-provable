// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

mod bimap;
pub use bimap::*;

#[macro_export]
macro_rules! map {
    () => {
        std::collections::BTreeMap::new()
    };
    ( $( $key: expr => $value: expr ),* $(,)?) => {
        {
            let mut st = map!();
            $(
                st.insert( $arg, $value );
            )*
            st
        }
    };
}

#[macro_export]
macro_rules! set {
    () => {
        std::collections::BTreeSet::new()
    };
    ( $( $arg: expr ),* $(,)?) => {
        {
            let mut st = set!();
            $(
                st.insert( $arg );
            )*
            st
        }
    };
}

pub struct Raw(pub String);

impl std::fmt::Debug for Raw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn make<
    'a,
    T: 'a,
    Iter: IntoIterator<Item = &'a T>,
    U,
    F: Fn(&'a T) -> U,
    Res: FromIterator<U>,
>(
    items: Iter,
    f: F,
) -> Res {
    items.into_iter().map(f).collect()
}
