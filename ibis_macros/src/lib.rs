extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree, TokenTree::*};

#[proc_macro]
pub fn ibis(input: TokenStream) -> TokenStream {
    let mut curr = Vec::new();

    let mut definitions = "".to_string();
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
            let claim_name = if name.ends_with("Claim") {
                name.clone()
            } else {
                format!("{}Claim", &name)
            };

            // this is a struct definition
            definitions += &format!("
            @input
            #[derive(Debug, Ord, PartialOrd)]
            struct {name}Claim{args};
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
                _ => { panic!("Parse error") }
            }
            match definition.pop() {
                Some(Punct(ch)) => {
                    if ch != '-' {
                        panic!("Parse error: expected -");
                    }
                },
                _ => { panic!("Parse error") }
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
    {atoms}
    ", definitions=definitions, atoms=atoms).parse().unwrap()
}
