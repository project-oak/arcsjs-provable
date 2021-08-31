#[macro_use]
extern crate lazy_static;

use crepe::crepe;
use std::collections::HashMap;

type Ent = u32;

struct Context {
    last_ent: Ent,
    names: HashMap<Ent, String>,
}

impl Context {
    fn new() -> Self {
        Self {
            last_ent: 0,
            names: HashMap::new(),
        }
    }

    fn get_ent(&mut self) -> Ent {
        self.last_ent += 1;
        self.last_ent
    }

    fn name(&self, id: &Ent) -> &str {
        // TODO: Check!
        self.names.get(id).unwrap()
    }

    fn decl(&mut self, name: &str) -> Ent {
        let new_id: Ent = self.get_ent();
        // TODO: Check!
        self.names.insert(new_id, name.to_string());
        new_id
    }
}

crepe! {
    @input
    struct IsAClaim(Ent, Ent);
    @input
    struct Exists(Ent);

    @output
    struct IsA(Ent, Ent);

    IsA(x,x) <- Exists(x);
    IsA(x,x) <- IsAClaim(x, _);
    IsA(x,x) <- IsAClaim(_, x);
    IsA(x, z) <- IsAClaim(x, y), IsA(y, z);
}

fn main() {
    let mut ctx = Context::new();
    let mut runtime = Crepe::new();

    let socretes: Ent = ctx.decl("socretes");
    let man: Ent = ctx.decl("man");
    let mortal: Ent = ctx.decl("mortal");

    runtime.extend(&[Exists(socretes), Exists(man), Exists(mortal)]);
    runtime.extend(&[IsAClaim(socretes, man), IsAClaim(man, mortal)]);

    let (reachable,) = &runtime.run();
    for IsA(x, y) in reachable {
        println!("{} is a {}", ctx.name(x), ctx.name(y));
    }
}
