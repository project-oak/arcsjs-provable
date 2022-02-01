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
        let (solutions, _type, nodes, _claim, _check, has_tags, _lpt, leak, _subtype, trusted_withs, _edge) = self.run();


        // Solution(Sol);
        // Type(Ent); // type
        // Node(Ent, Ent); // identifier, type
        // Claim(Ent, Ent); // identifier, tag
        // Check(Ent, Ent); // identifier, tag
        // HasTag(Sol, Ent, Ent, Ent); // sol, source node, node, tag
        // LessPrivateThan(Ent, Ent); // tag, tag
        // Leak(Sol, Ent, Ent, Ent, Ent); // sol, node, expected_tag, source, tag2
        // Subtype(Ent, Ent); // sub, super
        // TrustedWithTag(Ent, Ent); // Node, Tag that it can remove
        // Edge(Sol, Ent, Ent);

        dbg!(&leak, &has_tags, &trusted_withs);
        let solutions: Vec<Sol> = solutions.iter().map(|Solution(sol)| *sol).collect();

        let mut g = DotGraph::default();

        let mut max = 0;
        let mut best = None;
        for s in &solutions {
            let l = s.edges().len();
            if l > max {
                best = Some(s);
                max = l;
            }
        }
        let solutions = best;
        for s in &solutions {
            g.nodes.push(
                format!("s{} [label=\"{}\" shape=ellipse]", &s.id, &s.id)
            );
            for node in &nodes {
                let n = format!("s{}_{}", &s.id, node.0);
                let mut extras: Vec<String> = vec![];
                for has_tag in &has_tags {
                    if has_tag.0 == **s && has_tag.2 == node.0 {
                        extras.push(format!("\\n'{}' from {}", has_tag.3, has_tag.1));
                    }
                }
                g.nodes.push(
                    format!("{} [label=\"{} : {}{}\" shape=record]", n, node.0, node.1, extras.join(""))
                );
                g.edges.push(
                    (
                        format!("s{}", &s.id),
                        n,
                        vec!["color=grey arrowhead=none style=dashed".to_string()] //format!("label=\"s{}\"", &s.id)]
                    )
                );
            }
            for (from_id, to_id) in s.edges() {
                let to = format!("s{}_{}", &s.id, to_id);
                let from = format!("s{}_{}", &s.id, from_id);
                g.edges.push(
                    (
                        from.clone(),
                        to.clone(),
                        vec![] //format!("label=\"s{}\"", &s.id)]
                    )
                );
            }
        }
        g
    }
}
