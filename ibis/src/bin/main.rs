use ibis::IbisError;

use ibis::{Sol, Ent, ibis};

fn main() -> Result<(), IbisError> {
    ibis! {
        Type(Ent); // type
        Node(Ent, Ent); // identifier, type
        Claim(Ent, Ent); // identifier, tag
        Check(Ent, Ent); // identifier, tag
        HasTag(Sol, Ent, Ent, Ent); // sol, source node, node, tag
        LessPrivateThan(Ent, Ent); // tag, tag

        Leak(Sol, Ent, Ent, Ent, Ent); // sol, node, expected_tag, source, tag2

        Subtype(Ent, Ent); // sub, super

        TrustedWithTag(Ent, Ent); // Node, Tag that it can remove

        Edge(Sol, Ent, Ent);
        Edge(sol, from, to) <- Solution(sol), Node(from, _), Node(to, _), (sol.has_edge(from, to));

        Solution(Sol); // current
        Solution(parent.add_edge(from, to)) <-
            Solution(parent),
            Node(from, from_type),
            Node(to, to_type),
            Subtype(from_type, to_type),
            (from != to),
            (!parent.has_edge(from, to));
        Solution(Sol::empty()) <- (true);

        HasTag(s, n, n, tag) <- Solution(s), Claim(n, tag);
        HasTag(s, source, down, tag) <- HasTag(s, source, curr, tag), !TrustedWithTag(curr, tag), Edge(s, curr, down); // Propagate 'downstream'.

        Leak(s, n, t1, source, t2) <-
            LessPrivateThan(t1, t2),
            Check(n, t1),
            HasTag(s, source, n, t2); // Check failed, node has a 'more private' tag i.e. is leaking.

        Subtype(x, x) <- Type(x);
        Subtype(x, z) <- Subtype(x, y), Subtype(y, z);

        LessPrivateThan(Ent::by_name("public"), Ent::by_name("private")) <- (true);

        TrustedWithTag(Ent::by_name("b"), Ent::by_name("private")) <- (true);

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

        private;
        public;
    }

    let mut runtime = Ibis::new();

    runtime.add_data(&[
        Type(number),
        Type(int),
        Type(string),
        Type(serializable),
        Type(number_or_string),
    ]);

    runtime.add_data(&[
        // int = 'int' & number & serializable
        Subtype(int, number),
        Subtype(int, serializable),
        // string = 'string' & serializable
        Subtype(string, serializable),
        // number_or_string = number | string
        Subtype(number, number_or_string),
        Subtype(string, number_or_string),
    ]);

    runtime.add_data(&[
        Node(a, int),
        Node(b, number),
        Node(c, string),
        Node(d, serializable),
        Node(e, number_or_string),
    ]);
    runtime.add_data(&[Claim(a, private)]);
    runtime.add_data(&[
        Check(e, public), // exfiltration
        Check(d, public), // exfiltration
    ]);

    eprintln!("Preparing graph...");
    let g = runtime.solve_graph();
    eprintln!("Done");
    println!("{}", g.to_dot());
    Ok(())
}
