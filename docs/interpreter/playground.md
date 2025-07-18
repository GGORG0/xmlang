# Online playground

The online playground is available at <https://xmlang.ggorg.xyz>.

It automatically saves your code in the browser's local storage, so you don't lose it.

## Loading code from GitHub

**Note:** This will replace the code you saved in the browser's local storage.

### Gist

The playground supports loading code from [GitHub Gist](https://gist.github.com/).

The Gist has to contain a single XML file with your code. If it contains multiple files, the first one will be used.

To load a Gist, get its ID (the part after your username in the URL) and append it to the playground URL like this:

```
https://xmlang.ggorg.xyz/?gist=8da581666cbfd662d6d5fbbe8fce3ca7
```

[The above URL](https://xmlang.ggorg.xyz/?gist=8da581666cbfd662d6d5fbbe8fce3ca7) will load the code from [this Gist](https://gist.github.com/GGORG0/8da581666cbfd662d6d5fbbe8fce3ca7).

### Repository

The playground also supports loading code from a file in a GitHub repository.

To load a file, append the repository owner, name, branch and the path to the file to the playground URL like this:

```
https://xmlang.ggorg.xyz/?owner=GGORG0&repo=xmlang&branch=master&file=examples/hello.xml
```

[The above URL](https://xmlang.ggorg.xyz/?owner=GGORG0&repo=xmlang&branch=master&file=examples/hello.xml) will load the code from [this file](https://github.com/GGORG0/xmlang/blob/master/examples/hello.xml).

## Stack

- WebAssembly (via [Wasmer.js](https://github.com/wasmerio/wasmer-js)) - allows running the XMLang interpreter in the browser inside a full WASI environment with a virtual filesystem.
- [CodeMirror](https://codemirror.net/) - code editor
- [Xterm.js](https://xtermjs.org/) - terminal emulator
- [Vite](https://vitejs.dev/) - build tool

## Building

1. Install the following dependencies:

    - [Node.js](https://nodejs.org/)
    - [Pnpm](https://pnpm.io/) (`corepack enable`)
    - [Rust](https://rustup.rs/) (with the `wasm32-wasip1` target: `rustup target add wasm32-wasip1`) - to build the WebAssembly module
    - [mdBook](https://rust-lang.github.io/mdBook/) (optional) - to build the documentation

2. Clone the repository:

    ```bash
    git clone https://github.com/GGORG0/xmlang.git
    ```

3. Change to the project directory:

    ```bash
    cd xmlang
    ```

4. Build the WebAssembly module (optional - this will be done automatically by Vite during the build):

    ```bash
    cargo build --release --target wasm32-wasip1
    ```

5. Build the documentation (optional):

    ```bash
    mdbook build
    ```

    The documentation will be built in the `book` directory.

6. Change to the `playground` directory:

    ```bash
    cd playground
    ```

7. Install the dependencies:

    ```bash
    pnpm install
    ```

8. Build the project:

    ```bash
    pnpm build
    ```

    Or start the development server:

    ```bash
    pnpm dev
    ```
