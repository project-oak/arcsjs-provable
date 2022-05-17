// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
#![allow(clippy::collapsible_if)]

use crate::type_struct::*;
use crate::util::make;
use crate::{apply, ent, name, Ent, Sol, SolutionData};
use crepe::crepe;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

crepe! {
    @input
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct FlagEnabled(pub &'static str, pub bool);
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
    pub struct SubtypeInput(pub Ent, pub Ent); // sub, super

    struct Subtype(pub Ent, pub Ent); // sub, super
    Subtype(x, y) <- SubtypeInput(x, y);

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
    pub struct TrustedToRemoveSecrecyTag(pub Ent, pub Ent); // node, tag
    @input
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct TrustedToRemoveSecrecyTagFromNode(pub Ent, pub Ent); // node, node from
    @input
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct TrustedToAddIntegrityTag(pub Ent, pub Ent); // node, tag
    @input
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct TrustedToAddIntegrityTagFromNode(pub Ent, pub Ent); // node, node from

    // Feedback
    @output
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct HasSecrecyTag(pub Sol, pub Ent, pub Ent, pub Ent); // solution, source node, node with tag, tag
    @output
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct HasIntegrityTag(pub Sol, pub Ent, pub Ent, pub Ent); // solution, source node, node with tag, tag
    @output
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct Leak(pub Sol, pub Ent, pub Ent, pub Ent, pub Ent); // sol, node, expected_tag, source, tag2
    @output
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct TypeError(pub Sol, pub Ent, pub Ent, pub Ent, pub Ent); // sol, node, ty, source, ty
    UncheckedSolution(parent.add_edge(from, to)) <-
        FlagEnabled(PLANNING, true),
        Node(_from_particle, from, from_type),
        Node(_to_particle, to, to_type),
        (from != to),
        CompatibleWith(from_type, to_type),
        // ({eprintln!("Connecting {}: {} to {}: {}", from, from_type, to, to_type); true}),
        UncheckedSolution(parent);

    HasCapability(cap, ty) <-
        KnownType(ty),
        (ty.is_a(WITH_CAPABILITY)),
        Subtype(ty.args()[0], cap);

    HasCapability(cap, ty) <-
        KnownType(ty),
        (ty.is_a(WITH_CAPABILITY)),
        HasCapability(cap, ty.args()[1]); // Has all the child capabilities too.

    // Base case: just types.
    CompatibleWith(x, y) <-
        KnownType(x),
        (!x.is_a(WITH_CAPABILITY)),
        KnownType(y),
        (!y.is_a(WITH_CAPABILITY)),
        // ({eprintln!("checking subtyping ({}) ({})", x, y); true}),
        Subtype(x, y);

    CompatibleWith(x, y) <- // Check that y has the capabilities required by x.
        KnownType(x),
        (x.is_a(WITH_CAPABILITY)),
        KnownType(y),
        HasCapability(y_cap, y), // For each of the capabilities y supports
        Subtype(x.args()[0], x_cap),
        Capability(x_cap, y_cap), // If this one is supported we can continue.
        CompatibleWith(x.args()[1], y);

    CompatibleWith(x, y) <- // If a type has no capabilities, discard the capabilities of it's possible super type.
        KnownType(x),
        (!x.is_a(WITH_CAPABILITY)),
        KnownType(y),
        (y.is_a(WITH_CAPABILITY)),
        // ({eprintln!("discarding capability from y ({}) ({})", x, y); true}),
        CompatibleWith(x, y.args()[1]);

    // TODO: Replace with the 'all' aggregate when it exists.
    // See https://github.com/ekzhang/crepe/issues/10
    struct SubtypesAllArgs(Ent, Ent, usize);
    SubtypesAllArgs(x, y, 0) <- KnownType(x), KnownType(y);
    SubtypesAllArgs(x, y, n+1) <-
        SubtypesAllArgs(x, y, n),
        (n < y.num_args()),
        Subtype(x, y.args()[n]);

    // TODO: Replace with the 'all' aggregate when it exists.
    // See https://github.com/ekzhang/crepe/issues/10
    struct SupertypesAllArgs(Ent, Ent, usize);
    SupertypesAllArgs(x, y, 0) <- KnownType(x), KnownType(y);
    SupertypesAllArgs(x, y, n+1) <-
        SupertypesAllArgs(x, y, n),
        (n < y.num_args()),
        Subtype(y.args()[n], x);

    Subtype(x, prod) <-
        KnownType(prod),
        (prod.is_a(PRODUCT)),
        KnownType(x),
        SubtypesAllArgs(x, prod, prod.num_args());

    Subtype(
        prod,
        arg
    ) <-
        KnownType(prod),
        (prod.is_a(PRODUCT)),
        for arg in prod.args();

    Subtype(
        union_type,
        x
    ) <-
        KnownType(union_type),
        (union_type.is_a(UNION)),
        KnownType(x),
        SupertypesAllArgs(x, union_type, union_type.num_args());

    Subtype(
        arg,
        union_type
    ) <-
        KnownType(union_type),
        (union_type.is_a(UNION)),
        for arg in union_type.args();

    Subtype(
        labelled,
        labelled.args()[1]
    ) <-
        KnownType(labelled),
        (labelled.is_a(LABELLED));

    Subtype(
        labelled,
        apply!(ent!(LABELLED), labelled.args()[0], sup)
    ) <-
        KnownType(labelled),
        (labelled.is_a(LABELLED)),
        Subtype(labelled.args()[1], sup);

    Subtype(
        apply!(x_generic, x_arg),
        apply!(y_generic, y_arg)
    ) <-
        Subtype(x_generic, ent!(GENERIC)),
        Subtype(x_generic, ent!(INDUCTIVE)),
        Subtype(y_generic, ent!(GENERIC)),
        Subtype(y_generic, ent!(INDUCTIVE)),
        Subtype(x_generic, y_generic),
        Subtype(x_arg, y_arg),
        KnownType(apply!(x_generic, x_arg)),
        KnownType(apply!(y_generic, y_arg));

    HasSecrecyTag(s, n, n, tag) <- UncheckedSolution(s), Claim(n, tag);
    HasSecrecyTag(s, source, *down, tag) <- // Propagate tags 'downstream'
        HasSecrecyTag(s, source, curr, tag),
        for (up, down) in &s.solution().edges,
        (*up == curr),
        !TrustedToRemoveSecrecyTag(*down, tag),
        !TrustedToRemoveSecrecyTagFromNode(*down, curr);

    HasSecrecyTag(s, source, down, tag) <- // Propagate tags 'across stream' (i.e. inside a particle)
        HasSecrecyTag(s, source, curr, tag),
        Node(particle, curr, curr_ty),
        HasCapability(curr_cap, curr_ty),
        Capability(_, curr_cap), // Is input (e.g. read)
        Node(particle, down, down_ty),
        (curr != down),
        !TrustedToRemoveSecrecyTag(down, tag),
        !TrustedToRemoveSecrecyTagFromNode(down, curr),
        HasCapability(down_cap, down_ty), // Has to be able to output it.
        Capability(down_cap, _); // Is output (e.g. write)

    Leak(s, n, t1, source, t2) <-
        Check(n, t1),
        LessPrivateThan(t1, t2),
        HasSecrecyTag(s, source, n, t2); // Check failed, node has a 'more private' tag i.e. is leaking.

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
    KnownType(arg) <- KnownType(ty), for arg in ty.args(); // Types arguments are types
    KnownType(x) <- Node(_par, _node, x); // Infer types that are used in the recipes.
    KnownType(x) <- Subtype(x, _);
    KnownType(y) <- Subtype(_, y);
    Subtype(x, ent!(UNIVERSAL)) <- KnownType(x); // Create a universal type.
    Subtype(x, x) <- KnownType(x); // Infer simple subtyping.
    Subtype(x, z) <- Subtype(x, y), Subtype(y, z); // Infer the transitivity of subtyping.
}

fn is_default<T: Default + Eq>(v: &T) -> bool {
    v == &T::default()
}

const PLANNING: &str = "planning";
const FLAGS: &[&str] = &[PLANNING];

#[derive(Default, Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default, skip_serializing_if = "is_default")]
    pub metadata: serde_json::Value,
    #[serde(default, skip_serializing_if = "is_default")]
    pub subtypes: Vec<SubtypeInput>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub less_private_than: Vec<LessPrivateThan>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub capabilities: Vec<Capability>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: BTreeMap<String, bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Feedback {
    #[serde(default, skip_serializing_if = "is_default")]
    pub leaks: Vec<Leak>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub type_errors: Vec<TypeError>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub has_secrecy_tags: Vec<HasSecrecyTag>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub has_integrity_tags: Vec<HasIntegrityTag>,
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
    #[serde(default, skip_serializing_if = "is_default")]
    pub warnings: Vec<String>,
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
    pub trusted_to_add_integrity_tag: Vec<TrustedToAddIntegrityTag>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub trusted_to_add_integrity_tag_from_node: Vec<TrustedToAddIntegrityTagFromNode>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub trusted_to_remove_secrecy_tag: Vec<TrustedToRemoveSecrecyTag>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub trusted_to_remove_secrecy_tag_from_node: Vec<TrustedToRemoveSecrecyTagFromNode>,
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
            warnings: Vec::new(),
            id: Some(sol),
            feedback: Feedback::default(),
            metadata: serde_json::Value::Null,
            nodes: vec![],
            claims: vec![],
            checks: vec![],
            trusted_to_remove_secrecy_tag: vec![],
            trusted_to_remove_secrecy_tag_from_node: vec![],
            trusted_to_add_integrity_tag: vec![],
            trusted_to_add_integrity_tag_from_node: vec![],
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

    pub fn extract_solutions_with_loss(mut self, loss: Option<usize>) -> Ibis {
        let mut runtime = Crepe::new();
        let mut warnings = Vec::new();
        for (key, value) in &self.config.flags {
            if let Some(flag) = FLAGS.iter().find(|flag| flag == &key) {
                runtime.extend(&[FlagEnabled(flag, *value)]);
            } else {
                warnings.push(format!(
                    "Unknown flag {:?} set to: {:?}. Known flags are {}",
                    key,
                    value,
                    FLAGS.join(", ")
                ));
            }
        }
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
                trusted_to_remove_secrecy_tag,
                trusted_to_remove_secrecy_tag_from_node,
                trusted_to_add_integrity_tag,
                trusted_to_add_integrity_tag_from_node,
                feedback: _,
                metadata: _,
                id: _,
                edges: _,    // To be captured by sol
                warnings: _, // These should be regenerated.
                #[cfg(feature = "ancestors")]
                    ancestors: _,
            } = recipe;
            runtime.extend(checks);
            runtime.extend(claims);
            runtime.extend(nodes);
            runtime.extend(trusted_to_remove_secrecy_tag);
            runtime.extend(trusted_to_remove_secrecy_tag_from_node);
            runtime.extend(trusted_to_add_integrity_tag);
            runtime.extend(trusted_to_add_integrity_tag_from_node);
        }

        let (solutions, unchecked_solutions, has_secrecy_tags, has_integrity_tags, leaks, type_errors) = runtime.run();
        let recipes: Vec<Sol> = if let Some(true) = &self.config.flags.get(PLANNING) {
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
                    has_integrity_tags: has_integrity_tags
                        .iter()
                        .filter(|HasIntegrityTag(has_tag_s, _, _, _)| has_tag_s == s)
                        .cloned()
                        .collect(),
                    has_secrecy_tags: has_secrecy_tags
                        .iter()
                        .filter(|HasSecrecyTag(has_tag_s, _, _, _)| has_tag_s == s)
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
        let mut shared = self.shared;
        shared.warnings.extend(warnings);
        for recipe in self.recipes.drain(0..) {
            shared.nodes.extend(recipe.nodes);
            shared.claims.extend(recipe.claims);
            shared.checks.extend(recipe.checks);
            shared
                .trusted_to_remove_secrecy_tag
                .extend(recipe.trusted_to_remove_secrecy_tag);
            shared
                .trusted_to_remove_secrecy_tag_from_node
                .extend(recipe.trusted_to_remove_secrecy_tag_from_node);
        }
        Ibis {
            config: self.config,
            num_unchecked_solutions: unchecked_solutions.len(),
            num_solutions: solutions.len(),
            num_selected: recipes.len(),
            recipes,
            shared,
        }
    }
}
