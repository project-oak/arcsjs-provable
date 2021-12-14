use crepe::crepe;
use paste::paste;
use std::collections::HashMap;

type Ent = u32;
type Rel = u32;

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
    struct P(Rel, Ent, Ent); // Premise

    @input
    struct IsA(Rel); // Relation
    @input
    struct HasType(Rel); // Relation

    @output
    struct C(Rel, Ent, Ent); // Conclusion

    C(r, x,x) <- IsA(r), P(r, x, _);
    C(r, x,x) <- IsA(r), P(r, _, x);
    C(r, x, z) <- IsA(r), C(r, x, y), P(r, y, z);
}

macro_rules! relations {
    ($ctx: expr, $runtime: expr $(, $name: ident )+ ) => {
        {
            $(
                let $name: Ent = $ctx.decl(stringify!($name));
                $runtime.extend(&[paste!( [< $name:camel >] )($name)]);
            )*
            (
            $(
                $name,
            )*
            )
        }
    };
}

macro_rules! entities {
    ($ctx: expr $(, $name: ident )+ ) => {
        {
            $(
                let $name: Ent = $ctx.decl(stringify!($name));
            )*
            (
            $(
                $name,
            )*
            )
        }
    };
}

macro_rules! facts {
    ($runtime: expr $(, $tuple: expr )+ ) => {
        {
            $runtime.extend(&[ $($tuple, )* ]);
        }
    };
}

fn main() {
    let mut ctx = Context::new();
    let mut runtime = Crepe::new();

    let (is_a, _has_type) = relations!(ctx, runtime, is_a, has_type);
    let (socretes, man, mortal) = entities!(ctx, socretes, man, mortal);

    facts!(runtime,
       P(is_a, socretes, man),
       P(is_a, man, mortal)
    );

    let (concs,) = &runtime.run();
    for C(r, x, y) in concs {
        if *r != is_a || *x != socretes {
            continue;
        }
        println!("{} {} {}", ctx.name(x), ctx.name(r), ctx.name(y));
    }
}
