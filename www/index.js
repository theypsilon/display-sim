const ui = document.getElementById('ui');
const form = document.getElementById('form');
const loading = document.getElementById('loading');
const input = document.getElementById('file');
const startCustom = document.getElementById('start-custom');
const startAnimation = document.getElementById('start-animation');
const antialias = document.getElementById('antialias');
const scaleX = document.getElementById('scale-x');
const scaleY = document.getElementById('scale-y');
const scaleCustomInputs = document.getElementById('scale-custom-inputs');
const dropZone = document.getElementById('drop-zone');
const worker = new Worker("worker.js");
const infoHide = document.getElementById('info-hide');
const infoPanel = document.getElementById('info-panel');
const fpsCounter = document.getElementById('fps-counter');

const cameraPosX = document.getElementById('camera-pos-x');
const cameraPosY = document.getElementById('camera-pos-y');
const cameraPosZ = document.getElementById('camera-pos-z');
const cameraDirX = document.getElementById('camera-dir-x');
const cameraDirY = document.getElementById('camera-dir-y');
const cameraDirZ = document.getElementById('camera-dir-z');
const cameraAxisUpX = document.getElementById('camera-axis-up-x');
const cameraAxisUpY = document.getElementById('camera-axis-up-y');
const cameraAxisUpZ = document.getElementById('camera-axis-up-z');

const pixelScaleX = document.getElementById('pixel-scale-x');
const pixelScaleY = document.getElementById('pixel-scale-y');
const pixelGap = document.getElementById('pixel-gap');

// SETTING UP STATIC EVENT HANDLERS

input.onchange = () => {
    const file = input.files[0];
    const url = (window.URL || window.webkitURL).createObjectURL(file);
    worker.postMessage({url: url});
    console.log(new Date().toISOString(), "message sent to worker");
};

