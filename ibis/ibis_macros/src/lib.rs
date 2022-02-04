// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree, TokenTree::*};

#[derive(Default)]
struct IbisBuilder {
    definitions: String,
    trait_impls: String,
    atoms: String,
}

impl IbisBuilder {
    fn add_atom(&mut self, name: TokenTree) {
        // This is an atom definition;
        let lower_name = format!("{}", name).to_lowercase();
        self.atoms += &format!(
            "static {lower_name}: Ent = Ent::by_name(\"{name}\");",
            lower_name = lower_name,
            name = name
        );
    }
    fn add_rule(&mut self, name: TokenTree, args: TokenTree, definition: &mut Vec<TokenTree>) {
        match definition.pop() {
            Some(Punct(ch)) => {
                if ch != '<' {
                    panic!("Parse error: expected <-");
                }
            }
            Some(token) => {
                panic!("Parse error: unexpected {:?} (1)", token)
            }
            None => {
                panic!("Parse error: unexpected EOL (1)")
            }
        }
        match definition.pop() {
            Some(Punct(ch)) => {
                if ch != '-' {
                    panic!("Parse error: expected -");
                }
            }
            Some(token) => {
                panic!("Parse error: unexpected {:?} (2)", token)
            }
            None => {
                panic!("Parse error: unexpected EOL (2)")
            }
        }
        definition.reverse();
        // panic!("name: {}, args: {}, tail: {:?}", name, args, definition);
        // this is a rule definition
        self.definitions += &format!(
            "
        {name}{args} <- {tail};
        ",
            name = name,
            args = args,
            tail = TokenStream::from_iter(definition.iter().cloned())
        );
    }

    fn add_relation(&mut self, name: TokenTree, args: TokenTree) {
            let mut arity = 0;
            let mut arg_names: Vec<String> = vec![];

            let mut curr_arg = false; // we're not currently in an arg
            let mut make_arg = || {
                arity += 1;
                arg_names.push(format!("arg{}", arity));
            };

            let mut new_args = vec![];
            match &args {
                Group(stream) => {
                    for token in stream.stream() {
                        match token {
                            Punct(ch) => {
                                if ch == ',' {
                                    make_arg();
                                    curr_arg = false; // we're not currently in an arg
                                    continue;
                                }
                            }
                            arg => {
                                let arg = format!("{}", &arg);
                                if &arg == "Sol" {
                                    new_args.push("#[serde(skip, default)]Sol".to_string());
                                } else {
                                    new_args.push(arg);
                                }
                            }
                        }
                        curr_arg = true;
                    }
                }
                _ => {
                    panic!("expected {} to be a group", args);
                }
            }
            if curr_arg {
                // finish the arg
                make_arg();
            }

            let name = format!("{}", name);
            let claim_name = if name.ends_with("Input") {
                name.clone()
            } else {
                format!("{}Input", &name)
            };

            if name != claim_name {
                self.trait_impls += &format!(
"
impl ToInput for {name} {{
    type U = {claim_name};
    fn to_claim(self) -> {claim_name} {{
        let {name}({arg_names}) = self;
        {claim_name}({arg_names})
    }}
}}
impl Extend<{name}> for Ibis {{
    fn extend<Iter: IntoIterator<Item={name}> >(&mut self, data: Iter) {{
        self.inner.extend(data.into_iter().map(|datum|datum.to_claim()));
    }}
}}

",
                    name = name,
                    claim_name = claim_name,
                    arg_names = arg_names.join(", ")
                );
            }
            self.trait_impls += &format!(
"
impl ToInput for {claim_name} {{
    type U = {claim_name};
    fn to_claim(self) -> {claim_name} {{
        self
    }}
}}
impl Extend<{claim_name}> for Ibis {{
    fn extend<Iter: IntoIterator<Item={claim_name}> >(&mut self, data: Iter) {{
        self.inner.extend(data.into_iter());
    }}
}}

",
                claim_name = claim_name
            );

            // this is a struct definition
            self.definitions += &format!(
"
    @input
    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct {name}Input({args});
    @output

    #[derive(Debug, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct {name}({args});

    {name}({arg_names}) <- {claim_name}({arg_names});",
                name = name,
                claim_name = claim_name,
                args = new_args.join(", "),
                arg_names = arg_names.join(", ")
            );

    }

    fn add_definition(&mut self, definition: &mut Vec<TokenTree>) {
        if definition.is_empty() {
            return;
        }
        definition.reverse();
        let name = definition.pop().expect("Definition must have a name");
        if definition.is_empty() {
            return self.add_atom(name);
        }
        let args = definition.pop().expect("Definition must have args");
        if definition.is_empty() {
            return self.add_relation(name, args);
        }
        self.add_rule(name, args, definition);
    }

    fn build(self) -> TokenStream {
        format!(
"
use crepe::crepe;
crepe!{{
{definitions}
}}
pub struct Ibis {{
    inner: Crepe,
}}
{trait_impls}
{atoms}
",
            definitions = self.definitions,
            atoms = self.atoms,
            trait_impls = self.trait_impls,
        )
        .parse()
        .unwrap()
    }
}

#[proc_macro]
pub fn ibis(input: TokenStream) -> TokenStream {
    let mut curr = Vec::new();
    let mut builder = IbisBuilder::default();

    for token in input {
        let is_semi = match &token {
            Punct(ch) => *ch == ';',
            _ => false,
        };
        if is_semi {
            builder.add_definition(&mut curr);
            curr = Vec::new();
        } else {
            curr.push(token);
        }
    }
    // also add the last definition
    builder.add_definition(&mut curr);
    builder.build()
}
