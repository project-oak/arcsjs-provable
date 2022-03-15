# Ibis - a knowledge engine for type checking asynchronously communicating modules

[![Ibis](https://github.com/googlestaging/arcsjs-provable/actions/workflows/ibis.yml/badge.svg)](https://github.com/googlestaging/arcsjs-provable/actions/workflows/ibis.yml)

#### This is not an officially supported Google product

# Getting started

Try out [the playground](https://project-oak.github.io/arcsjs-provable/ibis/playground).
(Early) documentation can be found [here](https://project-oak.github.io/arcsjs-provable/ibis/docs/ibis/) thanks to `Rustdoc`.

## Building and running Ibis locally

For the following you'll need [git]() and [cargo](https://rustup.rs).
```
# Assuming git and cargo are already installed
git clone https://github.com/project-oak/arcsjs-provable.git
cd arcsjs-provable/ibis
cat demo.json | cargo run --bin dot > out.dot
```

Ibis also has a test suite that can be run with

```
cargo test
```

### Optional tools & dependencies
- [cargo-wasi](https://bytecodealliance.github.io/cargo-wasi/install.html) (for compilation to WASM)
- [graphviz](https://graphviz.org/download/) (dot command is used to render debugging information)
- [python3](https://docs.python.org/3/using/unix.html#getting-and-installing-the-latest-version-of-python) (using http.server for a development server)

# Contributing

Please see the project's [contributing guide](../contributing.md).

# License

Please see the project's [license](../LICENSE).
