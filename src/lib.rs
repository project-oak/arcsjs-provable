use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::cell::RefCell;
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

struct Ctx {
    last_id: EntId,
    name_by_id: HashMap<EntId, String>,
    id_by_name: HashMap<String, EntId>,
}

impl Ctx {
    fn new() -> Self {
        Self {
            last_id: 0,
            name_by_id: HashMap::new(),
            id_by_name: HashMap::new(),
        }
    }
}

lazy_static! {
    static ref CTX: Mutex<RefCell<Ctx>> = Mutex::new(RefCell::new(Ctx::new()));
}

fn get_id_by_name(ctx: &Ctx, name: &str) -> Option<EntId> {
    ctx.borrow().id_by_name.get(name).cloned()
}

fn get_name_by_id(id: EntId) -> Option<String> {
    let guard = CTX.lock().expect("Shouldn't fail");
    let ctx = (*guard).borrow();
    ctx.borrow().name_by_id.get(&id).cloned()
}

impl Ent {
    fn new(ctx: &mut Ctx, name: &str) -> Self {
        let id = ctx.last_id;
        ctx.last_id += 1;
        ctx.id_by_name.insert(name.to_string(), id);
        ctx.name_by_id.insert(id, name.to_string());
        Ent { id }
    }

    pub fn name(&self) -> String {
        get_name_by_id(self.id).expect("All entities should have a name")
    }

    fn get_by_name(ctx: &mut Ctx, name: &str) -> Option<Ent> {
        get_id_by_name(&ctx, name).map(|id| Ent { id })
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
