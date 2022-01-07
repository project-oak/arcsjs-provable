use ibis::{ibis, facts, set, Ent};
use pretty_assertions::assert_eq;

#[test]
fn list_types_subtype() {
    ibis!{
        Subtype(Ent, Ent);
        Instance(Ent, Ent);
        Type(Ent);

        Type(t) <- Subtype(t, _);
        Type(t) <- Subtype(_, t);
        Type(t) <- Instance(_, t);

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
        plato;
        socretes;
        man;
        mortal;
    }
    let list_man = Ent::by_name("List(man)");
    let list_mortal = Ent::by_name("List(mortal)");

    let mut runtime = Ibis::new();

    // specify all the 'dynamic' facts
    facts!(
        runtime,
        Subtype(man, mortal),
        Subtype(list_man, list_man),
        Subtype(list_mortal, list_mortal),
        Instance(plato, man),
        Instance(socretes, man)
    );

    let (subtypes, instances, _types) = &runtime.run();
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
    ibis! {
        Subtype(Ent, Ent);
        Type(Ent);
        GenericType(Ent);
        InductiveType(Ent);

        Type(t) <- Subtype(t, _);
        Type(t) <- Subtype(_, t);
        Type(t) <- Instance(_, t);
        Type(t) <- InductiveType(t);
        Type(t) <- GenericType(t);

        SpecialisationOf(Ent, Ent);

        SpecialisationOf(x, y) <- SpecialisationBy(x, y, _);

        SpecialisationOf(x, y) <-
            GenericType(y),
            Type(x),
            (x.name().starts_with(&(y.name()+"("))),
            (x.name().ends_with(")"));

        SpecialisationBy(Ent, Ent, Ent);

        SpecialisationBy(Ent::by_name(&format!("{}({})", y.name(), x.name())), y, x) <-
            GenericType(y),
            Type(x),
            Type(Ent::by_name(&format!("{}({})", y.name(), x.name())));

        Instance(Ent, Ent);

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
        plato;
        socretes;
        man;
        mortal;
        List;
        Iterable;
    }

    let mut runtime = Ibis::new();

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

    let (subtypes, _types, _generic_types, _inductive_types, _specialisations_of, _specialisations_by, instances) = &runtime.run();
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
