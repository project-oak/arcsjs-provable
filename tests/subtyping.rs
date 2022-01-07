use crepe::crepe;
use ibis::{facts, set, Ent};
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
        struct SubtypeClaim(Ent, Ent);
        @output
        #[derive(Debug)]
        struct Subtype(Ent, Ent);
        Subtype(x,y) <- SubtypeClaim(x, y);

        @input
        #[derive(Debug)]
        struct InstanceClaim(Ent, Ent);
        @output
        #[derive(Debug)]
        struct Instance(Ent, Ent);
        Instance(x,y) <- InstanceClaim(x, y);

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

#[test]
fn list_types_subtype() {
    crepe! {
        @input
        #[derive(Debug)]
        struct SubtypeClaim(Ent, Ent);
        @output
        #[derive(Debug)]
        struct Subtype(Ent, Ent);
        Subtype(x,y) <- SubtypeClaim(x, y);

        struct Type(Ent);

        Type(t) <- Subtype(t, _);
        Type(t) <- Subtype(_, t);
        Type(t) <- Instance(_, t);

        @input
        #[derive(Debug)]
        struct InstanceClaim(Ent, Ent);
        @output
        #[derive(Debug)]
        struct Instance(Ent, Ent);
        Instance(x,y) <- InstanceClaim(x, y);

        Subtype(x, z) <- Subtype(x, y), Subtype(y, z);
        Instance(x, z) <- Instance(x, y), Subtype(y, z);

        Subtype(
            x,
            y,
        ) <- Type(x),
            Type(y),
            (x.name().starts_with("List(")),
            (y.name().starts_with("List(")),
            (x.name().ends_with(")")),
            (y.name().ends_with(")")),
            Subtype(
                Ent::by_name(&x.name()[5..x.name().len()-1]),
                Ent::by_name(&y.name()[5..y.name().len()-1])
            );
    }

    let mut runtime = Crepe::new();

    let plato = Ent::by_name("plato");
    let socretes = Ent::by_name("socretes");
    let man = Ent::by_name("man");
    let mortal = Ent::by_name("mortal");
    let list_man = Ent::by_name("List(man)");
    let list_mortal = Ent::by_name("List(mortal)");

    // specify all the 'dynamic' facts
    facts!(
        runtime,
        Subtype(man, mortal),
        Subtype(list_man, list_man),
        Subtype(list_mortal, list_mortal),
        Instance(plato, man),
        Instance(socretes, man)
    );

    let (subtypes, instances) = &runtime.run();
    assert_eq!(
        subtypes,
        &set![
            Subtype(man, mortal),
            Subtype(list_man, list_man),
            Subtype(list_man, list_mortal),
            Subtype(list_mortal, list_mortal)
        ]
    );

    assert_eq!(
        instances,
        &set![
            Instance(socretes, man),
            Instance(plato, man),
            Instance(socretes, mortal),
            Instance(plato, mortal)
        ]
    );
}

#[test]
fn iterator_types_subtype() {
    crepe! {
        @input
        #[derive(Debug)]
        struct SubtypeClaim(Ent, Ent);
        @output
        #[derive(Debug, Ord, PartialOrd)]
        struct Subtype(Ent, Ent);
        Subtype(x,y) <- SubtypeClaim(x, y);

        @input
        #[derive(Debug)]
        struct TypeClaim(Ent);
        struct Type(Ent);
        Type(x) <- TypeClaim(x);

        @input
        #[derive(Debug)]
        struct GenericTypeClaim(Ent);
        struct GenericType(Ent);
        GenericType(x) <- GenericTypeClaim(x);

        @input
        #[derive(Debug)]
        struct InductiveTypeClaim(Ent);
        struct InductiveType(Ent);
        InductiveType(x) <- InductiveTypeClaim(x);

        Type(t) <- Subtype(t, _);
        Type(t) <- Subtype(_, t);
        Type(t) <- Instance(_, t);
        Type(t) <- InductiveType(t);
        Type(t) <- GenericType(t);

        struct SpecialisationBy(Ent, Ent, Ent);
        SpecialisationBy(Ent::by_name(&format!("{}({})", y.name(), x.name())), y, x) <-
            GenericType(y),
            Type(x),
            Type(Ent::by_name(&format!("{}({})", y.name(), x.name())));

        @input
        #[derive(Debug)]
        struct InstanceClaim(Ent, Ent);
        @output
        #[derive(Debug)]
        struct Instance(Ent, Ent);
        Instance(x,y) <- InstanceClaim(x, y);

        Subtype(x, x) <- Type(x);
        Subtype(x, z) <- Subtype(x, y), Subtype(y, z);
        Instance(x, z) <- Instance(x, y), Subtype(y, z);

        Subtype(
            x,
            y
        ) <-
            SpecialisationBy(x, x_wrapper, x_arg),
            InductiveType(x_wrapper),
            Subtype(x_wrapper, y_wrapper),
            SpecialisationBy(y, y_wrapper, y_arg),
            InductiveType(y_wrapper),
            Subtype(x_arg, y_arg);
    }

    let mut runtime = Crepe::new();

    let plato = Ent::by_name("plato");
    let socretes = Ent::by_name("socretes");
    let man = Ent::by_name("man");
    let mortal = Ent::by_name("mortal");
    let list = Ent::by_name("List");
    let iterable = Ent::by_name("Iterable");
    let list_man = Ent::by_name("List(man)");
    let iterable_man = Ent::by_name("Iterable(man)");
    let list_mortal = Ent::by_name("List(mortal)");
    let iterable_mortal = Ent::by_name("Iterable(mortal)");

    // specify all the 'dynamic' facts
    facts!(
        runtime,
        Subtype(man, mortal),
        Type(list),
        Type(iterable),
        Subtype(list, iterable),
        Instance(plato, man),
        Instance(socretes, man),
        GenericType(list),
        GenericType(iterable),
        InductiveType(list),
        InductiveType(iterable),
        // We shouldn't have to list these
        Type(list_man),
        Type(list_mortal),
        Type(iterable_mortal),
        Type(iterable_man)
    );

    let (subtypes, instances) = &runtime.run();
    let mut subtypes: Vec<Subtype> = subtypes.iter().filter(|Subtype(x, y)| x != y).map(|x|x.clone()).collect();
    let mut expected = vec![
            Subtype(man, mortal),
            Subtype(list, iterable),
            Subtype(list_man, list_mortal), // Check that a list of men, is a list of mortals
            Subtype(list_man, iterable_man), // Check that a list of men, is an interable of men
            Subtype(iterable_man, iterable_mortal), // Check that an iterable of men, is an iterable of mortals
            Subtype(list_mortal, iterable_mortal), // Check that a list of mortals, is an iterable of mortals
            Subtype(list_man, iterable_mortal) // Check that a list of men, is an iterable of mortals
        ];

    subtypes.sort();
    expected.sort();

    assert_eq!(
        subtypes,
        expected,
    );

    assert_eq!(
        instances,
        &set![
            Instance(socretes, man),
            Instance(plato, man),
            Instance(socretes, mortal),
            Instance(plato, mortal)
        ]
    );
}
