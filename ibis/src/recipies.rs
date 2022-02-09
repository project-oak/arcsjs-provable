use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::dot::*;
use crate::{ibis, Ent, Sol, ToInput, SolutionData};

ibis! {
    Solution(Sol);
    Type(Ent); // type
    LessPrivateThan(Ent, Ent); // tag, tag
    Subtype(Ent, Ent); // sub, super

    Node(Sol, Ent, Ent, Ent); // sol, particle-identifier, identifier, type
    Claim(Sol, Ent, Ent); // sol, identifier, tag
    Check(Sol, Ent, Ent); // sol, identifier, tag
    TrustedToRemoveTag(Sol, Ent, Ent); // sol, Node, Tag that it can remove

    // Feedback
    HasTag(Sol, Ent, Ent, Ent); // solution, source node, node with tag, tag
    Leak(Sol, Ent, Ent, Ent, Ent); // sol, node, expected_tag, source, tag2
    TypeError(Sol, Ent, Ent, Ent, Ent); // sol, node, ty, source, ty

    Solution(parent.add_edge(from, to)) <-
        Solution(parent),
        Node(parent, from_particle, from, from_type),
        Node(parent, to_particle, to, to_type),
        Subtype(from_type, to_type),
        (from != to),
        (!parent.has_edge(from, to));
    HasTag(s, n, n, tag) <- Solution(s), Claim(s, n, tag);
    HasTag(s, source, down, tag) <-
        Solution(s),
        Node(s, _curr_particle, curr, _),
        Node(s, _down_particle, down, _),
        HasTag(s, source, curr, tag),
        (s.has_edge(curr, down)),
        !TrustedToRemoveTag(s, down, tag); // Propagate 'downstream'.

    Leak(s, n, t1, source, t2) <-
        LessPrivateThan(t1, t2),
        Check(s, n, t1),
        HasTag(s, source, n, t2); // Check failed, node has a 'more private' tag i.e. is leaking.

    TypeError(s, from, from_ty, to, to_ty) <-
        Node(s, _from_p, from, from_ty),
        Node(s, _to_p, to, to_ty),
        (s.has_edge(from, to)),
        !Subtype(from_ty, to_ty); // Check failed, from writes an incompatible type into to

    Subtype(x, x) <- Type(x);
    Subtype(x, z) <- Subtype(x, y), Subtype(y, z)
}


#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    metadata: serde_json::Value,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    types: Vec<Type>,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    subtypes: Vec<Subtype>,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    less_private_than: Vec<LessPrivateThan>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Feedback {
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    leaks: Vec<Leak>,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    type_errors: Vec<TypeError>,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    has_tags: Vec<HasTag>,
}

