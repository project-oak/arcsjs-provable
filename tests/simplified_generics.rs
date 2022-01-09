use ibis::{apply, ent, facts, ibis, is_a, Ent};
use pretty_assertions::assert_eq;

#[test]
fn list_types_subtype() {
    ibis! {
        Subtype(Ent, Ent);
        Type(Ent);

        Type(t) <- Subtype(t, _);
        Type(t) <- Subtype(_, t);

        Subtype(t, t) <- Type(t);

        Subtype(x, z) <- Subtype(x, y), Subtype(y, z);

        Subtype(
            x,
            y,
        ) <- Type(x),
            Type(y),
            (is_a!(x, ent!("List"))),
            (is_a!(y, ent!("List"))),
            Subtype(
                Ent::by_name(&x.name()[5..x.name().len()-1]),
                Ent::by_name(&y.name()[5..y.name().len()-1])
            );
        plato;
        socretes;
        man;
        mortal;
        List;
    }
    let list_man = apply!(list, man);
    let list_mortal = apply!(list, mortal);

    let mut runtime = Ibis::new();

    // specify all the 'dynamic' facts
    facts!(
        runtime,
        Subtype(man, mortal),
        Type(list_man),
        Type(list_mortal),
        Subtype(plato, man),
        Subtype(socretes, man)
    );

    let (subtypes, _types) = &runtime.run();
    let mut subtypes: Vec<Subtype> = subtypes
        .iter()
        .filter(|Subtype(x, y)| x != y)
        .map(|x| x.clone())
        .collect();
    subtypes.sort();
    let mut expected = vec![
        Subtype(man, mortal),
        Subtype(list_man, list_mortal),
        Subtype(socretes, man),
        Subtype(plato, man),
        Subtype(socretes, mortal),
        Subtype(plato, mortal),
    ];
    expected.sort();
    assert_eq!(subtypes, expected);
}

#[test]
fn iterator_types_subtype() {
    ibis! {
        Subtype(Ent, Ent);
        Type(Ent);
        GenericType(Ent);
        InductiveType(Ent);
        SpecialisationBy(Ent, Ent, Ent);

        Type(t) <- Subtype(t, _);
        Type(t) <- Subtype(_, t);
        Type(t) <- InductiveType(t);
        Type(t) <- GenericType(t);

        Subtype(x, z) <- Subtype(x, y), Subtype(y, z);

        SpecialisationBy(apply!(y, x), y, x) <-
            GenericType(y),
            Type(x),
            Type(apply!(y, x));

        Subtype(x, x) <- Type(x);
        Subtype(x, z) <- Subtype(x, y), Subtype(y, z);

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

    let list_man = apply!(list, man);
    let iterable_man = apply!(iterable, man);
    let list_mortal = apply!(list, mortal);
    let iterable_mortal = apply!(iterable, mortal);

    // specify all the 'dynamic' facts
    facts!(
        runtime,
        Subtype(man, mortal),
        Subtype(list, iterable),
        Subtype(plato, man),
        Subtype(socretes, man),
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

    let (subtypes, _types, _generic_types, _inductive_types, _specialisations_by) = &runtime.run();
    let mut subtypes: Vec<Subtype> = subtypes
        .iter()
        .filter(|Subtype(x, y)| x != y)
        .map(|x| x.clone())
        .collect();
    let mut expected = vec![
        Subtype(man, mortal),
        Subtype(list, iterable),
        Subtype(list_man, list_mortal), // Check that a list of men, is a list of mortals
        Subtype(list_man, iterable_man), // Check that a list of men, is an interable of men
        Subtype(iterable_man, iterable_mortal), // Check that an iterable of men, is an iterable of mortals
        Subtype(list_mortal, iterable_mortal), // Check that a list of mortals, is an iterable of mortals
        Subtype(list_man, iterable_mortal), // Check that a list of men, is an iterable of mortals
        Subtype(socretes, man),
        Subtype(plato, man),
        Subtype(socretes, mortal),
        Subtype(plato, mortal),
    ];

    subtypes.sort();
    expected.sort();

    assert_eq!(subtypes, expected,);
}
