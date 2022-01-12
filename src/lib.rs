use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::cell::RefCell;
#[cfg(feature = "ancestors")]
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::sync::Mutex;

mod ids;
mod solution;
mod util;
extern crate ibis_macros;

pub use ibis_macros::*;
pub use ids::*;
use solution::*;
pub use util::*;

impl Sol {
    fn new_with_id(ctx: &mut Ctx, sol: Sol, solution: Solution) -> Self {
        ctx.solution_to_id.insert(solution.clone(), sol);
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
            .solution_to_id
            .get(&new_solution)
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

struct Ctx {
    last_id: EntId,
    solution_id: u32,
    name_by_id: HashMap<Ent, String>,
    id_by_name: HashMap<String, Ent>,
    id_to_solution: HashMap<Sol, Solution>,
    solution_to_id: HashMap<Solution, Sol>,
    #[cfg(feature = "ancestors")]
    ancestors: HashMap<Sol, BTreeSet<Sol>>,
}

impl Ctx {
    fn new() -> Self {
        Self {
            last_id: 0,
            solution_id: 0, // zero is never used except for the 'empty' solution
            name_by_id: HashMap::new(),
            id_by_name: HashMap::new(),
            id_to_solution: HashMap::new(),
            solution_to_id: HashMap::new(),
            #[cfg(feature = "ancestors")]
            ancestors: HashMap::new(),
        }
    }
}

lazy_static! {
    static ref CTX: Mutex<RefCell<Ctx>> = Mutex::new(RefCell::new(Ctx::new()));
}

impl Ent {
    fn new(ctx: &mut Ctx, name: &str) -> Self {
        let id = ctx.last_id;
        ctx.last_id += 1;
        let ent = Ent { id };
        ctx.id_by_name.insert(name.to_string(), ent);
        ctx.name_by_id.insert(ent, name.to_string());
        ent
    }

    pub fn name(&self) -> String {
        let guard = CTX.lock().expect("Shouldn't fail");
        let ctx = (*guard).borrow();
        ctx.borrow()
            .name_by_id
            .get(self)
            .cloned()
            .expect("All entities should have a name")
    }

    fn get_by_name(ctx: &mut Ctx, name: &str) -> Option<Ent> {
        ctx.id_by_name.get(name).cloned()
    }

    pub fn by_name(name: &str) -> Ent {
        let guard = CTX.lock().expect("Shouldn't fail");
        let mut ctx = (*guard).borrow_mut();
        Ent::get_by_name(&mut ctx, name).unwrap_or_else(|| Ent::new(&mut ctx, name))
    }
}

impl std::fmt::Display for Ent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::fmt::Debug for Ent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ent")
            .field("id", &self.id)
            .field("{name}", &self.name())
            .finish()
    }
}

#[macro_export]
macro_rules! facts {
    ($runtime: expr $(, $name: ident ($($arg: expr ),*) )* $(,)? ) => {
        {
            use paste::paste;
            $(
                $runtime.extend(&[
                    paste!( [< $name Claim >]) (
                        $($arg, )*
                    ),
                ]);
            )*
        }
    }
}

#[macro_export]
macro_rules! ent {
    ($fmt: expr) => {
        Ent::by_name($fmt)
    };
    ($fmt: expr, $($names: expr),*) => {
        Ent::by_name(&format!($fmt, $( $names.name(), )*))
    }
}

#[macro_export]
macro_rules! apply {
    ($type: expr, $arg: expr) => {
        Ent::by_name(&format!("{}({})", $type.name(), $arg.name()))
    };
}

#[macro_export]
macro_rules! is_a {
    ($type: expr, $parent: expr) => {
        ($type.name().starts_with(&($parent.name() + "(")) && $type.name().ends_with(")"))
    };
}
