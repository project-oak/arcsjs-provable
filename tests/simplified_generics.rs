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
