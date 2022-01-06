use std::collections::HashMap;

#[macro_export]
macro_rules! set {
    () => {
        std::collections::HashSet::new()
    };
    ( $( $arg: expr ),* ) => {
        {
            let mut st = set!();
            $(
                st.insert( $arg );
            )*
            st
        }
    };
}


// type Ent = &'static str;
type EntId = u32;

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Hash)]
pub struct Ent {
    id: EntId,
}

static mut LAST_ENT: EntId = 0;

static mut NAME_BY_ID: Option<HashMap<EntId, String>> = None; // HashMap::new();
static mut ID_BY_NAME: Option<HashMap<String, EntId>> = None; // HashMap::new();

fn id_by_name() -> &'static mut HashMap<String, EntId> {
    unsafe {
        if ID_BY_NAME == None {
            ID_BY_NAME = Some(HashMap::new());
        }
        ID_BY_NAME.as_mut().expect("Should never fail: ID_BY_NAME")
    }
}

fn name_by_id() -> &'static mut HashMap<EntId, String> {
    unsafe {
        if NAME_BY_ID == None {
            NAME_BY_ID = Some(HashMap::new());
        }
        NAME_BY_ID.as_mut().expect("Should never fail: NAME_BY_ID")
    }
}

impl Ent {
    pub fn new(name: &str) -> Self {
        let id = unsafe {
            LAST_ENT += 1;
            LAST_ENT
        };
        name_by_id().insert(id, name.to_string());
        id_by_name().insert(name.to_string(), id);
        Ent { id }
    }

    pub fn name(&self) -> &String {
        name_by_id()
            .get(&self.id)
            .expect("All entities should have a name")
    }

    pub fn get_by_name(name: &str) -> Option<Ent> {
        id_by_name().get(name).map(|id| Ent { id: *id })
    }

    pub fn by_name(name: &str) -> Ent {
        Ent::get_by_name(name).unwrap_or_else(|| Ent::new(name))
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
        paste!{
            @input
            struct [<$name Claim>]$args
            @output
            struct $name $args

            $name $args <- [<$name Claim>]$args;
        }
    }
}

#[macro_export]
macro_rules! facts {
    ($runtime: expr $(, $name: ident ($first: expr $(, $arg: expr )*) )+ ) => {
        {
            use paste::paste;
            $(
                $runtime.extend(&[
                    paste!( [< $name Claim >]) (
                        $first $(, $arg)*
                    ),
                ]);
            )*
        }
    };
}
