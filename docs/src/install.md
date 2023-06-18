# Installing repeto

You can install **repeto** in a few different ways, depending on your needs:

### Rust library

Currently, repeto **isn't** available as a package on [crates.io](https://crates.io). However, you can instruct Cargo to
depend on it via the GitHub repository. Here's how:

```toml
repeto = { git = "https://github.com/nucleohub/repeto", rev = "e56c83d" }
```

In this example, `rev` is the desired git commit hash. Note that the minimum supported version of Rust is 1.68, an
earlier versions may work but have not been tested.

### Python bindings

For Linux distributions, you can install pre-built Python 3.7 bindings for repeto directly using pip:

```shell
python -m pip install repeto
```

If you're using a different platform or interpreter, pip will try to build repeto from the source code.
To do this, you'll need a working Rust/Cargo toolchain ([link](https://www.rust-lang.org/tools/install)). 
Note that the minimum supported Python version is 3.7.
