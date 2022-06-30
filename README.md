# image-rust-wasm
Rust/WebAssembly image processing demonstration 🦀⚡

- Web Workers and SharedArrayBuffer support
- WebAssembly thread support ([Browsers](https://webassembly.org/roadmap/))
- [cross-origin isolation policies](https://web.dev/coop-coep/) enabled (using [serve](https://www.npmjs.com/package/serve))
- Use [Comlink](https://github.com/GoogleChromeLabs/comlink) to expose required wasm-bindgen methods to the main thread

### Building
```bash
yarn build
```

### Start
```bash
yarn start
```

