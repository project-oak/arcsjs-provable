// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use std::collections::BTreeMap;
use std::hash::Hash;

#[macro_export]
macro_rules! set {
    () => {
        std::collections::HashSet::new()
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

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct BiMap<T, U> {
    forward: BTreeMap<T, U>,
    back: BTreeMap<U, T>,
}

impl<T: Ord + Eq + Hash + Clone, U: Ord + Eq + Hash + Clone> Default for BiMap<T, U> {
    // Implement Default for BiMap manually to avoid incorrect trait bounds T: Default and
    // U: Default.
    // For more info see: https://github.com/rust-lang/rust/issues/26925
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Eq + Hash + Clone, U: Ord + Eq + Hash + Clone> BiMap<T, U> {
    pub fn new() -> Self {
        Self {
            forward: BTreeMap::new(),
            back: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, t: T, u: U) {
        self.forward.insert(t.clone(), u.clone());
        self.back.insert(u, t);
    }

    pub fn get(&self, t: &T) -> Option<&U> {
        self.forward.get(t)
    }

    pub fn get_back<Q: ?Sized>(&self, u: &Q) -> Option<&T>
    where
        U: std::borrow::Borrow<Q>,
        Q: Ord + Hash + Eq,
    {
        self.back.get(u)
    }
}
