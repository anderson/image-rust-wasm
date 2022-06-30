import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';

function wrapExports(imageWasm) {
  return ({ imgData, filterOpt }) => {
    const image = imageWasm.openImage(imgData);
    const start = performance.now();

    imageWasm[filterOpt](image);

    const time = performance.now() - start;
    const rawImageData = imageWasm.getRawImageData(image);

    return {
      rawImageData: Comlink.transfer(rawImageData, [rawImageData.buffer]),
      time
    };
  };
}

async function initHandlers() {
  let [singleThread, multiThread] = await Promise.all([
    (async () => {
      const singleThread = await import('./pkg/image_wasm.js');
      await singleThread.default();
      return wrapExports(singleThread);
    })(),
    (async () => {
      if (!(await threads())) return;

      const multiThread = await import(
        './pkg-parallel/image_wasm.js'
      );
      await multiThread.default();
      await multiThread.initThreadPool(navigator.hardwareConcurrency);
      return wrapExports(multiThread);
    })()
  ]);

  return Comlink.proxy({
    singleThread,
    supportsThreads: !!multiThread,
    multiThread
  });
}

Comlink.expose({
  handlers: initHandlers()
});
