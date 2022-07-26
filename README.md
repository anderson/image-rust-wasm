# image-rust-wasm
Rust/WebAssembly image processing demonstration 🦀⚡

- Web Workers and SharedArrayBuffer support
- WebAssembly thread support ([browsers](https://webassembly.org/roadmap/))
- [cross-origin isolation policies](https://web.dev/coop-coep/) enabled (using [serve](https://www.npmjs.com/package/serve))
- Use [Comlink](https://github.com/GoogleChromeLabs/comlink) to expose required wasm-bindgen methods to the main thread

For a live demo, check out https://nanos.red/image-rust-wasm/.

### Building
```bash
yarn build
```

### Start
```bash
yarn start
```

