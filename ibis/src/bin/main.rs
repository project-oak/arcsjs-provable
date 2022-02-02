use ibis::{IbisError, ibis, Ent};
use serde_json::Value;

fn get_array<T: Sized, F: Fn(&Value) -> T>(value: &Value, func: &F) -> Vec<T> {
    match value {
        Value::Array(value) => value.iter().map(func).collect(),
        _ => {
            eprintln!("Expected array of relations, found {:?}", value);
            vec![]
        }
    }
}

fn get_str(value: &Value) -> &str {
    match value {
        Value::String(value) => value,
        _ => panic!("Expected string, found {:?}", value),
    }
}

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

    let mut runtime = Ibis::new();

    let data = include_str!("../../demo.json");
    let v: Value = serde_json::from_str(data).expect("JSON Error?");
    let relations = match v {
        Value::Object(relations) => relations,
        _ => panic!("Expected object, found {:?}", v),
    };
    for (relation_name, values) in relations {
        match relation_name.as_str() {
            "types" => {
                let values = get_array(&values, &|typename| {
                    let typename = get_str(typename);
                    Type(Ent::by_name(typename))
                });
                runtime.add_data(values);
            }
            "subtypes" => {
                let values = get_array(&values, &|subtype| {
                    let values: Vec<Ent> = get_array(subtype, &|s| Ent::by_name(get_str(s)));
                    if let [sub, sup] = &values[..] {
                        Subtype(*sub, *sup)
                    } else {
                        panic!("Expected 2-tuple for subtypes, found {:?}", subtype);
                    }
                });
                runtime.add_data(values);
            }
            "nodes" => {
                let values = get_array(&values, &|node| {
                    let values: Vec<Ent> = get_array(node, &|s| Ent::by_name(get_str(s)));
                    if let [particle, node, ty] = &values[..] {
                        Node(*particle, *node, *ty)
                    } else {
                        panic!("Expected 3-tuple for node, found {:?}", node);
                    }
                });
                runtime.add_data(values);
            }
            "claims" => {
                let values = get_array(&values, &|claim| {
                    let values: Vec<Ent> = get_array(claim, &|s| Ent::by_name(get_str(s)));
                    if let [node, label] = &values[..] {
                        Claim(*node, *label)
                    } else {
                        panic!("Expected 2-tuple for claim, found {:?}", claim);
                    }
                });
                runtime.add_data(values);
            }
            "checks" => {
                let values = get_array(&values, &|check| {
                    let values: Vec<Ent> = get_array(check, &|s| Ent::by_name(get_str(s)));
                    if let [node, label] = &values[..] {
                        Check(*node, *label)
                    } else {
                        panic!("Expected 2-tuple for check, found {:?}", check);
                    }
                });
                runtime.add_data(values);
            }
            "less_private_than" => {
                let values = get_array(&values, &|less_private_than| {
                    let values: Vec<Ent> = get_array(less_private_than, &|s| Ent::by_name(get_str(s)));
                    if let [sublabel, suplabel] = &values[..] {
                        LessPrivateThan(*sublabel, *suplabel)
                    } else {
                        panic!("Expected 2-tuple for less_private_than, found {:?}", less_private_than);
                    }
                });
                runtime.add_data(values);
            }
            "trusted_with_tag" => {
                let values = get_array(&values, &|trusted_with| {
                    let values: Vec<Ent> = get_array(trusted_with, &|s| Ent::by_name(get_str(s)));
                    if let [node, label] = &values[..] {
                        TrustedWithTag(*node, *label)
                    } else {
                        panic!("Expected 2-tuple for trusted_with, found {:?}", trusted_with);
                    }
                });
                runtime.add_data(values);
            }
            _ => eprintln!("Unknown relation named {:?}", relation_name),
        }
    }

    eprintln!("Preparing graph...");
    let g = runtime.solve_graph();
    eprintln!("Done");
    println!("{}", g.to_dot());
    Ok(())
}
