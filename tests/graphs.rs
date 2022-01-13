use ibis::{ent, facts, ibis, Ent, Sol};
use pretty_assertions::assert_eq;

#[test]
fn create_permutations() {
    ibis! {
        Char(char);
        Solution(Ent); // current
        Solution(Ent::by_name(&format!("{}{}", parent.name(), ch))) <-
            Solution(parent),
            Char(ch),
            (!parent.name().contains(ch));
    }

    let mut runtime = Ibis::new();
    facts!(runtime, Char('a'), Char('b'), Char('c'), Solution(ent!("")));

    let (_char, solutions) = runtime.run();
    let mut solutions: Vec<String> = solutions.iter().map(|Solution(ent)| ent.name()).collect();

    let mut expected: Vec<String> = vec![
        "", "a", "b", "c", "ab", "ba", "bc", "cb", "acb", "ac", "ca", "abc", "bac", "bca", "cba",
        "cab",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    solutions.sort();
    expected.sort();
    assert_eq!(solutions, expected);
}

#[test]
fn create_combinations() {
    ibis! {
        Char(char);
        Solution(Ent); // current
        Solution(Ent::by_name(&format!("{}{}", parent.name(), ch))) <-
            Solution(parent),
            Char(ch),
            (parent.name().chars().last() < Some(ch)),
            (!parent.name().contains(ch));
    }

    let mut runtime = Ibis::new();
    facts!(runtime, Char('a'), Char('b'), Char('c'), Solution(ent!("")));

    let (_char, solutions) = runtime.run();
    let mut solutions: Vec<String> = solutions.iter().map(|Solution(ent)| ent.name()).collect();
    solutions.sort();

    let mut expected: Vec<String> = vec!["", "a", "ab", "abc", "ac", "b", "bc", "c"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    expected.sort();
    assert_eq!(solutions, expected);
}

#[test]
fn create_edges() {
    ibis! {
        Node(Ent);
        Solution(Sol); // current
        Solution(parent.add_edge(from, to)) <-
            Solution(parent),
            Node(from),
            Node(to),
            (from != to),
            (!parent.solution().has_edge(from, to));
        Solution(Sol::empty()) <- (true);
    }

    let mut runtime = Ibis::new();

    runtime.add_data(&[
        Node(ent!("a")),
        Node(ent!("b")),
    ]);

    let (_char, solutions) = runtime.run();

    // dbg!(&solutions);

    let mut solutions: Vec<String> = solutions
        .iter()
        .map(|Solution(sol)| format!("{}", sol))
        .collect();
    solutions.sort();

    assert_eq!(
        solutions,
        vec![
            "Sol { {edges}: \"\" }",
            "Sol { {edges}: \"(a, b)\" }",
            "Sol { {edges}: \"(a, b), (b, a)\" }",
            "Sol { {edges}: \"(b, a)\" }",
        ]
    );
}

#[test]
fn create_all_directed_graphs_with_4_nodes() {
    // Useful for performance estimations, not a proper bench mark.
    // Calculates the number of different graphs with 4 nodes
    // 4*3 = 12 possible directed edges (excluding self edges)
    // 2^12 = 4096 graphs
    // Note: Do not try with larger graphs, the |results| are O(2^(n*(n-1))).
    // e.g. for 5 nodes theres over a trillion results to calculate.
    ibis! {
        Node(Ent);
        Solution(Sol); // current
        Solution(parent.add_edge(from, to)) <-
            Solution(parent),
            Node(from),
            Node(to),
            (from != to),
            (!parent.solution().has_edge(from, to));
        Solution(Sol::empty()) <- (true);
    }

    let mut runtime = Ibis::new();

    runtime.add_data(('a'..'e').map(|node| Node(Ent::by_name(&format!("{}", node)))));

    let (_char, solutions) = runtime.run();

    // let solutions: Vec<Sol> = solutions.iter().map(|Solution(sol)| *sol).collect();
    // dbg!(&solutions);
    // let mut solutions: Vec<String> = solutions.iter().map(|sol| format!("{}", sol)).collect();
    // solutions.sort();

    assert_eq!(solutions.len(), 4096);
}
