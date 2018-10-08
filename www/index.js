import {Animation_Source, main} from 'wasm-game-of-life'

const input = document.createElement('input');
input.type = "file";
input.onchange = () => {
    const file = input.files[0];
    const url = (window.URL || window.webkitURL).createObjectURL(file);
    worker.postMessage({url: url});
};
document.body.appendChild(input);

const worker = new Worker("worker.js");
const promise = new Promise((resolve, reject) => {
    worker.onmessage = (event) => {
        const img = document.createElement('img');
        img.src = "data:image/png;base64," + encode(event.data.buffer);
        img.onload = () => {
            const canvas = document.createElement('canvas');
            const ctx = canvas.getContext('2d');
            canvas.width = img.width;
            canvas.height = img.height;
            ctx.drawImage(img, 0, 0);
            var rawImg = ctx.getImageData(0, 0, img.width, img.height);
            resolve(rawImg);
            input.remove();
        }
    }
    worker.onerror = (e) => reject(e);
});

/*const promise = new Promise((resolve, reject) => {
    var img = document.createElement('img');
    const timeout = setTimeout(() => {
        reject(new Error('Image took too much to load.'));
    }, 10000);
    img.onload = () => {
        clearTimeout(timeout);
        const canvas = document.createElement('canvas');
        const ctx = canvas.getContext('2d');
        canvas.width = img.width;
        canvas.height = img.height;
        ctx.drawImage(img, 0, 0);
        var rawImg = ctx.getImageData(0, 0, img.width, img.height);
        resolve(rawImg);
    };
    img.src = "assets/wwix_00.png";
});*/

promise.then((rawImg) => {
    console.log(rawImg);
    const scale_x = rawImg.width == 256 && rawImg.height == 224 ? 256 / 224 : 1;
    const animation = new Animation_Source(rawImg.width, rawImg.height, 1 / 60, scale_x, 1);
    animation.add(rawImg.data);

    const dpi = window.devicePixelRatio;
    const width = window.screen.width;
    const height = window.screen.height;

    const canvas = document.getElementById("1-canvas");

    canvas.width = width * dpi;
    canvas.height = height * dpi;

    canvas.style.width = width;
    canvas.style.height = height;

    const gl = canvas.getContext('webgl2', { 
        alpha: true, 
        antialias: true, 
        depth: true, 
        failIfMajorPerformanceCaveat: false, 
        powerPreference: "high-performance",
        premultipliedAlpha: true, 
        preserveDrawingBuffer: false, 
        stencil: false 
    });

    var documentElement = document.documentElement;
    documentElement.requestFullscreen = documentElement.requestFullscreen
        || documentElement.webkitRequestFullScreen 
        || documentElement.mozRequestFullscreen 
        || documentElement.msRequestFullscreen;

    canvas.onmousedown = (e) => {
        if (e.buttons != 1) return;
        canvas.requestPointerLock();
        if (window.screen.width != window.innerWidth && window.screen.height != window.innerHeight) {
            documentElement.requestFullscreen();
        }
    };

    window.addEventListener('exit_pointer_lock', () => {
        document.exitPointerLock();
    }, false);

    canvas.requestPointerLock = canvas.requestPointerLock || canvas.mozRequestPointerLock;
    document.exitPointerLock = document.exitPointerLock || document.mozExitPointerLock;

    if (!gl) throw new Error("Could not get webgl context.");

    main(gl, animation);
});

function encode( buffer ) {
    var binary = '';
    var bytes = new Uint8Array( buffer );
    var len = bytes.byteLength;
    for (var i = 0; i < len; i++) {
        binary += String.fromCharCode( bytes[ i ] );
    }
    return window.btoa( binary );
}