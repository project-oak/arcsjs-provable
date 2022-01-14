use crepe::crepe;
use ibis::{facts, set, Ent};
use pretty_assertions::assert_eq;

#[test]
fn static_subtyping_socretes_is_mortal() {
    crepe! {
        @input
        #[derive(Debug)]
        struct SubtypeInput(Ent, Ent);
        @output
        #[derive(Debug)]
        struct Subtype(Ent, Ent);
        Subtype(x,y) <- SubtypeInput(x, y);

        @input
        #[derive(Debug)]
        struct HasTagInput(Ent, Ent);
        @output
        #[derive(Debug)]
        struct HasTag(Ent, Ent);
        HasTag(x,y) <- HasTagInput(x, y);

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
    assert_eq!(
        subtypes,
        &set![
            Subtype(man, man),
            Subtype(socretes, socretes),
            Subtype(socretes, man),
            Subtype(plato, plato),
            Subtype(plato, man)
        ]
    );

    assert_eq!(
        tags,
        &set![
            HasTag(man, mortal),
            HasTag(socretes, mortal),
            HasTag(plato, mortal)
        ]
    );
}

#[test]
fn dynamic_subtyping_mr_socretes_is_mortal() {
    crepe! {
        @input
        #[derive(Debug)]
        struct SubtypeInput(Ent, Ent);
        @output
        #[derive(Debug)]
        struct Subtype(Ent, Ent);
        Subtype(x,y) <- SubtypeInput(x, y);

        @input
        #[derive(Debug)]
        struct InstanceInput(Ent, Ent);
        @output
        #[derive(Debug)]
        struct Instance(Ent, Ent);
        Instance(x,y) <- InstanceInput(x, y);

        @output
        #[derive(Debug)]
        struct Man(Ent);
        Man(x) <- Instance(x, Ent::by_name("man"));

        Man(Ent::by_name(&("Mr. ".to_string()+&y.name()))) <- Man(y), ((&y.name()).starts_with("Mr. "));
        Subtype(x, z) <- Subtype(x, y), Subtype(y, z);
        Instance(x, z) <- Instance(x, y), Subtype(y, z);
    }

    let mut runtime = Crepe::new();

    let plato = Ent::by_name("plato");
    let socretes = Ent::by_name("socretes");
    let man = Ent::by_name("man");
    let mortal = Ent::by_name("mortal");

    // specify all the 'dynamic' facts
    facts!(
        runtime,
        Subtype(man, mortal),
        Instance(plato, man),
        Instance(socretes, man)
    );

    let (subtypes, instances, men) = &runtime.run();
    assert_eq!(subtypes, &set![Subtype(man, mortal)]);

    assert_eq!(
        instances,
        &set![
            Instance(socretes, man),
            Instance(plato, man),
            Instance(socretes, mortal),
            Instance(plato, mortal)
        ]
    );

    assert_eq!(men, &set![Man(socretes), Man(plato)]);
}
