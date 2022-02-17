use crate::util::make;
use crate::{apply, arg, ent, ibis, is_a, Ent, Sol, SolutionData, ToInput};
use serde::{Deserialize, Serialize};

ibis! {
    Solution(Sol);
    Type(Ent); // type
    LessPrivateThan(Ent, Ent); // tag, tag
    Capability(Ent, Ent); // cap from, cap to
    Subtype(Ent, Ent); // sub, super
    Node(Ent, Ent, Ent, Ent); // particle-identifier, identifier, capability, type
    Claim(Ent, Ent); // identifier, tag
    Check(Ent, Ent); // identifier, tag

    // Feedback
    HasTag(Sol, Ent, Ent, Ent); // solution, source node, node with tag, tag
    Leak(Sol, Ent, Ent, Ent, Ent); // sol, node, expected_tag, source, tag2
    TypeError(Sol, Ent, Ent, Ent, Ent); // sol, node, ty, source, ty
    CapabilityError(Sol, Ent, Ent, Ent, Ent); // sol, node, cap, source, cap

    Solution(parent.add_edge(from, to)) <-
        Capability(from_capability, to_capability),
        Node(from_particle, from, from_capability, from_type),
        Node(to_particle, to, to_capability, to_type),
        (from != to),
        Subtype(from_type, to_type),
        Solution(parent),
        (!parent.has_edge(from, to));

    Subtype(
        prod,
        arg!(prod, 0)
    ) <-
        Type(prod),
        (is_a!(prod, ent!("ibis::ProductType")));

    Subtype(
        prod,
        arg!(prod, 1)
    ) <-
        Type(prod),
        (is_a!(prod, ent!("ibis::ProductType")));

    Subtype(
        labelled,
        arg!(labelled, 1)
    ) <-
        Type(labelled),
        (is_a!(labelled, ent!("ibis::Labelled")));

    Subtype(
        apply!(x_generic, x_arg),
        apply!(y_generic, y_arg)
    ) <-
        Subtype(x_generic, ent!("ibis::GenericType")),
        Subtype(x_generic, ent!("ibis::InductiveType")),
        Subtype(y_generic, ent!("ibis::GenericType")),
        Subtype(y_generic, ent!("ibis::InductiveType")),
        Subtype(x_generic, y_generic),
        Subtype(x_arg, y_arg),
        Type(apply!(x_generic, x_arg)),
        Type(apply!(y_generic, y_arg));

    HasTag(s, n, n, tag) <- Solution(s), Claim(n, tag);
    HasTag(s, source, down, tag) <- // Propagate tags 'downstream'
        HasTag(s, source, curr, tag),
        Node(_down_particle, down, _, _),
        (s.has_edge(curr, down)),
        (!s.is_trusted_to_remove_tag(down, tag));

    HasTag(s, source, down, tag) <- // Propagate tags 'across stream' (i.e. inside a particle)
        HasTag(s, source, curr, tag),
        Node(particle, curr, _, _),
        Node(particle, down, _, _),
        (!s.is_trusted_to_remove_tag(down, tag));

    Leak(s, n, t1, source, t2) <-
        Check(n, t1),
        LessPrivateThan(t1, t2),
        HasTag(s, source, n, t2); // Check failed, node has a 'more private' tag i.e. is leaking.

    TypeError(s, from, from_ty, to, to_ty) <-
        Node(_from_p, from, _, from_ty),
        Node(_to_p, to, _, to_ty),
        Solution(s),
        (s.has_edge(from, to)),
        !Subtype(from_ty, to_ty); // Check failed, from writes an incompatible type into to

    CapabilityError(s, from, from_capability, to, to_capability) <-
        Node(_from_p, from, from_capability, _),
        Node(_to_p, to, to_capability, _),
        Solution(s),
        (s.has_edge(from, to)),
        !Capability(from_capability, to_capability); // Check failed, from writes an incompatible type into to

    Type(x) <- Node(_par, _node, _cap, x); // Infer types that are used in the recipies.
    Type(x) <- Subtype(x, _);
    Type(y) <- Subtype(_, y);
    Subtype(x, ent!("ibis::UniversalType")) <- Type(x); // Create a universal type.
    Subtype(x, x) <- Type(x); // Infer simple subtyping.
    Subtype(x, z) <- Subtype(x, y), Subtype(y, z) // Infer the transitivity of subtyping.
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub metadata: serde_json::Value,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<Type>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtypes: Vec<Subtype>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub less_private_than: Vec<LessPrivateThan>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<Capability>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Feedback {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub leaks: Vec<Leak>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_errors: Vec<TypeError>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capability_errors: Vec<CapabilityError>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub has_tags: Vec<HasTag>,
}

fn starting_recipies() -> Vec<Recipe> {
    vec![Recipe::default()]
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Ibis {
    #[serde(flatten)]
    pub config: Config,
    #[serde(default = "starting_recipies", skip_serializing_if = "Vec::is_empty")]
    pub recipies: Vec<Recipe>,
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Recipe {
    #[serde(default)]
    pub metadata: serde_json::Value,
    #[serde(skip, default)]
    pub id: Option<Sol>,
    // Do not deserialize the feedback on a recipe: Re-generate it each time for consistency.
    #[serde(skip_deserializing, flatten)]
    pub feedback: Option<Feedback>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<Node>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub claims: Vec<Claim>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checks: Vec<Check>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trusted_to_remove_tag: Vec<(Ent, Ent)>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edges: Vec<(Ent, Ent)>,
    #[cfg(feature = "ancestors")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ancestors: Vec<Sol>,
}

impl Recipe {
    pub fn from_sol(sol: Sol) -> Self {
        let solution = sol.solution();
        Recipe {
            #[cfg(feature = "ancestors")]
            ancestors: sol.ancestors().iter().cloned().collect(),
            id: Some(sol),
            feedback: None,
            metadata: serde_json::Value::Null,
            nodes: solution
                .nodes
                .iter()
                .map(|node| {
                    let particle = solution.node_to_particle.get(node).unwrap();
                    let ty = solution.node_types.get(node).unwrap();
                    let cap = solution.node_capabilities.get(node).unwrap();
                    Node(*particle, *node, *cap, *ty)
                })
                .collect(),
            claims: solution
                .claims
                .iter()
                .map(|(node, tag)| Claim(*node, *tag))
                .collect(),
            checks: solution
                .checks
                .iter()
                .map(|(node, tag)| Check(*node, *tag))
                .collect(),
            trusted_to_remove_tag: solution.trusted_to_remove_tag.iter().cloned().collect(),
            edges: solution.edges.iter().cloned().collect(),
        }
    }

    pub fn with_feedback(mut self, feedback: Feedback) -> Self {
        self.feedback = Some(feedback);
        self
    }
}

impl From<&Recipe> for Sol {
    fn from(recipe: &Recipe) -> Self {
        // Convert the recipe to its 'solution data'
        let solution = SolutionData::from(recipe);
        // Get an id to represent that.
        Sol::new_blocking(solution)
    }
}

impl From<&Recipe> for SolutionData {
    fn from(recipe: &Recipe) -> Self {
        Self {
            edges: make(&recipe.edges, Clone::clone),
            checks: make(&recipe.checks, |Check(node, tag)| (*node, *tag)),
            claims: make(&recipe.claims, |Claim(node, tag)| (*node, *tag)),
            node_to_particle: make(&recipe.nodes, |Node(particle, node, _cap, _ty)| {
                (*node, *particle)
            }),
            node_types: make(&recipe.nodes, |Node(_particle, node, _cap, ty)| {
                (*node, *ty)
            }),
            node_capabilities: make(&recipe.nodes, |Node(_particle, node, cap, _ty)| {
                (*node, *cap)
            }),
            nodes: make(&recipe.nodes, |Node(_particle, node, _cap, _ty)| *node),
            trusted_to_remove_tag: make(&recipe.trusted_to_remove_tag, Clone::clone),
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
            nodes: make(&solution.nodes, |node| {
                let particle = solution.node_to_particle.get(node).unwrap();
                let ty = solution.node_types.get(node).unwrap();
                let cap = solution.node_capabilities.get(node).unwrap();
                Node(*particle, *node, *cap, *ty)
            }),
            claims: make(&solution.claims, |(node, tag)| Claim(*node, *tag)),
            checks: make(&solution.checks, |(node, tag)| Check(*node, *tag)),
            trusted_to_remove_tag: make(&solution.trusted_to_remove_tag, Clone::clone),
            edges: make(&solution.edges, Clone::clone),
            #[cfg(feature = "ancestors")]
            ancestors: sol.ancestors().iter().cloned().collect(),
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

impl Ibis {
    pub fn new() -> Self {
        Ibis::default() // All the accumulated recipe info
    }

    pub fn add_recipies(&mut self, recipies: Ibis) {
        let Ibis {
            config:
                Config {
                    metadata: _,
                    types,
                    subtypes,
                    less_private_than,
                    capabilities,
                },
            mut recipies, // Mutation required to move rather than copy the data.
        } = recipies;
        self.config.types.extend(types);
        self.config.subtypes.extend(subtypes);
        self.config.less_private_than.extend(less_private_than);
        self.config.capabilities.extend(capabilities);
        self.recipies.extend(recipies.drain(0..));
    }

    pub fn extract_solutions(self) -> Ibis {
        self.extract_solutions_with_loss(None)
    }

    pub fn extract_best_solutions(self) -> Ibis {
        self.extract_solutions_with_loss(Some(0))
    }

    pub fn extract_solutions_with_loss(self, loss: Option<usize>) -> Ibis {
        let mut runtime = Crepe::new();
        runtime.extend(self.config.types.iter().map(|ty| ty.to_claim()));
        runtime.extend(self.config.subtypes.iter().map(|sub_ty| sub_ty.to_claim()));
        runtime.extend(
            self.config
                .less_private_than
                .iter()
                .map(|lpt| lpt.to_claim()),
        );
        runtime.extend(
            self.config
                .capabilities
                .iter()
                .map(|capability| capability.to_claim()),
        );
        runtime.extend(self.recipies.iter().map(|recipe| {
            // Convert to a solution (via id)
            SolutionInput(Sol::from(recipe))
        }));

        for recipe in self.recipies {
            runtime.extend(recipe.checks.iter().map(|check| check.to_claim()));
            runtime.extend(recipe.claims.iter().map(|claim| claim.to_claim()));
            runtime.extend(recipe.nodes.iter().map(|node| node.to_claim()));
        }

        let (
            solutions,
            mut types,
            mut less_private_than,
            mut capabilities,
            mut subtypes,
            _nodes,
            _claims,
            _checks,
            has_tags,
            leaks,
            type_errors,
            capability_errors,
        ) = runtime.run();
        let all_recipies = solutions.iter().map(|Solution(s)| {
            Recipe::from_sol(*s).with_feedback(Feedback {
                leaks: leaks
                    .iter()
                    .filter(|Leak(leak_s, _, _, _, _)| leak_s == s)
                    .cloned()
                    .collect(),
                type_errors: type_errors
                    .iter()
                    .filter(|TypeError(type_s, _, _, _, _)| type_s == s)
                    .cloned()
                    .collect(),
                capability_errors: capability_errors
                    .iter()
                    .filter(|CapabilityError(cap_s, _, _, _, _)| cap_s == s)
                    .cloned()
                    .collect(),
                has_tags: has_tags
                    .iter()
                    .filter(|HasTag(has_tag_s, _, _, _)| has_tag_s == s)
                    .cloned()
                    .collect(),
            })
        });
        let all_recipies_len = all_recipies.len();
        let mut recipies: Vec<Recipe> = all_recipies
            .filter(|recipe| {
                (recipe
                    .feedback
                    .as_ref()
                    .map(|f| f.leaks.len() + f.type_errors.len() + f.capability_errors.len() == 0))
                .unwrap_or(false)
            })
            .collect();
        let recipies_len = recipies.len();
        let recipies = if let Some(loss) = loss {
            let mut max = 0;
            for r in &recipies {
                let l = r.edges.len();
                if max < l {
                    max = l;
                }
            }
            recipies
                .drain(0..)
                .filter(|recipe| recipe.edges.len() >= max - loss)
                .collect()
        } else {
            recipies
        };
        eprintln!(
            "Selected {} of {} valid solutions. (Generated {} solutions)",
            recipies.len(),
            recipies_len,
            all_recipies_len
        );
        Ibis {
            config: Config {
                metadata: serde_json::Value::Null,
                types: types.drain().collect(),
                subtypes: subtypes.drain().collect(),
                less_private_than: less_private_than.drain().collect(),
                capabilities: capabilities.drain().collect(),
            },
            recipies,
        }
    }
}