worker.onmessage = (event) => {
    console.log(new Date().toISOString(), "worker replied");
    const previewUrl = URL.createObjectURL(event.data.blob);
    const img = new Image();
    img.src = previewUrl;
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

window.ondrop = event => {
    event.preventDefault();
};

window.ondragover = event => {
    event.preventDefault();
    event.dataTransfer.dropEffect = 'none';
};

window.addEventListener('app-event.toggle_info_panel', () => {
    if (document.getElementById('gl-canvas')) {
        if (infoPanel.classList.contains('display-none')) {
            infoPanel.classList.remove('display-none');
        } else {
            infoPanel.classList.add('display-none');
            window.dispatchEvent(new CustomEvent('app-event.top_message', {
                detail: "Toggle the Info Panel by pressing SPACE."
            }));
        }
    }
}, false);

window.addEventListener('app-event.exit_pointer_lock', () => {
    document.exitPointerLock();
}, false);

window.addEventListener('app-event.exiting_session', () => {
    prepareUi();
    document.getElementById('gl-canvas').remove();
    infoPanel.classList.add("display-none");
}, false);

window.addEventListener('app-event.fps', event => {
    fpsCounter.innerHTML = Math.round(event.detail);
}, false);

window.addEventListener('app-event.top_message', event => {
    const existingTopMessage = document.getElementById('top-message');
    if (existingTopMessage) {
        existingTopMessage.remove();
    }
    const div = document.createElement('div');
    div.id = 'top-message';
    const span = document.createElement('span');
    span.innerHTML = event.detail;
    div.appendChild(span);
    document.body.appendChild(div);
    let opacity = 0.75;
    div.style.opacity = opacity;
    setTimeout(() => {
        function fade() {
            if (opacity >= 0.01) {
                opacity -= 0.01;
                div.style.opacity = opacity;
                setTimeout(fade, 16);
            } else {
                div.remove();
            }
        }
        fade();
    }, 1000);
}, false);

window.addEventListener('app-event.camera_update', event => {
    cameraPosX.innerHTML = Math.round(event.detail[0] * 100) / 100;
    cameraPosY.innerHTML = Math.round(event.detail[1] * 100) / 100;
    cameraPosZ.innerHTML = Math.round(event.detail[2] * 100) / 100;
    cameraDirX.innerHTML = Math.round(event.detail[3] * 100) / 100;
    cameraDirY.innerHTML = Math.round(event.detail[4] * 100) / 100;
    cameraDirZ.innerHTML = Math.round(event.detail[5] * 100) / 100;
    cameraAxisUpX.innerHTML = Math.round(event.detail[6] * 100) / 100;
    cameraAxisUpY.innerHTML = Math.round(event.detail[7] * 100) / 100;
    cameraAxisUpZ.innerHTML = Math.round(event.detail[8] * 100) / 100;
}, false);

window.addEventListener('app-event.change_pixel_scale_x', event => {
    pixelScaleX.innerHTML = Math.round(event.detail * 1000.0) / 1000.0;
}, false);

window.addEventListener('app-event.change_pixel_scale_y', event => {
    pixelScaleY.innerHTML = Math.round(event.detail * 1000.0) / 1000.0;
}, false);

window.addEventListener('app-event.change_pixel_gap', event => {
    pixelGap.innerHTML = Math.round(event.detail * 1000.0) / 1000.0;
}, false);

infoHide.onclick = () => {
    if (document.getElementById('gl-canvas')) {
        infoPanel.classList.add('display-none');
        window.dispatchEvent(new CustomEvent('app-event.top_message', {
            detail: "Show the Info Panel again by pressing SPACE."
        }));
    }
}
dropZone.onclick = () => {
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
document.form.scale.forEach(s => {
    s.onclick = function() {
        if (this.id === "scale-custom") {
            scaleCustomInputs.classList.remove("display-none");
        } else {
            scaleCustomInputs.classList.add("display-none");
        }
    }
});

prepareUi();

function prepareUi() {

    const scaleId = localStorage.getItem('scale') || "scale-1";
    document.getElementById(scaleId).checked = true;
    if (scaleId === 'scale-custom') {
        scaleCustomInputs.classList.remove("display-none");
    }
    scaleX.value = localStorage.getItem('scale-x') || 1;
    scaleY.value = localStorage.getItem('scale-y') || 1;
    
    antialias.checked = localStorage.getItem('antialias') === "true";
    document.getElementById(localStorage.getItem('powerPreference') || 'powerPreference-1').checked = true;
    
    if (document.getElementById('preview')) {
        startCustom.classList.remove('display-none');
    }
    
    ui.classList.remove('display-none');
    loading.classList.add('display-none');
    
    const startPromise = new Promise((startResolve, startReject) => {
        startCustom.onclick = () => {
            ui.classList.add("display-none");
            loading.classList.remove("display-none");
            setTimeout(() => {
                const img = document.getElementById('preview');
                const canvas = document.createElement('canvas');
                const ctx = canvas.getContext('2d');
                canvas.width = img.width;
                canvas.height = img.height;
                ctx.drawImage(img, 0, 0);
                var rawImg = ctx.getImageData(0, 0, img.width, img.height);
    
                startResolve([rawImg])
            }, 50);
        }
        startAnimation.onclick = () => {
            ui.classList.add("display-none");
            loading.classList.remove("display-none");

            const animationPromise = new Promise((imgResolve, imgReject) => {
                const img = new Image();
                img.src = "assets/wwix_spritesheet.png";
                img.onload = () => {
                    const canvas = document.createElement('canvas');
                    const ctx = canvas.getContext('2d');
                    canvas.width = img.width;
                    canvas.height = img.height;
                    ctx.drawImage(img, 0, 0);
                    const columns = Math.floor(img.width / 256);
                    const rawImgs = [];
                    for (let i = 0; i < 45; i++) {
                        const x = i % columns;
                        const y = Math.floor(i / columns);
                        rawImgs.push(ctx.getImageData(x * 256, y * 224, 256, 224));
                    }
                    imgResolve(rawImgs);
                }
                img.onerror = (e) => imgReject(e);
            });
            animationPromise.then(startResolve);
        }
    });
    
    startPromise.then((rawImgs) => {
        console.log(new Date().toISOString(), "image readed");
        const dpi = window.devicePixelRatio;
        const width = window.screen.width;
        const height = window.screen.height;
    
        const scale = form.querySelector('input[name="scale"]:checked');
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
    
        const canvas = document.createElement("canvas");
    
        canvas.id = 'gl-canvas';

        canvas.width = width * dpi;
        canvas.height = height * dpi;
    
        canvas.style.width = width;
        canvas.style.height = height;

        infoPanel.style.setProperty("max-height", height - 50);
    
        document.body.appendChild(canvas);
    
        const powerPreference = form.querySelector('input[name="powerPreference"]:checked');
    
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
        console.log("gl context form", ctxOptions);
        const gl = canvas.getContext('webgl2', ctxOptions);
    
        var documentElement = document.documentElement;
        documentElement.requestFullscreen = documentElement.requestFullscreen
            || documentElement.webkitRequestFullScreen 
            || documentElement['mozRequestFullScreen'] 
            || documentElement.msRequestFullscreen;
    
        canvas.onmousedown = (e) => {
            if (e.buttons != 1) return;
            canvas.requestPointerLock();
            if (window.screen.width != window.innerWidth && window.screen.height != window.innerHeight) {
                documentElement.requestFullscreen();
            }
        };
    
        canvas.requestPointerLock = canvas.requestPointerLock || canvas.mozRequestPointerLock;
        document.exitPointerLock = document.exitPointerLock || document.mozExitPointerLock;
    
        if (!gl) throw new Error("Could not get webgl context.");
    
        import('./wasm_game_of_life.js').then(module => {
            const animation = new module.Animation_Source(rawImgs[0].width, rawImgs[0].height, width, height, 1 / 60, +scaleX, +scaleY);
            for (let i = 0; i < rawImgs.length; i++) {
                const rawImg = rawImgs[i];
                animation.add(rawImg.data.buffer);
            }
    
            console.log(new Date().toISOString(), "calling wasm main");
            module.main(gl, animation);
            console.log(new Date().toISOString(), "wasm main done");
        
            loading.classList.add("display-none");
            infoPanel.classList.remove("display-none");
        });
    });

}