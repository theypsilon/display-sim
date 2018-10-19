const ui = document.getElementById('ui');
const form = document.getElementById('form');
const loading = document.getElementById('loading');
const inputFileUpload = document.getElementById('file');
const startCustom = document.getElementById('start-custom');
const startAnimation = document.getElementById('start-animation');
const antialias = document.getElementById('antialias');
const scaleX = document.getElementById('scale-x');
const scaleY = document.getElementById('scale-y');
const scaleCustomInputs = document.getElementById('scale-custom-inputs');
const dropZone = document.getElementById('drop-zone');

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
    }, event.detail.length * 100);
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
inputFileUpload.onchange = () => {
  const file = inputFileUpload.files[0];
  const url = (window.URL || window.webkitURL).createObjectURL(file);
  processFileToUpload(url);
};
dropZone.onclick = () => {
    document.getElementById('file').click();
}
dropZone.ondragover = event => {
    event.stopPropagation();
    event.preventDefault();
    event.dataTransfer.dropEffect = 'copy';
};
dropZone.ondrop = event => {
    event.stopPropagation();
    event.preventDefault();
    var file = event.dataTransfer.files[0];
    const url = (window.URL || window.webkitURL).createObjectURL(file);
    processFileToUpload(url);
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
    const scaleSelectionInput = document.getElementById(localStorage.getItem('scale')) || document.getElementById("scale-auto");
    scaleSelectionInput.checked = true;
    const scaleSelectionId = scaleSelectionInput.id;
    if (scaleSelectionId === 'scale-custom') {
        scaleCustomInputs.classList.remove("display-none");
    }
    scaleX.value = localStorage.getItem('scale-x') || 1;
    scaleY.value = localStorage.getItem('scale-y') || 1;
    
    antialias.checked = localStorage.getItem('antialias') === "true";
    const powerPreferenceInput = document.getElementById(localStorage.getItem('powerPreference')) || document.getElementById('powerPreference-1');
    powerPreferenceInput.checked = true;
    
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
                    for (let i = 0; i <= 45; i++) {
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
        let stretch = false;
        localStorage.setItem("scale", scale.id);

        const imageWidth = rawImgs[0].width;
        const imageHeight = rawImgs[0].height;
        switch(scale.id) {
            case "scale-auto":
                let scaleSelectionIdPostfix = 'none';
                if (imageWidth == 256 && imageHeight == 224) {
                    scaleSelectionIdPostfix = '256x224';
                } else if (imageWidth == 256 && imageHeight == 240) {
                    scaleSelectionIdPostfix = '256x240';
                } else if (imageWidth == 320 && imageHeight == 224) {
                    scaleSelectionIdPostfix = '320x224';
                } else if (imageWidth == 160 && imageHeight == 144) {
                    scaleSelectionIdPostfix = '160x144';
                } else if (imageWidth == 240 && imageHeight == 160) {
                    scaleSelectionIdPostfix = '240x160';
                } else if (imageWidth == 320 && imageHeight == 200) {
                    scaleSelectionIdPostfix = '320x200';
                }
                scaleX = document.getElementById('scale-' + scaleSelectionIdPostfix).value;
                window.dispatchEvent(new CustomEvent('app-event.top_message', {
                    detail: "Scaling auto detect applying: " + scaleSelectionIdPostfix + (scaleSelectionIdPostfix == "none" ? "" : " on 4:3")
                }));
                break;
            case "scale-stretch":
                scaleX = (width/height)/(imageWidth/imageHeight);
                stretch = true;
                break;
            case "scale-custom":
                scaleX = document.getElementById('scale-x').value;
                const scaleY = document.getElementById('scale-y').value;
                localStorage.setItem("scale-x", scaleX);
                localStorage.setItem("scale-y", scaleY);
                scaleX = +scaleX / +scaleY;
                break;
            default:
                scaleX = scale.value;
                break;
        }
    
        const canvas = document.createElement("canvas");
    
        canvas.id = 'gl-canvas';

        canvas.width = Math.floor(width * dpi / 80) * 80;
        canvas.height = Math.floor(height * dpi / 60) * 60;
    
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

        import(/* webpackPrefetch: true */"./crt_3d_sim").then(wasm => {
            const animation = new wasm.Animation_Source(rawImgs[0].width, rawImgs[0].height, canvas.width, canvas.height, 1 / 60, +scaleX, stretch, dpi);
            for (let i = 0; i < rawImgs.length; i++) {
                const rawImg = rawImgs[i];
                animation.add(rawImg.data.buffer);
            }
    
            console.log(new Date().toISOString(), "calling wasm main");
            wasm.main(gl, animation);
            console.log(new Date().toISOString(), "wasm main done");
        
            loading.classList.add("display-none");
            infoPanel.classList.remove("display-none");
        });
    });
}


async function processFileToUpload(url) {
    var xhr = new XMLHttpRequest();
    xhr.open('GET', url, true);
    xhr.responseType = 'blob';
    xhr.send(null);

    await new Promise(resolve => xhr.onload = () => resolve());

    const previewUrl = URL.createObjectURL(xhr.response);
    const img = new Image();
    img.src = previewUrl;

    await new Promise(resolve => img.onload = () => resolve());
    
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