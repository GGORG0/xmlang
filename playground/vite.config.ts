import { defineConfig } from "vite";
import { exec } from "node:child_process";

const wasmContentTypePlugin = {
  name: "wasm-content-type-plugin",
  configureServer(server) {
    server.middlewares.use((req, res, next) => {
      if (req.url.endsWith(".wasm")) {
        res.setHeader("Content-Type", "application/wasm");
      }
      next();
    });
  },
};

const cargoBuildPlugin = {
  name: "cargo-build",
  buildStart: () => {
    return new Promise<void>((resolve, reject) => {
      exec(
        "cargo build --target=wasm32-wasip1 --manifest-path=../Cargo.toml --release --quiet",
        (err, stdout, stderr) => {
          if (err) {
            console.log("Stdout:", stdout);
            console.log("Stderr:", stderr);
            reject(err);
          } else {
            console.log("Rebuilt WASM module successfully.");
            resolve();
          }
        }
      );
    });
  },
};

export default defineConfig({
  server: {
    headers: {
      "Cross-Origin-Opener-Policy": "same-origin",
      "Cross-Origin-Embedder-Policy": "require-corp",
    },
  },
  plugins: [wasmContentTypePlugin, cargoBuildPlugin],
  optimizeDeps: {
    exclude: ["@wasmer/sdk"],
  },
});
