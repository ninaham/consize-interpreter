# Consize Rust

This is a Rust Implementation of the [Consize programming language](https://github.com/denkspuren/consize). This Implementation contains a pre-processing step to improve the speed of Consize at runtime.

## Usage
Install the Rust toolchain by executing the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After that, you can run the Consize Rust implementation by executing the following command inside the project directory:

```bash
cargo run -- <consize-code>
```

To try the different preprocessing steps, you can use the following flags:

```bash
cargo run -- -l <level> <consize-code>
```

To see the available levels, you can use the following command:

```bash
cargo run -- -h
```

If you do not want to install the rust toolchain, you can download the binary and use the following command to run the Consize Rust implementation:

```bash
path/to/consize-interpreter <consize-code>
```

Note that due to the CLI arguments, the consize code must be within double quotes if it contains spaces or the spaces must be escaped.