use ibis::{facts, ibis, Ent, Sol, ent};
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
    facts!(runtime,
        Char('a'),
        Char('b'),
        Char('c'),
        Solution(ent!(""))
    );

    let (_char, solutions,) = runtime.run();
    let mut solutions: Vec<String> = solutions.iter().map(|Solution(ent)| ent.name()).collect();

    let mut expected: Vec<String> = vec!{
        "",
        "a",
        "b",
        "c",
        "ab",
        "ba",
        "bc",
        "cb",
        "acb",
        "ac",
        "ca",
        "abc",
        "bac",
        "bca",
        "cba",
        "cab",
    }.iter().map(|s| s.to_string()).collect();

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
    facts!(runtime,
        Char('a'),
        Char('b'),
        Char('c'),
        Solution(ent!(""))
    );


    let (_char, solutions,) = runtime.run();
    let mut solutions: Vec<String> = solutions.iter().map(|Solution(ent)| ent.name()).collect();
    solutions.sort();

    let mut expected: Vec<String> = vec!{
        "",
        "a",
        "ab",
        "abc",
        "ac",
        "b",
        "bc",
        "c",
    }.iter().map(|s| s.to_string()).collect();

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
    }

    let mut runtime = Ibis::new();
    facts!(runtime,
        Node(ent!("a")),
        Node(ent!("b")),
        Solution(Sol::empty()),
    );


    let (_char, solutions,) = runtime.run();

    // dbg!(&solutions);

    let mut solutions: Vec<String> = solutions.iter().map(|Solution(sol)| format!("{}", sol)).collect();
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
