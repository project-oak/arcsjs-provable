// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
use serde::{Deserialize, Serialize};
use crate::recipes::{Recipe, Ibis};

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde()]
pub struct D3Graph {
    nodes: Vec<String>,
    edges: Vec<(String, String, Vec<String>)>,
}

pub trait ToD3 {
    fn to_d3(&self) -> D3Graph;
}

impl D3Graph {
    pub fn add_node(&mut self, node: String) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, from: String, to: String, attrs: Vec<String>) {
        self.edges.push((from, to, attrs));
    }
}

impl ToD3 for Recipe {
    fn to_d3(&self) -> D3Graph {
        D3Graph::default()
    }
}

impl ToD3 for Ibis {
    fn to_d3(&self) -> D3Graph {
        D3Graph::default()
    }
}
