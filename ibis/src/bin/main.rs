use ibis::IbisError;

fn main() -> Result<(), IbisError> {
    eprintln!("Hello world!");
    println!("digraph ibis {{ a -> b [label=\"Foo\"] b -> a [color=blue][label=\"Nah\"] }}");
    Ok(())
}
