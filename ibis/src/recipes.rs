#![allow(clippy::collapsible_if)]

use crate::type_struct::*;
use crate::util::make;
use crate::{apply, arg, args, ent, is_a, name, Ent, Sol, SolutionData};
use crepe::crepe;
use serde::{Deserialize, Serialize};

crepe! {
    @input
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct PlanningIsEnabled(pub bool);
    @output
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct Solution(pub Sol);

    @input
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct Seed(pub Sol);

    @output
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct UncheckedSolution(pub Sol);

    UncheckedSolution(s) <- Seed(s);

    struct KnownType(pub Ent); // type

    @input
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct LessPrivateThan(pub Ent, pub Ent); // tag, tag
    @input
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct Capability(pub Ent, pub Ent); // cap from, cap to
    @input
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct Subtype(pub Ent, pub Ent); // sub, super

    struct ComputedSubtype(pub Ent, pub Ent); // sub, super
    ComputedSubtype(x, y) <- Subtype(x, y);

    struct CompatibleWith(pub Ent, pub Ent); // from, to
    struct HasCapability(pub Ent, pub Ent); // cap, ty

    @input
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct Node(pub Ent, pub Ent, pub Ent); // particle-identifier, identifier, capability, type
    @input
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct Claim(pub Ent, pub Ent); // identifier, tag
    @input
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct Check(pub Ent, pub Ent); // identifier, tag
    @input
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct TrustedToRemoveTag(pub Ent, pub Ent); // node, tag

    // Feedback
    @output
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct HasTag(pub Sol, pub Ent, pub Ent, pub Ent); // solution, source node, node with tag, tag
    @output
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct Leak(pub Sol, pub Ent, pub Ent, pub Ent, pub Ent); // sol, node, expected_tag, source, tag2
    @output
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct TypeError(pub Sol, pub Ent, pub Ent, pub Ent, pub Ent); // sol, node, ty, source, ty
    UncheckedSolution(parent.add_edge(from, to)) <-
        PlanningIsEnabled(true),
        Node(_from_particle, from, from_type),
        Node(_to_particle, to, to_type),
        (from != to),
        CompatibleWith(from_type, to_type),
        // ({eprintln!("Connecting {}: {} to {}: {}", from, from_type, to, to_type); true}),
        UncheckedSolution(parent);

    HasCapability(arg!(ty, 0), ty) <-
        KnownType(ty),
        (is_a!(ty, WITH_CAPABILITY));

    HasCapability(cap, ty) <-
        KnownType(ty),
        (is_a!(ty, WITH_CAPABILITY)),
        HasCapability(cap, arg!(ty, 1)); // Has all the child capabilities too.

    // Base case: just types.
    CompatibleWith(x, y) <-
        KnownType(x),
        (!is_a!(x, WITH_CAPABILITY)),
        KnownType(y),
        (!is_a!(y, WITH_CAPABILITY)),
        // ({eprintln!("checking subtyping ({}) ({})", x, y); true}),
        ComputedSubtype(x, y);

    CompatibleWith(x, y) <- // Check that y has the capabilities required by x.
        KnownType(x),
        (is_a!(x, WITH_CAPABILITY)),
        KnownType(y),
        HasCapability(cap, y), // For each of the capabilities y supports
        // ({eprintln!("checking y has cap ({}) ({})", x, y); true}),
        Capability(arg!(x, 0), cap), // If this one is supported we can continue.
        CompatibleWith(arg!(x, 1), y);

    CompatibleWith(x, y) <- // If a type has no capabilities, discard the capabilities of it's possible super type.
        KnownType(x),
        (!is_a!(x, WITH_CAPABILITY)),
        KnownType(y),
        (is_a!(y, WITH_CAPABILITY)),
        // ({eprintln!("discarding capability from y ({}) ({})", x, y); true}),
        CompatibleWith(x, arg!(y, 1));

    ComputedSubtype(
        x,
        prod
    ) <-
        KnownType(prod),
        (is_a!(prod, PRODUCT)),
        KnownType(x),
        ComputedSubtype(x, arg!(prod, 0)),
        ComputedSubtype(x, arg!(prod, 1));

    ComputedSubtype(
        prod,
        arg!(prod, 0)
    ) <-
        KnownType(prod),
        (is_a!(prod, PRODUCT));

    ComputedSubtype(
        prod,
        arg!(prod, 1)
    ) <-
        KnownType(prod),
        (is_a!(prod, PRODUCT));

    ComputedSubtype(
        union_type,
        x
    ) <-
        KnownType(union_type),
        (is_a!(union_type, UNION)),
        KnownType(x),
        ComputedSubtype(arg!(union_type, 0), x),
        ComputedSubtype(arg!(union_type, 1), x);

    ComputedSubtype(
        arg!(union_type, 0),
        union_type
    ) <-
        KnownType(union_type),
        (is_a!(union_type, UNION));

    ComputedSubtype(
        arg!(union_type, 1),
        union_type
    ) <-
        KnownType(union_type),
        (is_a!(union_type, UNION));

    ComputedSubtype(
        labelled,
        arg!(labelled, 1)
    ) <-
        KnownType(labelled),
        (is_a!(labelled, LABELLED));

    ComputedSubtype(
        labelled,
        apply!(ent!(LABELLED), arg!(labelled, 0), sup)
    ) <-
        KnownType(labelled),
        (is_a!(labelled, LABELLED)),
        ComputedSubtype(arg!(labelled, 1), sup);

    ComputedSubtype(
        apply!(x_generic, x_arg),
        apply!(y_generic, y_arg)
    ) <-
        ComputedSubtype(x_generic, ent!(GENERIC)),
        ComputedSubtype(x_generic, ent!(INDUCTIVE)),
        ComputedSubtype(y_generic, ent!(GENERIC)),
        ComputedSubtype(y_generic, ent!(INDUCTIVE)),
        ComputedSubtype(x_generic, y_generic),
        ComputedSubtype(x_arg, y_arg),
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
        Node(particle, curr, _),
        Node(particle, down, _),
        !TrustedToRemoveTag(down, tag);

    Leak(s, n, t1, source, t2) <-
        Check(n, t1),
        LessPrivateThan(t1, t2),
        HasTag(s, source, n, t2); // Check failed, node has a 'more private' tag i.e. is leaking.

    TypeError(s, *from, from_ty, *to, to_ty) <-
        UncheckedSolution(s),
        for (from, to) in &s.solution().edges,
        Node(_from_p, *from, from_ty),
        Node(_to_p, *to, to_ty),
        !CompatibleWith(from_ty, to_ty); // Check failed, from writes an incompatible type into to

    Solution(s) <-
        UncheckedSolution(s),
        !TypeError(s, _, _, _, _),
        !Leak(s, _, _, _, _);

    KnownType(name!(ty)) <- KnownType(ty); // Types without their arguments are still types
    KnownType(arg) <- KnownType(ty), for arg in args!(ty); // Types arguments are types
    KnownType(x) <- Node(_par, _node, x); // Infer types that are used in the recipes.
    KnownType(x) <- ComputedSubtype(x, _);
    KnownType(y) <- ComputedSubtype(_, y);
    ComputedSubtype(x, ent!(UNIVERSAL)) <- KnownType(x); // Create a universal type.
    ComputedSubtype(x, x) <- KnownType(x); // Infer simple subtyping.
    ComputedSubtype(x, z) <- ComputedSubtype(x, y), ComputedSubtype(y, z); // Infer the transitivity of subtyping.
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
    pub subtypes: Vec<Subtype>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub less_private_than: Vec<LessPrivateThan>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub capabilities: Vec<Capability>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: Flags,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Feedback {
    #[serde(default, skip_serializing_if = "is_default")]
    pub leaks: Vec<Leak>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub type_errors: Vec<TypeError>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub has_tags: Vec<HasTag>,
}

fn starting_recipes() -> Vec<Recipe> {
    vec![Recipe::default()]
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde()]
pub struct Ibis {
    #[serde(flatten)]
    pub config: Config,
    #[serde(flatten)]
    pub shared: Recipe,
    #[serde(default = "starting_recipes", skip_serializing_if = "is_default")]
    pub recipes: Vec<Recipe>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub num_unchecked_solutions: usize,
    #[serde(default, skip_serializing_if = "is_default")]
    pub num_solutions: usize,
    #[serde(default, skip_serializing_if = "is_default")]
    pub num_selected: usize,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Recipe {
    #[serde(default, skip_serializing_if = "is_default")]
    pub metadata: serde_json::Value,
    #[serde(skip, default)]
    pub id: Option<Sol>,
    // Do not deserialize the feedback on a recipe: Re-generate it each time for consistency.
    #[serde(flatten)]
    pub feedback: Feedback,
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
            feedback: Feedback::default(),
            metadata: serde_json::Value::Null,
            nodes: vec![],
            claims: vec![],
            checks: vec![],
            trusted_to_remove_tag: vec![],
            edges: solution.edges.iter().cloned().collect(),
        }
    }

    pub fn with_feedback(mut self, feedback: Feedback) -> Self {
        self.feedback = feedback;
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
        Recipe::from_sol(sol)
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
                    subtypes,
                    less_private_than,
                    capabilities,
                    flags,
                },
            mut recipes, // Mutation required to move rather than copy the data.
            num_unchecked_solutions: _,
            num_solutions: _,
            num_selected: _,
            shared,
        } = recipes;
        self.config.flags = flags; // TODO: Merge not overwrite.
        self.config.subtypes.extend(subtypes);
        self.config.less_private_than.extend(less_private_than);
        self.config.capabilities.extend(capabilities);
        self.recipes.extend(recipes.drain(0..));
        self.shared = shared; // TODO: Merge not overwrite.
    }

    pub fn extract_solutions_with_loss(self, loss: Option<usize>) -> Ibis {
        let mut runtime = Crepe::new();
        runtime.extend(&[PlanningIsEnabled(self.config.flags.planning)]);
        runtime.extend(self.config.subtypes.clone());
        runtime.extend(self.config.less_private_than.clone());
        runtime.extend(self.config.capabilities.clone());

        let maybe_shared: Option<&Recipe> = if Sol::from(&self.shared) == Sol::default() {
            None
        } else {
            Some(&self.shared)
        };
        for recipe in self.recipes.iter().chain(maybe_shared) {
            // Add necessary data to this module and add a 'new solution'.
            let sol = Sol::from(recipe);
            runtime.extend(&[Seed(sol)]);
        }

        for recipe in self.recipes.iter().chain(Some(self.shared.clone()).iter()) {
            // Add necessary data to this module and add a 'new solution'.
            let Recipe {
                checks,
                claims,
                nodes,
                trusted_to_remove_tag,
                feedback: _,
                metadata: _,
                id: _,
                edges: _, // To be captured by sol
                #[cfg(feature = "ancestors")]
                    ancestors: _,
            } = recipe;
            runtime.extend(checks);
            runtime.extend(claims);
            runtime.extend(nodes);
            runtime.extend(trusted_to_remove_tag);
        }

        let (solutions, unchecked_solutions, mut has_tags, mut leaks, mut type_errors) = runtime.run();
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
                    leaks: leaks.drain_filter(|leak| leak.0 == *s).collect(),
                    type_errors: type_errors.drain_filter(|ty| ty.0 == *s).collect(),
                    has_tags: has_tags.drain_filter(|has_tag| has_tag.0 == *s).collect(),
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
                .drain_filter(|recipe| recipe.edges.len() >= max - loss)
                .collect()
        } else {
            recipes
        };
        Ibis {
            config: self.config,
            num_unchecked_solutions: unchecked_solutions.len(),
            num_solutions: solutions.len(),
            num_selected: recipes.len(),
            recipes,
            shared: self.shared,
        }
    }
}
