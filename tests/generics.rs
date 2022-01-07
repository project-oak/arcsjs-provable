use crepe::crepe;
use ibis::{facts, set, Ent};
use pretty_assertions::assert_eq;

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

        struct SpecialisationOfClaim(Ent, Ent);
        struct SpecialisationOf(Ent, Ent);
        SpecialisationOf(x, y) <- SpecialisationOfClaim(x, y);

        SpecialisationOf(x, y) <- SpecialisationBy(x, y, _);

        SpecialisationOf(x, y) <-
            GenericType(y),
            Type(x),
            (x.name().starts_with(&(y.name()+"("))),
            (x.name().ends_with(")"));

        struct SpecialisationByClaim(Ent, Ent, Ent);
        struct SpecialisationBy(Ent, Ent, Ent);
        SpecialisationBy(x, y, z) <- SpecialisationByClaim(x, y, z);

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

