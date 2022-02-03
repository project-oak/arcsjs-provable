// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use ibis::{ibis, Ent, Sol};
use pretty_assertions::assert_eq;

#[test]
fn create_type_checked_graphs() {
    ibis! {
        Type(Ent); // type
        Node(Ent, Ent); // identifier, type
        Subtype(Ent, Ent); // sub, super

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

        private;
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
        Subtype(int, number),
        Subtype(int, serializable),
        Subtype(string, serializable),
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

    let (_types, _nodes, _subtypes, _edge, solutions) = runtime.run();

    let solutions: Vec<Sol> = solutions.iter().map(|Solution(sol)| *sol).collect();
    let mut max = 0;
    for sol in &solutions {
        let len = sol.edges().len();
        if len > max {
            max = len;
        }
    }
    assert_eq!(solutions.len(), 64);
    let solutions: Vec<&Sol> = solutions
        .iter()
        .filter(|sol| sol.edges().len() == max)
        .collect();
    assert_eq!(solutions.len(), 1);
    let best = solutions[0];
    let edges: Vec<(Ent, Ent)> = best.edges().iter().map(|edge| *edge).collect();
    assert_eq!(edges, vec![(a, b), (a, d), (a, e), (b, e), (c, d), (c, e)]);
}

#[test]
fn create_tagged_type_checked_graphs() {
    ibis! {
        Type(Ent); // type
        Node(Ent, Ent); // identifier, type
        Claim(Ent, Ent); // identifier, tag
        Check(Ent, Ent); // identifier, tag
        HasTag(Sol, Ent, Ent, Ent); // sol, source node, node, tag
        LessPrivateThan(Ent, Ent); // tag, tag

        Leak(Sol, Ent, Ent, Ent, Ent); // sol, node, expected_tag, source, tag2

        Subtype(Ent, Ent); // sub, super

        TrustedToRemoveTag(Ent, Ent); // Node, Tag that it can remove

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
        HasTag(s, source, down, tag) <- HasTag(s, source, curr, tag), !TrustedToRemoveTag(curr, tag), Edge(s, curr, down); // Propagate 'downstream'.

        Leak(s, n, t1, source, t2) <-
            LessPrivateThan(t1, t2),
            Check(n, t1),
            HasTag(s, source, n, t2); // Check failed, node has a 'more private' tag i.e. is leaking.

        Subtype(x, x) <- Type(x);
        Subtype(x, z) <- Subtype(x, y), Subtype(y, z);

        LessPrivateThan(Ent::by_name("public"), Ent::by_name("private")) <- (true);

        TrustedToRemoveTag(Ent::by_name("b"), Ent::by_name("private")) <- (true);

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

    let (
        _types,
        _nodes,
        _claims,
        _checks,
        _has_tags,
        _conflicting,
        _leaks,
        _subtypes,
        _trusted_to_remove_tag,
        _edge,
        solutions,
    ) = runtime.run();

    let solutions: Vec<Sol> = solutions.iter().map(|Solution(sol)| *sol).collect();
    // assert_eq!(solutions.len(), 1);
    //let _has_tags: Vec<&HasTag> = _has_tags.iter().filter(|HasTag(s, _source, _node, _tag)| s == best).collect();
    //let _leaks: Vec<&Leak> = _leaks.iter().filter(|Leak(s, _node, _expected, _source, _tag2)| s == best).collect();
    let mut leak_map: HashMap<Sol, Vec<Leak>> = HashMap::new();
    for sol in &solutions {
        leak_map.insert(*sol, Vec::new());
    }
    for leak in _leaks {
        leak_map
            .get_mut(&leak.0)
            .expect("every solution should have a leak set")
            .push(leak);
    }

    // dbg!(&_leaks);

    let filtered_sols: Vec<Sol> = solutions
        .iter()
        .filter(|s| leak_map.get(&s).unwrap().len() == 0)
        .cloned()
        .collect();

    let mut max = 0;
    for sol in &filtered_sols {
        let len = sol.edges().len();
        if len > max {
            max = len;
        }
    }
    // assert_eq!(solutions.len(), 64);
    let filtered_sols: Vec<&Sol> = filtered_sols
        .iter()
        .filter(|sol| sol.edges().len() == max)
        .collect();
    dbg!(filtered_sols);
    // dbg!(_has_tags);

    // let best = filtered_sols[0];
    // let edges: Vec<(Ent, Ent)> = best.edges().iter().map(|edge| *edge).collect();
    // assert_eq!(edges, vec![(a, b), (a, d), (a, e), (b, e), (c, d), (c, e)]);
}
