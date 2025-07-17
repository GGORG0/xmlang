import { defineConfig } from 'vite';
import { exec } from 'node:child_process';
import { viteStaticCopy } from 'vite-plugin-static-copy';

const wasmContentTypePlugin = {
    name: 'wasm-content-type-plugin',
    configureServer(server) {
        server.middlewares.use((req, res, next) => {
            if (req.url.endsWith('.wasm')) {
                res.setHeader('Content-Type', 'application/wasm');
            }
            next();
        });
    },
};

const cargoBuildPlugin = {
    name: 'cargo-build',
    buildStart: () => {
        return new Promise<void>((resolve, reject) => {
            exec(
                'cargo build --target=wasm32-wasip1 --manifest-path=../Cargo.toml --release --quiet',
                (err, stdout, stderr) => {
                    if (err) {
                        console.log('Stdout:', stdout);
                        console.log('Stderr:', stderr);
                        reject(err);
                    } else {
                        console.log('Rebuilt WASM module successfully.');
                        resolve();
                    }
                }
            );
        });
    },
};

const mdbookBuildPlugin = {
    name: 'mdbook-build',
    buildStart: () => {
        return new Promise<void>((resolve, reject) => {
            exec(
                'mdbook build ..',
                { env: { ...process.env, RUST_LOG: 'error' } },
                (err, stdout, stderr) => {
                    if (err) {
                        console.log('Stdout:', stdout);
                        console.log('Stderr:', stderr);
                        reject(err);
                    } else {
                        console.log('Rebuilt docs successfully.');
                        resolve();
                    }
                }
            );
        });
    },
};

const staticCopyPlugin = viteStaticCopy({
    targets: [
        {
            src: 'node_modules/coi-serviceworker/coi-serviceworker.min.js',
            dest: '.',
        },
        {
            src: '../book',
            dest: '.',
            rename: 'docs',
        },
    ],
});

export default defineConfig({
    server: {
        headers: {
            'Cross-Origin-Opener-Policy': 'same-origin',
            'Cross-Origin-Embedder-Policy': 'require-corp',
        },
        fs: {
            allow: ['..'],
        },
    },
    plugins: [
        wasmContentTypePlugin,
        cargoBuildPlugin,
        mdbookBuildPlugin,
        staticCopyPlugin,
    ],
    optimizeDeps: {
        exclude: ['@wasmer/sdk'],
    },
    base: './',
});
