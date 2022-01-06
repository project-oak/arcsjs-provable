use crepe::crepe;
use ibis::{Ent, facts};

crepe! {
    struct Exists(Ent);

    @input
    struct SubtypeClaim(Ent, Ent);
    @output
    struct Subtype(Ent, Ent);
    Subtype(x,y) <- SubtypeClaim(x, y);
    // relation!{Subtype(Ent, Ent)}

    @input
    struct HasTagClaim(Ent, Ent);
    @output
    struct HasTag(Ent, Ent);
    HasTag(x,y) <- HasTagClaim(x, y);

    Exists(x) <- Subtype(x, _);
    Exists(x) <- Subtype(_, x);

    @output
    struct Man(Ent);
    Man(x) <- Subtype(x, Ent::by_name("man")), Subtype(x, Ent::by_name("individual"));

    Man(Ent::by_name(&("Mr. ".to_string()+y.name()))) <- Man(y), (&y.name()[0..4] != "Mr. ");
    // Subtype("man", "mortal") <- (true);

    Subtype(x,x) <- Subtype(x, _);
    Subtype(x,x) <- Subtype(_, x);
    Subtype(x, z) <- Subtype(x, y), Subtype(y, z);
    HasTag(x, z) <- Subtype(x, y), HasTag(y, z);
}

fn main() {
    let mut runtime = Crepe::new();

    let plato = Ent::by_name("plato");
    let individual = Ent::by_name("individual");
    let socretes = Ent::by_name("socretes");
    let man = Ent::by_name("man");
    let mortal = Ent::by_name("mortal");

    // specify all the 'dynamic' facts
    facts!(
        runtime,
        Subtype(plato, individual),
        Subtype(socretes, individual),
        Subtype(plato, man),
        Subtype(socretes, man),
        HasTag(man, mortal)
    );

    let (subtypes, tags, men) = &runtime.run();
    for Subtype(x, y) in subtypes {
        //if *x != "socretes" {
        //continue;
        //}
        println!("{} is a {}", x, y);
    }
    for HasTag(x, y) in tags {
        //if *x != "socretes" {
        //continue;
        //}
        println!("{} has tag {}", x, y);
    }
    for Man(x) in men {
        //if *x != "socretes" {
        //continue;
        //}
        println!("{} 'is a man'", x);
    }
}
