# Ibis - a knowledge engine for type checking asynchronously communicating modules

[![Ibis](https://github.com/googlestaging/arcsjs-provable/actions/workflows/ibis.yml/badge.svg)](https://github.com/googlestaging/arcsjs-provable/actions/workflows/ibis.yml)

See the playground [here](https://project-oak.github.io/arcsjs-provable/ibis/playground)

#### This is not an officially supported Google product

# Getting started

For the following you'll need [git]() and [cargo](https://rustup.rs).
```
# Assuming git and cargo are already installed
git clone https://github.com/project-oak/arcsjs-provable.git
cd arcsjs-provable/ibis
cat demo.json | cargo run > out.dot
```

### Optional tools & dependencies
- [cargo-wasi](https://bytecodealliance.github.io/cargo-wasi/install.html) (for compilation to WASM)
- [graphviz](https://graphviz.org/download/) (dot command is used to render debugging information)
- [python3](https://docs.python.org/3/using/unix.html#getting-and-installing-the-latest-version-of-python) (using http.server for a development server)

# Contributing

Please see the project's [contributing guide](../contributing.md).

# License

Please see the project's [license](../LICENSE).
