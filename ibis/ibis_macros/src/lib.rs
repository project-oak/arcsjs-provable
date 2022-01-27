extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree, TokenTree::*};

#[proc_macro]
pub fn ibis(input: TokenStream) -> TokenStream {
    let mut curr = Vec::new();

    let mut definitions = "".to_string();
    let mut trait_impls = "".to_string();
    let mut atoms = "".to_string();

    let mut add_definition = |mut definition: Vec<TokenTree>| {
        if definition.is_empty() {
            return;
        }

        definition.reverse();
        let name = definition.pop().expect("Definition must have a name");
        if definition.is_empty() {
            // This is an atom definition;
            let lower_name = format!("{}", name).to_lowercase();
            atoms += &format!("let {lower_name} = Ent::by_name(\"{name}\");", lower_name=lower_name, name=name);
            return;
        }
        let args = definition.pop().expect("Definition must have args");

        if definition.is_empty() {
            let mut arity = 0;
            let mut arg_names: Vec<String> = vec![];

            let mut curr_arg = false; // we're not currently in an arg
            let mut make_arg = || {
                arity += 1;
                arg_names.push(format!("arg{}", arity));
            };

            match &args {
                Group (stream) => {
                    for token in stream.stream() {
                        match token {
                            Punct(ch) => {
                                if ch == ',' {
                                    make_arg();
                                    curr_arg = false; // we're not currently in an arg
                                    continue;
                                }
                            },
                            _arg => {}
                        }
                        curr_arg = true;
                    }
                },
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
                trait_impls += &format!("
                    impl ToInput for {name} {{
                        type U = {claim_name};
                        fn to_claim(self) -> {claim_name} {{
                            let {name}({arg_names}) = self;
                            {claim_name}({arg_names})
                        }}
                    }}
                ", name=name, claim_name=claim_name, arg_names=arg_names.join(", "));
            }
            trait_impls += &format!("
                impl ToInput for {claim_name} {{
                    type U = {claim_name};
                    fn to_claim(self) -> {claim_name} {{
                        self
                    }}
                }}
            ", claim_name=claim_name);

            // this is a struct definition
            definitions += &format!("
            @input
            #[derive(Debug, Ord, PartialOrd)]
            struct {name}Input{args};
            @output
            #[derive(Debug, Ord, PartialOrd)]
            struct {name}{args};

            {name}({arg_names}) <- {claim_name}({arg_names});
            ", name=name, claim_name=claim_name, args=args, arg_names=arg_names.join(", "));
        } else {
            match definition.pop() {
                Some(Punct(ch)) => {
                    if ch != '<' {
                        panic!("Parse error: expected <-");
                    }
                },
                Some(token) => { panic!("Parse error: unexpected {:?} (1)", token) },
                None => { panic!("Parse error: unexpected EOL (1)") }
            }
            match definition.pop() {
                Some(Punct(ch)) => {
                    if ch != '-' {
                        panic!("Parse error: expected -");
                    }
                },
                Some(token) => { panic!("Parse error: unexpected {:?} (2)", token) },
                None => { panic!("Parse error: unexpected EOL (2)") }
            }
            definition.reverse();
            // panic!("name: {}, args: {}, tail: {:?}", name, args, definition);
            // this is a rule definition
            definitions += &format!("
            {name}{args} <- {tail};
            ", name=name, args=args, tail=TokenStream::from_iter(definition.iter().cloned()));
        }
    };

    for token in input {
        let is_semi = match &token {
            Punct ( ch ) => *ch == ';',
            _ => false,
        };
        if is_semi {
            add_definition(curr);
            curr = Vec::new();
        } else {
            curr.push(token);
        }
    }
    // also add the last definition
    add_definition(curr);

    format!("use crepe::crepe;
    crepe!{{
        {definitions}
    }};
    type Ibis=Crepe;

    pub trait ToInput {{
        type U;
        fn to_claim(self) -> Self::U;
    }}

    impl <T: ToInput + Clone> ToInput for &T {{
        type U = T::U;

        fn to_claim(self) -> Self::U {{
            self.clone().to_claim()
        }}
    }}

    impl Crepe {{
        // TODO: Remove clone requirement here
        fn add_data<T: ToInput, Iter: IntoIterator<Item=T>>(&mut self, data: Iter) where Crepe: Extend<T::U> {{
            self.extend(data.into_iter().map(|datum|datum.to_claim()));
        }}
    }}

    {trait_impls}

    {atoms}
    ", definitions=definitions, atoms=atoms, trait_impls=trait_impls).parse().unwrap()
}