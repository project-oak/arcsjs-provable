use crepe::crepe;
use paste::paste;
use std::collections::HashMap;

// type Ent = &'static str;
type EntId = u32;

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Hash)]
struct Ent {
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
    fn new(name: &str) -> Self {
        let id = unsafe {
            LAST_ENT += 1;
            LAST_ENT
        };
        name_by_id().insert(id, name.to_string());
        id_by_name().insert(name.to_string(), id);
        Ent { id }
    }

    fn name(&self) -> &String {
        name_by_id()
            .get(&self.id)
            .expect("All entities should have a name")
    }

    fn get_by_name(name: &str) -> Option<Ent> {
        id_by_name().get(name).map(|id| Ent { id: *id })
    }

    fn by_name(name: &str) -> Ent {
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

macro_rules! relation {
    ($name: ident $(, $arg: expr )* ) => {
        @input
        struct paste!([<$name Claim>])( $($arg, )* )
        @output
        struct $name( $($arg, )* )

        $name( $($arg, )*) <- paste!([<$name Claim>])( $($arg)* );
    }
}

macro_rules! facts {
    ($runtime: expr $(, $name: ident ($first: expr $(, $arg: expr )*) )+ ) => {
        {
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

crepe! {
    struct Exists(Ent);

    @input
    struct SubtypeClaim(Ent, Ent);
    @output
    struct Subtype(Ent, Ent);
    Subtype(x,y) <- SubtypeClaim(x, y);
    // relation!{Subtype(Ent, Ent)}

    @input
    struct HasTagClaim(Ent, Ent);
    @output
    struct HasTag(Ent, Ent);
    HasTag(x,y) <- HasTagClaim(x, y);

    Exists(x) <- Subtype(x, _);
    Exists(x) <- Subtype(_, x);

    @output
    struct Man(Ent);
    Man(x) <- Subtype(x, Ent::by_name("man")), Subtype(x, Ent::by_name("individual"));

    Man(Ent::by_name(&("Mr. ".to_string()+y.name()))) <- Man(y), (&y.name()[0..4] != "Mr. ");
    // Subtype("man", "mortal") <- (true);

    Subtype(x,x) <- Subtype(x, _);
    Subtype(x,x) <- Subtype(_, x);
    Subtype(x, z) <- Subtype(x, y), Subtype(y, z);
    HasTag(x, z) <- Subtype(x, y), HasTag(y, z);
}

fn main() {
    let mut runtime = Crepe::new();

    let plato = Ent::by_name("plato");
    let individual = Ent::by_name("individual");
    let socretes = Ent::by_name("socretes");
    let man = Ent::by_name("man");
    let mortal = Ent::by_name("mortal");

    // specify all the 'dynamic' facts
    facts!(
        runtime,
        Subtype(plato, individual),
        Subtype(socretes, individual),
        Subtype(plato, man),
        Subtype(socretes, man),
        HasTag(man, mortal)
    );

    let (subtypes, tags, men) = &runtime.run();
    for Subtype(x, y) in subtypes {
        //if *x != "socretes" {
        //continue;
        //}
        println!("{} is a {}", x, y);
    }
    for HasTag(x, y) in tags {
        //if *x != "socretes" {
        //continue;
        //}
        println!("{} has tag {}", x, y);
    }
    for Man(x) in men {
        //if *x != "socretes" {
        //continue;
        //}
        println!("{} 'is a man'", x);
    }
}
