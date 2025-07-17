# Local installation

The XMLang interpreter is written in Rust, and can be run locally on your machine.

## GitHub Actions

[![build native binaries](https://github.com/GGORG0/xmlang/actions/workflows/native.yml/badge.svg)](https://github.com/GGORG0/xmlang/actions/workflows/native.yml)

The interpreter is built automatically on every commit using GitHub Actions.

You can get the latest built executables for your platform from [this link](https://nightly.link/GGORG0/xmlang/workflows/native/master?preview).

Most likely you'll want to download these files:

- Windows:
    - [x86_64](https://nightly.link/GGORG0/xmlang/workflows/native/master/xmlang-x86_64-pc-windows-msvc)
    - [x86](https://nightly.link/GGORG0/xmlang/workflows/native/master/xmlang-i686-pc-windows-msvc)
    - [x86_64 (GNU)](https://nightly.link/GGORG0/xmlang/workflows/native/master/xmlang-x86_64-pc-windows-gnu)
- [MacOS x86_64](https://nightly.link/GGORG0/xmlang/workflows/native/master/xmlang-x86_64-apple-darwin)
- Linux:
    - [x86_64](https://nightly.link/GGORG0/xmlang/workflows/native/master/xmlang-x86_64-unknown-linux-musl)
    - [aarch64](https://nightly.link/GGORG0/xmlang/workflows/native/master/xmlang-aarch64-unknown-linux-gnu)

## Building from source

1. [Install Rust](https://rustup.rs/).
2. Clone the repository:

    ```bash
    git clone https://github.com/GGORG0/xmlang.git
    ```

3. Change to the project directory:

    ```bash
    cd xmlang
    ```

4. Build the project:

    ```bash
    cargo build --release
    ```

5. The built executable will be located in the `target/release` directory.
    You can run it with:

    ```bash
    ./target/release/xmlang examples/hello.xml
    ```
