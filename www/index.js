import {Animation_Source, main} from 'wasm-game-of-life'

const options = document.getElementById('options');
const loading = document.getElementById('loading');
const input = document.getElementById('file');
const start = document.getElementById('start');

const antialias = document.getElementById('antialias');
antialias.checked = localStorage.getItem('antialias') || false;
document.getElementById(localStorage.getItem('powerPreference') || 'powerPreference-1').checked = true;

input.onchange = () => {
    const file = input.files[0];
    const url = (window.URL || window.webkitURL).createObjectURL(file);
    worker.postMessage({url: url});
    console.log(new Date().toISOString(), "message sent to worker");
};
options.style.display = "initial";

const worker = new Worker("worker.js");
const promise = new Promise((resolve, reject) => {
    worker.onmessage = (event) => {
        console.log(new Date().toISOString(), "worker replied");
        const previewUrl = URL.createObjectURL(event.data.blob);
        loadImage(previewUrl);
    }
    worker.onerror = (e) => {
        reject(e);
    }
    start.onclick = () => {
        options.style.display = "none";
        loading.style.display = "initial";
        setTimeout(() => {
            const img = document.getElementById('preview');
            const canvas = document.createElement('canvas');
            const ctx = canvas.getContext('2d');
            canvas.width = img.width;
            canvas.height = img.height;
            ctx.drawImage(img, 0, 0);
            var rawImg = ctx.getImageData(0, 0, img.width, img.height);

            resolve(rawImg)
        }, 0);
    }

    function loadImage(url) {
        const img = new Image();
        img.src = url;
        console.log(new Date().toISOString(), "encoded");
        img.onload = () => {
            const preview = document.getElementById('preview');
            if (preview) {
                preview.remove();
            }
            img.id = 'preview';
            if (img.width > img.height) {
                img.style.width = "100px";
            } else {
                img.style.height = "100px";
            }
            options.appendChild(img);
            console.log(new Date().toISOString(), "image loaded");
            start.disabled = false;
        }
    }
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
    console.log(new Date().toISOString(), "image readed");
    const dpi = window.devicePixelRatio;
    const width = window.screen.width;
    const height = window.screen.height;

    const scale_x = rawImg.width == 256 && rawImg.height == 224 ? 256 / 224 : 1;
    const animation = new Animation_Source(rawImg.width, rawImg.height, width, height, 1 / 60, scale_x, 1);
    animation.add(rawImg.data.buffer);

    const canvas = document.createElement("canvas");

    canvas.width = width * dpi;
    canvas.height = height * dpi;

    canvas.style.width = width;
    canvas.style.height = height;

    document.body.appendChild(canvas);

    const powerPreference = options.querySelector('input[name="powerPreference"]:checked');

    const ctxOptions = { 
        alpha: true, 
        antialias: antialias.value, 
        depth: true, 
        failIfMajorPerformanceCaveat: false, 
        powerPreference: powerPreference.value,
        premultipliedAlpha: true, 
        preserveDrawingBuffer: false, 
        stencil: false 
    };
    localStorage.setItem('antialias', ctxOptions.antialias);
    localStorage.setItem('powerPreference', powerPreference.id);
    console.log(ctxOptions);
    const gl = canvas.getContext('webgl2', ctxOptions);

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

    console.log(new Date().toISOString(), "calling wasm main");
    main(gl, animation);
    console.log(new Date().toISOString(), "wasm main done");

    loading.style.display = "none";
});