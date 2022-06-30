import * as Comlink from 'comlink';

const imageInput = document.getElementById('imageInput');
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const timeOutput = document.getElementById('time');
const restoreButton = document.getElementById('restoreButton');
const filter = document.getElementById('filter');

var { width, height } = canvas;

imageInput.onchange = () => {
  loadImage();
}

function loadImage() {
  const file = imageInput.files[0];

  if (file) {
    const reader = new FileReader();
    reader.onload = () => {
      const img = new Image();
      img.onload = () => {
        width = canvas.width = img.width;
        height = canvas.height = img.height;
        ctx.drawImage(img, 0, 0, img.width, img.height);
      }
      img.src = reader.result;
    };
    reader.readAsDataURL(file);
  }
}

restoreButton.onclick = () => {
  loadImage();
}

(async function init() {
  let handlers = await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), {
      type: 'module'
    })
  ).handlers;

  function setupButton(id) {
    let handler = handlers[id];

    if (!handler) return;

    Object.assign(document.getElementById(id), {
      async onclick() {
        const imgData = ctx.getImageData(0, 0, width, height);
        const filterOpt = filter.value;

        let { rawImageData, time } = await handler({ imgData, filterOpt });

        const newImgData = new ImageData(rawImageData, width, height);
        timeOutput.value = `${time.toFixed(2)} ms`;

        ctx.putImageData(newImgData, 0, 0);
      },
      disabled: false
    });
  }

  setupButton('singleThread');
  if (await handlers.supportsThreads) {
    setupButton('multiThread');
  }
})();