fn starting_recipies() -> Vec<Recipe> {
    vec![Recipe::default()]
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Ibis {
    #[serde(flatten)]
    config: Config,
    #[serde(default = "starting_recipies", skip_serializing_if="Vec::is_empty")]
    recipies: Vec<Recipe>,
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Recipe {
    #[serde(default)]
    metadata: serde_json::Value,
    #[serde(skip, default)]
    id: Option<Sol>,
    // Do not deserialize the feedback on a recipe: Re-generate it each time for consistency.
    #[serde(skip_deserializing, flatten)]
    feedback: Option<Feedback>,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    nodes: Vec<Node>,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    claims: Vec<Claim>,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    checks: Vec<Check>,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    trusted_to_remove_tag: Vec<TrustedToRemoveTag>,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    edges: Vec<(Ent, Ent)>,
    #[cfg(feature = "ancestors")]
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    ancestors: Vec<Sol>,
}

impl Recipe {
    pub fn from_sol(sol: Sol) -> Self {
        let solution = sol.solution();
        Recipe {
            id: Some(sol),
            feedback: None,
            metadata: serde_json::Value::Null,
            nodes: solution.nodes.iter().map(|node|{
                let particle = solution.node_to_particle.get(node).unwrap();
                let ty = solution.node_types.get(node).unwrap();
                Node(sol, *particle, *node, *ty)
            }).collect(),
            claims: solution.claims.iter().map(|(node, tag)|{
                Claim(sol, *node, *tag)
            }).collect(),
            checks: solution.checks.iter().map(|(node, tag)|{
                Check(sol, *node, *tag)
            }).collect(),
            trusted_to_remove_tag: solution.trusted_to_remove_tag.iter().map(|(node, tag)|{
                TrustedToRemoveTag(sol, *node, *tag)
            }).collect(),
            edges: solution.edges.iter().cloned().collect(),
        }
    }

    pub fn with_feedback(mut self, feedback: Feedback) -> Self {
        self.feedback = Some(feedback);
        self
    }
}

fn sol_id(sol: &Sol) -> String {
    format!("sol_{}", &sol.id)
}

impl Ibis {

    pub fn to_dot(self: &Self) -> String {
        self.to_dot_repr().to_dot()
    }

    fn to_dot_repr(self: &Self) -> DotGraph {
        let mut g = DotGraph::default();

        let solutions = if true {
            self.recipies.iter().collect()
        } else {
            let mut max = 0;
            let mut best = None;
            for s in &self.recipies {
                let l = s.edges.len();
                if l > max {
                    best = Some(s);
                    max = l;
                }
            }
            vec![best.expect("Expected a 'best' solution")]
        };
        for s in solutions {
            let sol = &s.id.expect("Every recipe should have an id?");
            let s_id = sol_id(sol);
            let sol_graph = s.to_dot_repr();
            g.add_child(s_id.clone(), format!("Solution {}", &sol.id), sol_graph);
            #[cfg(feature = "ancestors")]
            {
                let solution_head = |sol| format!("{}_head", sol_id(sol));
                sol_graph.add_node(format!("{}[style=invis height = 0 width = 0 label=\"\"]", solution_head(s)));
                for ancestor in &s.ancestors() {
                    g.add_edge(solution_head(&s), solution_head(ancestor), vec![format!("ltail=cluster_{} lhead=cluster_{}", &s_id, sol_id(ancestor))]);
                }
            }
        }
        g
    }
}

impl From<&Recipe> for Sol {
    fn from(recipe: &Recipe) -> Self {
        // Convert the recipe to its 'solution data'
        let solution = SolutionData::from(recipe);
        // Get an id to represent that.
        let sol = Sol::new_blocking(solution);
        sol
    }
}

impl From<&Recipe> for SolutionData {
    fn from(recipe: &Recipe) -> Self {
        Self {
            edges: recipe.edges.iter().cloned().collect(),
            checks: recipe.checks.iter().map(|Check(_sol, node, tag)|(*node, *tag)).collect(),
            claims: recipe.claims.iter().map(|Claim(_sol, node, tag)|(*node, *tag)).collect(),
            node_to_particle: recipe.nodes.iter().map(|Node(_sol, particle, node, _ty)|(*node, *particle)).collect(),
            node_types: recipe.nodes.iter().map(|Node(_sol, _particle, node, ty)|(*node, *ty)).collect(),
            nodes: recipe.nodes.iter().map(|Node(_sol, _particle, node, _ty)|*node).collect(),
            trusted_to_remove_tag: recipe.trusted_to_remove_tag.iter().map(|TrustedToRemoveTag(_sol, node, tag)|(*node, *tag)).collect(),
        }
    }
}

impl From<Sol> for Recipe {
    fn from(sol: Sol) -> Self {
        let solution = sol.solution();
        Recipe {
            id: Some(sol),
            feedback: None,
            metadata: serde_json::Value::Null,
            nodes: solution.nodes.iter().map(|node|{
                let particle = solution.node_to_particle.get(node).unwrap();
                let ty = solution.node_types.get(node).unwrap();
                Node(sol, *particle, *node, *ty)
            }).collect(),
            claims: solution.claims.iter().map(|(node, tag)|{
                Claim(sol, *node, *tag)
            }).collect(),
            checks: solution.checks.iter().map(|(node, tag)|{
                Check(sol, *node, *tag)
            }).collect(),
            trusted_to_remove_tag: solution.trusted_to_remove_tag.iter().map(|(node, tag)|{
                TrustedToRemoveTag(sol, *node, *tag)
            }).collect(),
            edges: solution.edges.iter().cloned().collect(),
        }
    }
}
impl From<SolutionData> for Recipe {
    fn from(solution: SolutionData) -> Self {
        // Get an id for the solution data.
        let sol = Sol::new_blocking(solution);
        // Convert that id and solution data to a recipe.
        Recipe::from(sol)
    }
}


impl Recipe {
    fn to_dot_repr(self: &Self) -> DotGraph {
        let sol = &self.id.expect("Every recipe should have an id?");
        let s_id = sol_id(sol);
        let particle_id = |particle| format!("{}_p{}", &s_id, particle);
        let node_id = |node| format!("{}_h{}", &s_id, node);
        let mut sol_graph = DotGraph::default();
        let mut particles = HashMap::new();
        for Node(_node_s, particle, node, ty) in &self.nodes {
            let mut extras: Vec<String> = vec![];
            if let Some(feedback) = &self.feedback {
                for HasTag(_hts, source, sink, tag) in &feedback.has_tags {
                    if sink == node && source != node {
                        extras.push(format!("'{}' from {}", tag, source));
                    }
                }
            }
            for TrustedToRemoveTag(_trusted_s, trusted_n, tag) in &self.trusted_to_remove_tag {
                if trusted_n == node {
                    extras.push(format!("trusted to remove tag '{}'", tag));
                }
            }
            for Claim(_claim_s, claim_node, tag) in &self.claims {
                if claim_node == node {
                    extras.push(format!("claims to be '{}'", tag));
                }
            }
            for Check(_check_s, check_node, tag) in &self.checks {
                if check_node == node {
                    extras.push(format!("<font color=\"blue\">checked to be '{}'</font>", tag));
                }
            }
            let extras: Vec<String> = extras.iter().map(|ex| format!("<tr><td>{}</td></tr>", ex)).collect();
            let particle_g = particles.entry(particle).or_insert(DotGraph::default());
            particle_g.add_node(format!("{node_id} [shape=record label=< <table border=\"0\"><tr><td>{node} : {ty}</td></tr>{extras}</table>>]", node_id=node_id(node), node=node, ty=ty, extras=extras.join("")));
        }
        for (particle, particle_g) in particles {
            sol_graph.add_child(particle_id(particle), format!("{} : Particle", particle), particle_g);
        }

        if let Some(feedback) = &self.feedback {
            for Leak(_leak_s, node, expected, source, tag) in &feedback.leaks {
                sol_graph.add_edge(node_id(source), node_id(node), vec![format!("style=dotted color=red label=<<font color=\"red\">expected '{}', found contradiction '{}'</font>>", expected, tag)]);
            }

            for TypeError(_error_s, from, from_ty, to, to_ty) in &feedback.type_errors {
                sol_graph.add_edge(node_id(&from), node_id(&to), vec![format!("style=dotted color=red label=<<font color=\"red\">expected '{}', found incompatible type '{}'</font>>", to_ty, from_ty)]);
            }
        }

        for (from_id, to_id) in &self.id.expect("WAT").edges() {
            let from = format!("{}:s", node_id(from_id));
            let to = format!("{}:n", node_id(to_id));
            sol_graph.add_edge(
                from.clone(),
                to.clone(),
                vec![]
            );
        }
        sol_graph
    }
}

impl Ibis {
    pub fn new() -> Self {
        Ibis::default() // All the accumulated recipe info
    }

    pub fn add_recipies(&mut self, recipies: Ibis) {
        let Ibis {
            config: Config { metadata: _, types, subtypes, less_private_than },
            mut recipies, // Mutation required to move rather than copy the data.
        } = recipies;
        self.config.types.extend(types);
        self.config.subtypes.extend(subtypes);
        self.config.less_private_than.extend(less_private_than);
        self.recipies.extend(recipies.drain(0..));
    }

    pub fn extract_solutions(self) -> Ibis {
        let mut runtime = Crepe::new();
        runtime.extend(self.config.types.iter().map(|ty| ty.to_claim()));
        runtime.extend(self.config.subtypes.iter().map(|sub_ty| sub_ty.to_claim()));
        runtime.extend(self.config.less_private_than.iter().map(|lpt| lpt.to_claim()));
        runtime.extend(self.recipies.iter().map(|recipe| {
            // Convert to a solution (via id)
            SolutionInput(Sol::from(recipe))
            // TODO: inject the nodes!!!
        }));
        let (
            solutions,
            mut types,
            mut less_private_than,
            mut subtypes,
            _nodes,
            _claims,
            _checks,
            _trusted_to_remove_tag,
            has_tags,
            leaks,
            type_errors
        ) = runtime.run();
        let recipies = solutions.iter().map(|Solution(s)| {
            Recipe::from_sol(*s)
                .with_feedback(
                    Feedback {
                        leaks: leaks.iter().filter(|Leak(leak_s, _, _, _, _)| leak_s == s).cloned().collect(),
                        type_errors: type_errors.iter().filter(|TypeError(type_s, _, _, _, _)| type_s == s).cloned().collect(),
                        has_tags: has_tags.iter().filter(|HasTag(has_tag_s, _, _, _)| has_tag_s == s).cloned().collect(),
                    }
                )
        }).collect();
        Ibis {
            config: Config {
                metadata: serde_json::Value::Null,
                types: types.drain().collect(),
                subtypes: subtypes.drain().collect(),
                less_private_than: less_private_than.drain().collect(),
            },
            recipies,
        }
    }
}

