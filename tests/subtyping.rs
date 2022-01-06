use crepe::crepe;
use ibis::{set, facts, Ent};
use pretty_assertions::assert_eq;

#[test]
fn static_subtyping_socretes_is_mortal() {
    crepe! {
        @input
        #[derive(Debug)]
        struct SubtypeClaim(Ent, Ent);
        @output
        #[derive(Debug)]
        struct Subtype(Ent, Ent);
        Subtype(x,y) <- SubtypeClaim(x, y);

        @input
        #[derive(Debug)]
        struct HasTagClaim(Ent, Ent);
        @output
        #[derive(Debug)]
        struct HasTag(Ent, Ent);
        HasTag(x,y) <- HasTagClaim(x, y);

        Subtype(x,x) <- Subtype(x, _);
        Subtype(x,x) <- Subtype(_, x);
        Subtype(x, z) <- Subtype(x, y), Subtype(y, z);
        HasTag(x, z) <- Subtype(x, y), HasTag(y, z);
    }

    let mut runtime = Crepe::new();

    let plato = Ent::by_name("plato");
    let socretes = Ent::by_name("socretes");
    let man = Ent::by_name("man");
    let mortal = Ent::by_name("mortal");

    // specify all the 'dynamic' facts
    facts!(
        runtime,
        Subtype(plato, man),
        Subtype(socretes, man),
        HasTag(man, mortal)
    );

    let (subtypes, tags) = &runtime.run();
    assert_eq!(subtypes, &set![
        Subtype(man, man),
        Subtype(socretes, socretes),
        Subtype(socretes, man),
        Subtype(plato, plato),
        Subtype(plato, man)
    ]);

    assert_eq!(tags, &set![
        HasTag(man, mortal),
        HasTag(socretes, mortal),
        HasTag(plato, mortal)
    ]);
}


#[test]
fn dynamic_subtyping_mr_socretes_is_mortal() {
    crepe! {
        @input
        #[derive(Debug)]
        struct SubtypeClaim(Ent, Ent);
        @output
        #[derive(Debug)]
        struct Subtype(Ent, Ent);
        Subtype(x,y) <- SubtypeClaim(x, y);

        @input
        #[derive(Debug)]
        struct HasTagClaim(Ent, Ent);
        @output
        #[derive(Debug)]
        struct HasTag(Ent, Ent);
        HasTag(x,y) <- HasTagClaim(x, y);

        @output
        #[derive(Debug)]
        struct Man(Ent);
        Man(x) <- Subtype(x, Ent::by_name("man"));

        Man(Ent::by_name(&("Mr. ".to_string()+y.name()))) <- Man(y), ((&y.name()).starts_with("Mr. "));
        Subtype(x,x) <- Subtype(x, _);
        Subtype(x,x) <- Subtype(_, x);
        Subtype(x, z) <- Subtype(x, y), Subtype(y, z);
        HasTag(x, z) <- Subtype(x, y), HasTag(y, z);
    }

    let mut runtime = Crepe::new();

    let plato = Ent::by_name("plato");
    let socretes = Ent::by_name("socretes");
    let man = Ent::by_name("man");
    let mortal = Ent::by_name("mortal");

    // specify all the 'dynamic' facts
    facts!(
        runtime,
        Subtype(plato, man),
        Subtype(socretes, man),
        HasTag(man, mortal)
    );

    let (subtypes, tags, men) = &runtime.run();
    for Subtype(x, y) in subtypes {
        println!("{} is a {}", x, y);
    }
    for HasTag(x, y) in tags {
        println!("{} has tag {}", x, y);
    }
    for Man(x) in men {
        println!("{} 'is a man'", x);
    }
    assert_eq!(subtypes, &set![
        Subtype(man, man),
        Subtype(socretes, socretes),
        Subtype(socretes, man),
        Subtype(plato, plato),
        Subtype(plato, man)
    ]);

    assert_eq!(tags, &set![
        HasTag(man, mortal),
        HasTag(socretes, mortal),
        HasTag(plato, mortal)
    ]);
}
