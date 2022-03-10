use crate::util::make;
use crate::{apply, arg, ent, ibis, is_a, Ent, Sol, SolutionData, ToInput};
use serde::{Deserialize, Serialize};

ibis! {
    PlanningIsEnabled(bool);
    Solution(Sol);
    UncheckedSolution(Sol);
    KnownType(Ent); // type
    LessPrivateThan(Ent, Ent); // tag, tag
    Capability(Ent, Ent); // cap from, cap to
    Subtype(Ent, Ent); // sub, super
    Node(Ent, Ent, Ent, Ent); // particle-identifier, identifier, capability, type
    Claim(Ent, Ent); // identifier, tag
    Check(Ent, Ent); // identifier, tag
    TrustedToRemoveTag(Ent, Ent); // node, tag

    // Feedback
    HasTag(Sol, Ent, Ent, Ent); // solution, source node, node with tag, tag
    Leak(Sol, Ent, Ent, Ent, Ent); // sol, node, expected_tag, source, tag2
    TypeError(Sol, Ent, Ent, Ent, Ent); // sol, node, ty, source, ty
    CapabilityError(Sol, Ent, Ent, Ent, Ent); // sol, node, cap, source, cap

    UncheckedSolution(parent.add_edge(from, to)) <-
        PlanningIsEnabled(true),
        Capability(from_capability, to_capability),
        Node(from_particle, from, from_capability, from_type),
        Subtype(from_type, to_type),
        Node(to_particle, to, to_capability, to_type),
        (from != to),
        UncheckedSolution(parent);

    Subtype(
        x,
        prod
    ) <-
        KnownType(prod),
        (is_a!(prod, ent!("ibis.ProductType"))),
        KnownType(x),
        Subtype(x, arg!(prod, 0)),
        Subtype(x, arg!(prod, 1));

    Subtype(
        prod,
        arg!(prod, 0)
    ) <-
        KnownType(prod),
        (is_a!(prod, ent!("ibis.ProductType")));

    Subtype(
        prod,
        arg!(prod, 1)
    ) <-
        KnownType(prod),
        (is_a!(prod, ent!("ibis.ProductType")));

    Subtype(
        union_type,
        x
    ) <-
        KnownType(union_type),
        (is_a!(union_type, ent!("ibis.UnionType"))),
        KnownType(x),
        Subtype(arg!(union_type, 0), x),
        Subtype(arg!(union_type, 1), x);

    Subtype(
        arg!(union_type, 0),
        union_type
    ) <-
        KnownType(union_type),
        (is_a!(union_type, ent!("ibis.UnionType")));

    Subtype(
        arg!(union_type, 1),
        union_type
    ) <-
        KnownType(union_type),
        (is_a!(union_type, ent!("ibis.UnionType")));

    Subtype(
        labelled,
        arg!(labelled, 1)
    ) <-
        KnownType(labelled),
        (is_a!(labelled, ent!("ibis.Labelled")));

    Subtype(
        labelled,
        apply!("ibis.Labelled", arg!(labelled, 0), sup)
    ) <-
        KnownType(labelled),
        (is_a!(labelled, ent!("ibis.Labelled"))),
        Subtype(arg!(labelled, 1), sup);

    Subtype(
        apply!(x_generic, x_arg),
        apply!(y_generic, y_arg)
    ) <-
        Subtype(x_generic, ent!("ibis.GenericType")),
        Subtype(x_generic, ent!("ibis.InductiveType")),
        Subtype(y_generic, ent!("ibis.GenericType")),
        Subtype(y_generic, ent!("ibis.InductiveType")),
        Subtype(x_generic, y_generic),
        Subtype(x_arg, y_arg),
        KnownType(apply!(x_generic, x_arg)),
        KnownType(apply!(y_generic, y_arg));

    HasTag(s, n, n, tag) <- UncheckedSolution(s), Claim(n, tag);
    HasTag(s, source, *down, tag) <- // Propagate tags 'downstream'
        HasTag(s, source, curr, tag),
        for (up, down) in &s.solution().edges,
        (*up == curr),
        !TrustedToRemoveTag(*down, tag);

    HasTag(s, source, down, tag) <- // Propagate tags 'across stream' (i.e. inside a particle)
        HasTag(s, source, curr, tag),
        Node(particle, curr, _, _),
        Node(particle, down, _, _),
        !TrustedToRemoveTag(down, tag);

    Leak(s, n, t1, source, t2) <-
        Check(n, t1),
        LessPrivateThan(t1, t2),
        HasTag(s, source, n, t2); // Check failed, node has a 'more private' tag i.e. is leaking.

    TypeError(s, *from, from_ty, *to, to_ty) <-
        UncheckedSolution(s),
        for (from, to) in &s.solution().edges,
        Node(_from_p, *from, _, from_ty),
        Node(_to_p, *to, _, to_ty),
        !Subtype(from_ty, to_ty); // Check failed, from writes an incompatible type into to

    CapabilityError(s, *from, from_capability, *to, to_capability) <-
        UncheckedSolution(s),
        for (from, to) in &s.solution().edges,
        Node(_from_p, *from, from_capability, _),
        Node(_to_p, *to, to_capability, _),
        !Capability(from_capability, to_capability); // Check failed, from writes an incompatible type into to

    Solution(s) <-
        UncheckedSolution(s),
        !TypeError(s, _, _, _, _),
        !CapabilityError(s, _, _, _, _),
        !Leak(s, _, _, _, _);

    KnownType(x) <- Node(_par, _node, _cap, x); // Infer types that are used in the recipes.
    KnownType(x) <- Subtype(x, _);
    KnownType(y) <- Subtype(_, y);
    Subtype(x, ent!("ibis.UniversalType")) <- KnownType(x); // Create a universal type.
    Subtype(x, x) <- KnownType(x); // Infer simple subtyping.
    Subtype(x, z) <- Subtype(x, y), Subtype(y, z) // Infer the transitivity of subtyping.
}

