// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use serde::{Deserialize, Serialize};
use super::context::{Ctx, CTX};
use super::ent::*;
use super::solution::*;
use super::util::*;
use std::borrow::Borrow;
#[cfg(feature = "ancestors")]
use std::collections::BTreeSet;

pub type SolutionIdBackingType = u32;

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(from = "SolutionIdBackingType")]
pub struct Sol {
    pub id: SolutionIdBackingType,
}

impl From<SolutionIdBackingType> for Sol {
    fn from(id: SolutionIdBackingType) -> Self {
        Self { id }
    }
}

impl Sol {
    fn new_with_id(ctx: &mut Ctx, sol: Sol, solution: Solution) -> Self {
        ctx.id_to_solution.insert(sol, solution);
        #[cfg(feature = "ancestors")]
        ctx.ancestors.insert(sol, BTreeSet::default());
        sol
    }

    fn new(ctx: &mut Ctx, solution: Solution) -> Self {
        ctx.solution_id += 1;
        let sol = Sol {
            id: ctx.solution_id,
        };
        Sol::new_with_id(ctx, sol, solution)
    }

    pub fn empty() -> Self {
        let guard = CTX.lock().expect("Shouldn't fail");
        let mut ctx = (*guard).borrow_mut();
        let id = Sol { id: 0 };
        // The following inserts the 'default' Sol with the 'zero' id, clobbering the old data
        // This is safe because we only ever insert the 'default'
        Sol::new_with_id(&mut ctx, id, Solution::default())
    }

    fn get_solution(&self, ctx: &Ctx) -> Solution {
        ctx.borrow()
            .id_to_solution
            .get(self)
            .cloned()
            .expect("All solution ids should have a solution")
    }

    pub fn solution(&self) -> Solution {
        let guard = CTX.lock().expect("Shouldn't fail");
        let ctx = (*guard).borrow();
        self.get_solution(&ctx)
    }

    #[cfg(feature = "ancestors")]
    pub fn ancestors(&self) -> BTreeSet<Sol> {
        let guard = CTX.lock().expect("Shouldn't fail");
        let ctx = (*guard).borrow();
        ctx.borrow()
            .ancestors
            .get(self)
            .cloned()
            .expect("All solutions should have ancestors")
    }

    #[cfg(feature = "ancestors")]
    fn add_ancestor(&self, ctx: &mut Ctx, parent: Sol) {
        ctx.ancestors
            .get_mut(self)
            .cloned()
            .expect("All solutions should have ancestors")
            .insert(parent);
    }

    pub fn make_child(&self, update: &dyn Fn(&Solution) -> Solution) -> Sol {
        let guard = CTX.lock().expect("Shouldn't fail");
        let mut ctx = (*guard).borrow_mut();
        let new_solution = update(&self.get_solution(&ctx));
        let result = ctx
            .id_to_solution
            .get_back(&new_solution)
            .cloned()
            .unwrap_or_else(|| Sol::new(&mut ctx, new_solution));

        // Track the history of solutions
        #[cfg(feature = "ancestors")]
        result.add_ancestor(&mut ctx, *self);
        result
    }

    pub fn add_edge(&self, from: Ent, to: Ent) -> Sol {
        self.make_child(&|sol| sol.add_edge(from, to))
    }

    pub fn edges(&self) -> std::collections::BTreeSet<(Ent, Ent)> {
        self.solution().edges
    }

    pub fn has_edge(&self, from: Ent, to: Ent) -> bool {
        self.solution().has_edge(from, to)
    }

    #[cfg(feature = "ancestors")]
    fn ancestor_string(&self) -> String {
        let ancestors: Vec<String> = self
            .ancestors()
            .iter()
            .map(|anc| anc.id.to_string())
            .collect();
        ancestors.join(", ")
    }

    #[cfg(not(feature = "ancestors"))]
    fn ancestor_string(&self) -> String {
        "<ancestors disabled>".to_string()
    }
}

impl std::fmt::Display for Sol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let solution = self.solution();
        let mut edges: Vec<String> = solution
            .edges
            .iter()
            .map(|(f, t)| format!("({}, {})", f, t))
            .collect();
        edges.sort();
        let edges = edges.join(", ");
        f.debug_struct("Sol").field("{edges}", &edges).finish()
    }
}

impl std::fmt::Debug for Sol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let solution = self.solution();

        let edges: Vec<String> = solution
            .edges
            .iter()
            .map(|(f, t)| format!("({}, {})", f, t))
            .collect();
        let edges = edges.join(", ");
        f.debug_struct("Sol")
            .field("id", &self.id)
            .field("{ancestors}", &Raw(self.ancestor_string()))
            .field("{edges}", &Raw(edges))
            .finish()
    }
}
