#[derive(Default)]
pub struct DotGraph {
    nodes: Vec<String>,
    edges: Vec<(String, String, Vec<String>)>,
    children: Vec<(String, String, DotGraph)>,
}

impl DotGraph {
    pub fn add_node(&mut self, node: String) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, from: String, to: String, attrs: Vec<String>) {
        self.edges.push((from, to, attrs));
    }

    pub fn add_child(&mut self, name: String, label: String, child: DotGraph) {
        self.children.push((name, label, child));
    }

    pub fn to_dot(self) -> String {
        format!(
            "digraph solutions {{compound=true; {} }}",
            self.to_dot_items()
        )
    }

    pub fn to_dot_items(self) -> String {
        let mut items: Vec<String> = vec![];

        for node in self.nodes {
            items.push(node + ";");
        }

        for edge in self.edges {
            items.push(format!("{} -> {}[{}];", edge.0, edge.1, edge.2.join(" ")));
        }
        for (name, label, child) in self.children {
            items.push(format!(
                "subgraph cluster_{name} {{ {} color=\"#00000070\"; label=\"{label}\"}}",
                child.to_dot_items(),
                name = name,
                label = label
            ));
        }
        items.join("")
    }
}
