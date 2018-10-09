import {Animation_Source, main} from 'wasm-game-of-life'

const ui = document.getElementById('ui');
const options = document.getElementById('options');
const loading = document.getElementById('loading');
const input = document.getElementById('file');
const startCustom = document.getElementById('start-custom');
const antialias = document.getElementById('antialias');
const scaleX = document.getElementById('scale-x');
const scaleY = document.getElementById('scale-y');
const scaleCustomInputs = document.getElementById('scale-custom-inputs');
const dropZone = document.getElementById('drop-zone');
window.ondrop = event => {
    event.preventDefault();
};
window.ondragover = event => {
    event.preventDefault();
};
document.getElementById('file-button').onclick = () => {
    document.getElementById('file').click();
}
dropZone.ondragover = event => {
    console.log(event);
    event.stopPropagation();
    event.preventDefault();
    event.dataTransfer.dropEffect = 'copy';
};
dropZone.ondrop = event => {
    event.stopPropagation();
    event.preventDefault();
    var file = event.dataTransfer.files[0];
    const url = (window.URL || window.webkitURL).createObjectURL(file);
    worker.postMessage({url: url});
    console.log(new Date().toISOString(), "message sent to worker from drop");
};
document.options.scale.forEach(s => {
    s.onclick = function() {
        if (this.id === "scale-custom") {
            if (scaleCustomInputs.style.display === "none") {
                scaleCustomInputs.style.display = "initial";
            }
        } else {
            scaleCustomInputs.style.display = "none";
        }
    }
});

const scaleId = localStorage.getItem('scale') || "scale-1";
document.getElementById(scaleId).checked = true;
if (scaleId === 'scale-custom') {
    scaleCustomInputs.style.display = "initial";
}
scaleX.value = localStorage.getItem('scale-x') || 1;
scaleY.value = localStorage.getItem('scale-y') || 1;

antialias.checked = localStorage.getItem('antialias') === "true";
document.getElementById(localStorage.getItem('powerPreference') || 'powerPreference-1').checked = true;

input.onchange = () => {
    const file = input.files[0];
    const url = (window.URL || window.webkitURL).createObjectURL(file);
    worker.postMessage({url: url});
    console.log(new Date().toISOString(), "message sent to worker");
};
ui.style.display = "block";

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
    startCustom.onclick = () => {
        ui.style.display = "none";
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
            dropZone.innerHTML = "";
            dropZone.appendChild(img);
            console.log(new Date().toISOString(), "image loaded");
            startCustom.disabled = false;
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

    const scale = options.querySelector('input[name="scale"]:checked');
    let scaleX = 1;
    let scaleY = 1;
    localStorage.setItem("scale", scale.id);
    if (scale.id == "scale-custom") {
        scaleX = document.getElementById('scale-x').value;
        scaleY = document.getElementById('scale-y').value;
        localStorage.setItem("scale-x", scaleX);
        localStorage.setItem("scale-y", scaleY);
    } else {
        scaleX = scale.value;
    }

    const animation = new Animation_Source(rawImg.width, rawImg.height, width, height, 1 / 60, +scaleX, +scaleY);
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
        antialias: antialias.checked, 
        depth: true, 
        failIfMajorPerformanceCaveat: false, 
        powerPreference: powerPreference.value,
        premultipliedAlpha: false, 
        preserveDrawingBuffer: false, 
        stencil: false 
    };
    localStorage.setItem('antialias', ctxOptions.antialias ? "true" : "false");
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