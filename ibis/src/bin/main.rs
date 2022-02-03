use ibis::{IbisError, ibis, Ent};

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
        Subtype(Ent, Ent); // sub, super
        TrustedWithTag(Ent, Ent); // Node, Tag that it can remove
        Edge(Sol, Ent, Ent, Ent, Ent);

        Edge(sol, from_particle, from, to_particle, to) <- Solution(sol), Node(from_particle, from, _), Node(to_particle, to, _), (sol.has_edge(from, to));

        Solution(parent.add_edge(from, to)) <-
            Solution(parent),
            Node(_pfrom, from, from_type),
            Node(_pto, to, to_type),
            Subtype(from_type, to_type),
            (from != to),
            (!parent.has_edge(from, to));
        Solution(Sol::empty()) <- (true); // By default, seed the solution graph with an empty solution
        // TODO: Replace this with the 'current' state

        HasTag(s, n, n, tag) <- Solution(s), Claim(n, tag);
        HasTag(s, source, down, tag) <- HasTag(s, source, curr, tag), Edge(s, _curr_particle, curr, _down_particle, down), !TrustedWithTag(down, tag); // Propagate 'downstream'.

        Leak(s, n, t1, source, t2) <-
            LessPrivateThan(t1, t2),
            Check(n, t1),
            HasTag(s, source, n, t2); // Check failed, node has a 'more private' tag i.e. is leaking.

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

    // use serde::{Deserialize, Serialize};
    #[derive(Default, Debug, Serialize, Deserialize)]
    #[serde(default, deny_unknown_fields)]
    struct Recipe {
        metadata: serde_json::Value,
        types: Vec<Type>,
        subtypes: Vec<Subtype>,
        nodes: Vec<Node>,
        claims: Vec<Claim>,
        checks: Vec<Check>,
        less_private_than: Vec<LessPrivateThan>,
        trusted_with_tag: Vec<TrustedWithTag>,
    }

    let mut runtime = Ibis::new();

    let data = include_str!("../../demo.json");
    let recipe: Recipe = serde_json::from_str(data).expect("JSON Error?");

    dbg!(&recipe.metadata);

    runtime.add_data(recipe.types);
    runtime.add_data(recipe.subtypes);
    runtime.add_data(recipe.nodes);
    runtime.add_data(recipe.claims);
    runtime.add_data(recipe.checks);
    runtime.add_data(recipe.less_private_than);
    runtime.add_data(recipe.trusted_with_tag);

    eprintln!("Preparing graph...");
    let g = runtime.solve_graph();
    eprintln!("Done");
    println!("{}", g.to_dot());
    Ok(())
}
