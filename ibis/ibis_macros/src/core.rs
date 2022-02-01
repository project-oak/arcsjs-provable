use ibis::Sol;
use std::collections::HashMap;

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

#[derive(Default)]
struct DotGraph {
    nodes: Vec<String>,
    edges: Vec<(String, String, Vec<String>)>,
    children: Vec<(String, String, DotGraph)>,
}

impl DotGraph {
    fn add_node(&mut self, node: String) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, from: String, to: String, attrs: Vec<String>) {
        self.edges.push((from, to, attrs));
    }

    fn add_child(&mut self, name: String, label: String, child: DotGraph) {
        self.children.push((name, label, child));
    }

    fn to_dot(self) -> String {
        format!("digraph solutions {{ {} }}", self.to_dot_items())
    }

    fn to_dot_items(self) -> String {
        let mut items: Vec<String> = vec![];

        for node in self.nodes {
            items.push(node+";");
        }

        for edge in self.edges {
            items.push(format!("{} -> {}[{}];", edge.0, edge.1, edge.2.join(" ")));
        }
        for (name, label, child) in self.children {
            items.push(format!("subgraph cluster_{name} {{ {} color=\"#00000070\"; label=\"{label}\"}}", child.to_dot_items(), name=name, label=label));
        }
        items.join("")
    }
}

impl Crepe {
    // TODO: Remove clone requirement here
    fn add_data<T: ToInput, Iter: IntoIterator<Item=T>>(&mut self, data: Iter) where Crepe: Extend<T::U> {
        self.extend(data.into_iter().map(|datum|datum.to_claim()));
    }

    fn solve_graph(self) -> DotGraph {
        let (solutions, _type, nodes, _claim, _check, has_tags, _lpt, leak, _subtype, trusted_withs, edges) = self.run();


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

        // dbg!(&leak, &has_tags, &trusted_withs);
        let solutions: Vec<Sol> = solutions.iter().map(|Solution(sol)| *sol).collect();

        let mut g = DotGraph::default();

        let mut max = 0;
        let mut best = None;
        for s in &solutions {
            let l = s.edges().len();
            if l > max {
                best = Some(*s);
                max = l;
            }
        }
        // let solutions = best;
        for s in &solutions {
            let particle_id = |particle| format!("s{}_{}", &s.id, particle);
            let node_id = |node| format!("s{}_{}", &s.id, node);
            let mut sol_graph = DotGraph::default();
            let mut particles = HashMap::new();
            for Node(particle, node, ty) in &nodes {
                let mut extras: Vec<String> = vec![];
                for HasTag(hts, source, sink, tag) in &has_tags {
                    if hts == s && sink == node {
                        extras.push(format!("\\n'{}' from {}", tag, source));
                    }
                }
                for TrustedWithTag(trusted_n, tag) in &trusted_withs {
                    if trusted_n == node {
                        extras.push(format!("\\n trusted to remove tag '{}'", tag));
                    }
                }
                let mut particle_g = particles.entry(particle).or_insert(DotGraph::default());
                particle_g.add_node(format!("{node_id} [shape=record label=\"{node} : {ty}{extras}\"]", node_id=node_id(node), node=node, ty=ty, extras=extras.join("")));
            }
            for (particle, mut particle_g) in particles {
                sol_graph.add_child(particle_id(particle), format!("{} : Particle", particle), particle_g);
            }

            for Edge(es, from_particle, from_id, to_particle, to_id) in &edges {
                if es != s {
                    continue;
                }
                let from = format!("{}:s", node_id(from_id));
                let to = format!("{}:n", node_id(to_id));
                sol_graph.add_edge(
                    from.clone(),
                    to.clone(),
                    vec![]
                );
            }
            g.add_child(format!("sol_{}", &s.id), format!("Solution {}", &s.id), sol_graph);
        }
        g
    }
}
