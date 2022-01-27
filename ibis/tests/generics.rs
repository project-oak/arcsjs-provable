use crepe::crepe;
use ibis::{facts, Ent};
use pretty_assertions::assert_eq;

#[test]
fn fixed_list_types_subtype() {
    crepe! {
        @input
        #[derive(Debug)]
        struct SubtypeInput(Ent, Ent);
        @output
        #[derive(Debug, Ord, PartialOrd)]
        struct Subtype(Ent, Ent);
        Subtype(x,y) <- SubtypeInput(x, y);

        struct Type(Ent);

        Type(t) <- Subtype(t, _);
        Type(t) <- Subtype(_, t);
        Type(t) <- Instance(_, t);

        @input
        #[derive(Debug)]
        struct InstanceInput(Ent, Ent);
        @output
        #[derive(Debug, Ord, PartialOrd)]
        struct Instance(Ent, Ent);
        Instance(x,y) <- InstanceInput(x, y);

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

    let (subtypes, instances) = runtime.run();
    let mut subtypes: Vec<Subtype> = subtypes.iter().cloned().collect();
    subtypes.sort();
    let mut instances: Vec<Instance> = instances.iter().cloned().collect();
    instances.sort();
    let mut expected = vec![
        Subtype(man, mortal),
        Subtype(list_man, list_man),
        Subtype(list_man, list_mortal),
        Subtype(list_mortal, list_mortal),
    ];
    expected.sort();
    assert_eq!(subtypes, expected);

    let mut expected = vec![
        Instance(socretes, man),
        Instance(plato, man),
        Instance(socretes, mortal),
        Instance(plato, mortal),
    ];
    expected.sort();
    assert_eq!(instances, expected);
}

#[test]
fn fixed_iterator_types_subtype() {
    crepe! {
        @input
        #[derive(Debug)]
        struct SubtypeInput(Ent, Ent);
        @output
        #[derive(Debug, Ord, PartialOrd)]
        struct Subtype(Ent, Ent);
        Subtype(x,y) <- SubtypeInput(x, y);

        @input
        #[derive(Debug)]
        struct TypeInput(Ent);
        struct Type(Ent);
        Type(x) <- TypeInput(x);

        @input
        #[derive(Debug)]
        struct GenericTypeInput(Ent);
        struct GenericType(Ent);
        GenericType(x) <- GenericTypeInput(x);

        @input
        #[derive(Debug)]
        struct InductiveTypeInput(Ent);
        struct InductiveType(Ent);
        InductiveType(x) <- InductiveTypeInput(x);

        Type(t) <- Subtype(t, _);
        Type(t) <- Subtype(_, t);
        Type(t) <- Instance(_, t);
        Type(t) <- InductiveType(t);
        Type(t) <- GenericType(t);

        struct SpecialisationOfInput(Ent, Ent);
        struct SpecialisationOf(Ent, Ent);
        SpecialisationOf(x, y) <- SpecialisationOfInput(x, y);

        SpecialisationOf(x, y) <- SpecialisationBy(x, y, _);

        SpecialisationOf(x, y) <-
            GenericType(y),
            Type(x),
            (x.name().starts_with(&(y.name()+"("))),
            (x.name().ends_with(")"));

        struct SpecialisationByInput(Ent, Ent, Ent);
        struct SpecialisationBy(Ent, Ent, Ent);
        SpecialisationBy(x, y, z) <- SpecialisationByInput(x, y, z);

        SpecialisationBy(Ent::by_name(&format!("{}({})", y.name(), x.name())), y, x) <-
            GenericType(y),
            Type(x),
            Type(Ent::by_name(&format!("{}({})", y.name(), x.name())));

        @input
        #[derive(Debug)]
        struct InstanceInput(Ent, Ent);
        @output
        #[derive(Debug, Ord, PartialOrd)]
        struct Instance(Ent, Ent);
        Instance(x,y) <- InstanceInput(x, y);

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
    let mut subtypes: Vec<Subtype> = subtypes
        .iter()
        .filter(|Subtype(x, y)| x != y)
        .cloned()
        .collect();
    subtypes.sort();
    let mut instances: Vec<Instance> = instances.iter().cloned().collect();
    instances.sort();
    let mut expected = vec![
        Subtype(man, mortal),
        Subtype(list, iterable),
        Subtype(list_man, list_mortal), // Check that a list of men, is a list of mortals
        Subtype(list_man, iterable_man), // Check that a list of men, is an interable of men
        Subtype(iterable_man, iterable_mortal), // Check that an iterable of men, is an iterable of mortals
        Subtype(list_mortal, iterable_mortal), // Check that a list of mortals, is an iterable of mortals
        Subtype(list_man, iterable_mortal), // Check that a list of men, is an iterable of mortals
    ];
    expected.sort();

    assert_eq!(subtypes, expected);

    let mut expected = vec![
        Instance(socretes, man),
        Instance(plato, man),
        Instance(socretes, mortal),
        Instance(plato, mortal),
    ];
    expected.sort();

    assert_eq!(instances, expected);
}