use ibis::{ibis, Ent, IbisError};

fn main() -> Result<(), IbisError> {
    ibis! {
        Solution(Sol);
        Type(Ent); // type
        Node(Ent, Ent, Ent); // particle-identifier, identifier, type
        Claim(Ent, Ent); // identifier, tag
        Check(Ent, Ent); // identifier, tag
        HasTag(Sol, Ent, Ent, Ent); // sol, source node, node, tag
        LessPrivateThan(Ent, Ent); // tag, tag
        Leak(Sol, Ent, Ent, Ent, Ent); // sol, node, expected_tag, source, tag2
        TypeError(Sol, Ent, Ent, Ent, Ent); // sol, node, ty, source, ty
        Subtype(Ent, Ent); // sub, super
        TrustedToRemoveTag(Ent, Ent); // Node, Tag that it can remove
        Edge(Sol, Ent, Ent, Ent, Ent); // sol, from_p, from, to_p, to

        Edge(sol, from_particle, from, to_particle, to) <- Solution(sol), Node(from_particle, from, _), Node(to_particle, to, _), (sol.has_edge(from, to));

        Solution(parent.add_edge(from, to)) <-
            Solution(parent),
            Node(from_particle, from, from_type),
            Node(to_particle, to, to_type),
            Subtype(from_type, to_type),
            (from != to),
            (!parent.has_edge(from, to));
        HasTag(s, n, n, tag) <- Solution(s), Claim(n, tag);
        HasTag(s, source, down, tag) <- HasTag(s, source, curr, tag), Edge(s, _curr_particle, curr, _down_particle, down), !TrustedToRemoveTag(down, tag); // Propagate 'downstream'.

        Leak(s, n, t1, source, t2) <-
            LessPrivateThan(t1, t2),
            Check(n, t1),
            HasTag(s, source, n, t2); // Check failed, node has a 'more private' tag i.e. is leaking.

        TypeError(s, from, from_ty, to, to_ty) <-
            Edge(s, _from_p, from, _to_p, to),
            Node(_from_p, from, from_ty),
            Node(_to_p, to, to_ty),
            !Subtype(from_ty, to_ty); // Check failed, from writes an incompatible type into to

        Subtype(x, x) <- Type(x);
        Subtype(x, z) <- Subtype(x, y), Subtype(y, z);

        Number;
        Int;
        String;
        number_or_string;
        Serializable;

        a;
        b;
        c;
        d;
        e;

        p_a;
        p_b;
        p_c;
        p_de;

        private;
        public;
    }

    let mut runtime = Ibis::new();

    let data = include_str!("../../demo.json");
    let recipe: Recipe = serde_json::from_str(data).expect("JSON Error?");

    runtime.add_recipe(recipe);

    eprintln!("Preparing graph...");
    let solutions = runtime.extract_solutions();
    eprintln!("Done");
    println!("{}", solutions.to_dot());
    Ok(())
}

/*
#[test]
fn demo_json_round_trips() {
    let data = include_str!("../../demo.json");
    let recipe: Recipe = serde_json::from_str(data).expect("JSON Error?");

    let serialized = serde_json::to_string(&recipe).unwrap();
    let deserialized: Recipe = serde_json::from_str(&serialized).unwrap();

}*/
