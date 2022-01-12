use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::sync::Mutex;

extern crate ibis_macros;
pub use ibis_macros::*;

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

type EntId = u64;

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Hash)]
pub struct Ent {
    id: EntId,
}

type SolId = u32;

#[derive(Copy, Clone, PartialOrd, Ord, Eq, Hash)]
pub enum Sol {
    Any,
    Id {id: SolId},
}

impl PartialEq for Sol {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Sol::Any, _) => true,
            (_, Sol::Any) => true,
            (Sol::Id{id: self_id}, Sol::Id{id: other_id}) => self_id == other_id,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct SolData {
    edges: BTreeSet<(Ent, Ent)>,
}

impl Default for SolData {
    fn default() -> Self {
        Self {
            edges: BTreeSet::new(),
        }
    }
}

impl SolData {
    pub fn has_edge(&self, from: Ent, to: Ent) -> bool {
        self.edges.contains(&(from, to))
    }

    pub fn add_edge(&self, from: Ent, to: Ent) -> SolData {
        let mut edges = self.edges.clone();
        edges.insert((from, to));
        SolData {
            edges
        }
    }
}


impl Sol {
    fn new_with_id(ctx: &mut Ctx, sol: Sol, solution: SolData) -> Self {
        ctx.solution_to_id.insert(solution.clone(), sol);
        ctx.id_to_solution.insert(sol, solution);
        sol
    }

    fn new(ctx: &mut Ctx, solution: SolData) -> Self {
        ctx.solution_id += 1;
        let sol = Sol::Id{id: ctx.solution_id};
        Sol::new_with_id(ctx, sol, solution)
    }

    pub fn empty() -> Self {
        let guard = CTX.lock().expect("Shouldn't fail");
        let mut ctx = (*guard).borrow_mut();
        let id = Sol::Id{id: 0};
        ctx.ancestors.insert(id, BTreeSet::default());
        Sol::new_with_id(&mut ctx, id, SolData::default()) // unsafe....
    }

    pub fn any() -> Self {
        Self::Any
    }

    pub fn solution(&self) -> SolData {
        let guard = CTX.lock().expect("Shouldn't fail");
        let ctx = (*guard).borrow();
        ctx.borrow().id_to_solution.get(self).cloned().expect("All solution ids should have a solution")
    }

    pub fn ancestors(&self) -> BTreeSet<Sol> {
        let guard = CTX.lock().expect("Shouldn't fail");
        let ctx = (*guard).borrow();
        ctx.borrow().ancestors.get(self).cloned().expect("All solutions should have ancestors")
    }

    pub fn add_edge(&self, from: Ent, to: Ent) -> Sol {
        let new_solution = self.solution().add_edge(from, to);
        let guard = CTX.lock().expect("Shouldn't fail");
        let mut ctx = (*guard).borrow_mut();
        let result = ctx.solution_to_id.get(&new_solution).cloned().unwrap_or_else(|| Sol::new(&mut ctx, new_solution));

        // Track the history of solutions
        use std::collections::hash_map::Entry;
        let ancestors: &mut BTreeSet<Sol> = match ctx.ancestors.entry(result) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(BTreeSet::default())
        };
        ancestors.insert(*self);

        result
    }
}

impl std::fmt::Display for Sol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sol::Any => write!(f, "sol_any"),
            Sol::Id{id: _} => {
                let solution = self.solution();
                let mut edges: Vec<String> = solution.edges.iter().map(|(f, t)| format!("({}, {})", f, t)).collect();
                edges.sort();
                let edges = edges.join(", ");
                f.debug_struct("Sol")
                        .field("{edges}", &edges)
                        .finish()
            }
        }
    }
}

impl std::fmt::Debug for Sol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sol::Any => write!(f, "sol_any"),
            Sol::Id{id} => {
                let solution = self.solution();
                let ancestors = self.ancestors();
                let edges: Vec<String> = solution.edges.iter().map(|(f, t)| format!("({}, {})", f, t)).collect();
                let edges = edges.join(", ");
                f.debug_struct("Sol")
                        .field("id", id)
                        .field("{ancestors}", &ancestors)
                        .field("{edges}", &edges)
                        .finish()
            }
        }
    }
}

struct Ctx {
    last_id: EntId,
    solution_id: u32,
    name_by_id: HashMap<Ent, String>,
    id_by_name: HashMap<String, Ent>,
    id_to_solution: HashMap<Sol, SolData>,
    solution_to_id: HashMap<SolData, Sol>,
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
        ctx.borrow().name_by_id.get(self).cloned().expect("All entities should have a name")
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
macro_rules! relation {
    ($name: ident $args: tt) => {
        use paste::paste;
        paste! {
            @input
            struct [<$name Claim>]$args
            @output
            struct $name $args

            $name $args <- [<$name Claim>]$args;
        }
    };
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