fn is_default<T: Default + Eq>(v: &T) -> bool {
    v == &T::default()
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Flags {
    #[serde(default, skip_serializing_if = "is_default")]
    pub planning: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default, skip_serializing_if = "is_default")]
    pub metadata: serde_json::Value,
    #[serde(default, skip_serializing_if = "is_default")]
    pub types: Vec<KnownType>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub subtypes: Vec<Subtype>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub less_private_than: Vec<LessPrivateThan>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub capabilities: Vec<Capability>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: Flags,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Feedback {
    #[serde(default, skip_serializing_if = "is_default")]
    pub leaks: Vec<Leak>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub type_errors: Vec<TypeError>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub capability_errors: Vec<CapabilityError>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub has_tags: Vec<HasTag>,
}

fn starting_recipes() -> Vec<Recipe> {
    vec![Recipe::default()]
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Ibis {
    #[serde(flatten)]
    pub config: Config,
    #[serde(default = "starting_recipes", skip_serializing_if = "is_default")]
    pub recipes: Vec<Recipe>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub num_unchecked_solutions: usize,
    #[serde(default, skip_serializing_if = "is_default")]
    pub num_solutions: usize,
    #[serde(default, skip_serializing_if = "is_default")]
    pub num_selected: usize,
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Recipe {
    #[serde(default, skip_serializing_if = "is_default")]
    pub metadata: serde_json::Value,
    #[serde(skip, default)]
    pub id: Option<Sol>,
    // Do not deserialize the feedback on a recipe: Re-generate it each time for consistency.
    #[serde(skip_deserializing, flatten)]
    pub feedback: Option<Feedback>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub nodes: Vec<Node>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub claims: Vec<Claim>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub checks: Vec<Check>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub trusted_to_remove_tag: Vec<TrustedToRemoveTag>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub edges: Vec<(Ent, Ent)>,
    #[cfg(feature = "ancestors")]
    #[serde(default, skip_serializing_if = "is_default")]
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
            nodes: vec![],
            claims: vec![],
            checks: vec![],
            trusted_to_remove_tag: vec![],
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
        let data = SolutionData {
            edges: make(&recipe.edges, Clone::clone),
        };
        Sol::new_blocking(data)
    }
}

impl From<Sol> for Recipe {
    fn from(sol: Sol) -> Self {
        let solution = sol.solution();
        Recipe {
            id: Some(sol),
            feedback: None,
            metadata: serde_json::Value::Null,
            nodes: vec![],
            claims: vec![],
            checks: vec![],
            trusted_to_remove_tag: vec![],
            edges: make(&solution.edges, Clone::clone),
            #[cfg(feature = "ancestors")]
            ancestors: sol.ancestors().iter().cloned().collect(),
        }
    }
}

impl Ibis {
    pub fn new() -> Self {
        Ibis::default() // All the accumulated recipe info
    }

    pub fn add_recipes(&mut self, recipes: Ibis) {
        let Ibis {
            config:
                Config {
                    metadata: _,
                    types,
                    subtypes,
                    less_private_than,
                    capabilities,
                    flags,
                },
            mut recipes, // Mutation required to move rather than copy the data.
            num_unchecked_solutions: _,
            num_solutions: _,
            num_selected: _,
        } = recipes;
        self.config.flags = flags; // TODO: Merge not overwrite.
        self.config.types.extend(types);
        self.config.subtypes.extend(subtypes);
        self.config.less_private_than.extend(less_private_than);
        self.config.capabilities.extend(capabilities);
        self.recipes.extend(recipes.drain(0..));
    }

    pub fn extract_solutions_with_loss(self, loss: Option<usize>) -> Ibis {
        let mut runtime = Crepe::new();
        runtime.extend(&[PlanningIsEnabled(self.config.flags.planning).to_claim()]);
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

        for recipe in self.recipes {
            runtime.extend(recipe.checks.iter().map(|check| check.to_claim()));
            runtime.extend(recipe.claims.iter().map(|claim| claim.to_claim()));
            runtime.extend(recipe.nodes.iter().map(|node| node.to_claim()));
            runtime.extend(
                recipe
                    .trusted_to_remove_tag
                    .iter()
                    .map(|trusted| trusted.to_claim()),
            );
            // Add necessary data to this module and add a 'new solution'.
            let sol = Sol::from(&recipe);
            runtime.extend(vec![UncheckedSolutionInput(sol)]);
        }

        let (
            _flags,
            solutions,
            unchecked_solutions,
            mut types,
            mut less_private_than,
            mut capabilities,
            mut subtypes,
            _nodes,
            _claims,
            _checks,
            _trusted_to_remove_tag,
            has_tags,
            leaks,
            type_errors,
            capability_errors,
        ) = runtime.run();
        let recipes: Vec<Sol> = if self.config.flags.planning {
            solutions.iter().map(|Solution(s)| *s).collect()
        } else {
            unchecked_solutions
                .iter()
                .map(|UncheckedSolution(s)| *s)
                .collect()
        };
        let mut recipes: Vec<Recipe> = recipes
            .iter()
            .map(|s| {
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
            })
            .collect();
        let recipes = if let Some(loss) = loss {
            let mut max = 0;
            for r in &recipes {
                let l = r.edges.len();
                if max < l {
                    max = l;
                }
            }
            recipes
                .drain(0..)
                .filter(|recipe| recipe.edges.len() >= max - loss)
                .collect()
        } else {
            recipes
        };
        Ibis {
            config: Config {
                metadata: serde_json::Value::Null,
                types: types.drain().collect(),
                subtypes: subtypes.drain().collect(),
                less_private_than: less_private_than.drain().collect(),
                capabilities: capabilities.drain().collect(),
                flags: self.config.flags.clone(),
            },
            num_unchecked_solutions: unchecked_solutions.len(),
            num_solutions: solutions.len(),
            num_selected: recipes.len(),
            recipes,
        }
    }
}
