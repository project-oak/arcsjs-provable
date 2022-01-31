use ibis::Sol;

type Ibis=Crepe;

pub trait ToInput {
    type U;
    fn to_claim(self) -> Self::U;
}

impl <T: ToInput + Clone> ToInput for &T {
    type U = T::U;

    fn to_claim(self) -> Self::U {
        self.clone().to_claim()
    }
}

struct DotGraph {
    nodes: Vec<String>,
    edges: Vec<(String, String, Vec<String>)>,
}

impl Default for DotGraph {
    fn default() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
        }
    }
}

impl DotGraph {
    fn to_dot(self) -> String {
        let mut items: Vec<String> = vec![];

        for node in self.nodes {
            items.push(node);
        }

        for edge in self.edges {
            let attrs: Vec<String> = edge.2.iter().map(|attr|format!("[{}]", attr)).collect();
            items.push(format!("{} -> {}{}", edge.0, edge.1, attrs.join("")));
        }
        format!("digraph name {{ {} }}", items.join("; "))
    }
}

impl Crepe {
    // TODO: Remove clone requirement here
    fn add_data<T: ToInput, Iter: IntoIterator<Item=T>>(&mut self, data: Iter) where Crepe: Extend<T::U> {
        self.extend(data.into_iter().map(|datum|datum.to_claim()));
    }

    fn solve_graph(self) -> DotGraph {
        let results = self.run();
        let solutions: Vec<Sol> = results.0.iter().map(|Solution(sol)| *sol).collect();

        dbg!(&solutions);

        let mut g = DotGraph::default();

        for s in &solutions {
            for (to_id, from_id) in s.edges() {
                let to = format!("s{}_{}", &s.id, to_id);
                let from = format!("s{}_{}", &s.id, from_id);
                g.nodes.push(
                    format!("{} [label=\"{}\"]", &to, to_id)
                );
                g.nodes.push(
                    format!("{} [label=\"{}\"]", &from, from_id)
                );
                g.edges.push(
                    (
                        from,
                        to,
                        vec![format!("label=\"s{}\"", &s.id)]
                    )
                );
            }
        }
        g
    }
}
