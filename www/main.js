const scalingAutoHtmlId = 'scale-auto';
const scalingCustomHtmlId = 'scale-custom';
const scalingStretchHtmlId = 'scale-stretch';
const glCanvasHtmlId = 'gl-canvas';
const topMessageHtmlId = 'top-message';
const previewHtmlId = 'preview';

const scalingHtmlName = 'scale';
const powerPreferenceHtmlName = 'powerPreference';

const uiDeo = document.getElementById('ui');
const formDeo = document.getElementById('form');
const loadingDeo = document.getElementById('loading');
const inputFileUploadDeo = document.getElementById('file');
const startCustomDeo = document.getElementById('start-custom');
const startAnimationDeo = document.getElementById('start-animation');
const antialiasDeo = document.getElementById('antialias');
const scalingCustomXDeo = document.getElementById('scale-x');
const scalingCustomYDeo = document.getElementById('scale-y');
const scaleCustomInputsDeo = document.getElementById('scale-custom-inputs');
const dropZoneDeo = document.getElementById('drop-zone');

const infoHideDeo = document.getElementById('info-hide');
const infoPanelDeo = document.getElementById('info-panel');
const fpsCounterDeo = document.getElementById('fps-counter');

const cameraPosXDeo = document.getElementById('camera-pos-x');
const cameraPosYDeo = document.getElementById('camera-pos-y');
const cameraPosZDeo = document.getElementById('camera-pos-z');
const cameraDirXDeo = document.getElementById('camera-dir-x');
const cameraDirYDeo = document.getElementById('camera-dir-y');
const cameraDirZDeo = document.getElementById('camera-dir-z');
const cameraAxisUpXDeo = document.getElementById('camera-axis-up-x');
const cameraAxisUpYDeo = document.getElementById('camera-axis-up-y');
const cameraAxisUpZDeo = document.getElementById('camera-axis-up-z');

const pixelScaleXDeo = document.getElementById('pixel-scale-x');
const pixelScaleYDeo = document.getElementById('pixel-scale-y');
const pixelGapDeo = document.getElementById('pixel-gap');

const getGlCanvasDeo = () => document.getElementById(glCanvasHtmlId);
const getPreviewDeo = () => document.getElementById(previewHtmlId);
const getTopMessageDeo = () => document.getElementById(topMessageHtmlId);
const getScalingSelectionDeo = scalingSelectionIdPostfix => document.getElementById('scale-' + scalingSelectionIdPostfix);

const visibility = makeVisibility();
const storage = makeStorage();

window.ondrop = event => {
    event.preventDefault();
};

window.ondragover = event => {
    event.preventDefault();
    event.dataTransfer.dropEffect = 'none';
};

window.addEventListener('app-event.toggle_info_panel', () => {
    if (!getGlCanvasDeo()) {
        return;
    }
    if (visibility.isInfoPanelVisible() === false) {
        visibility.showInfoPanel();
    } else {
        visibility.hideInfoPanel();
        window.dispatchEvent(new CustomEvent('app-event.top_message', {
            detail: 'Toggle the Info Panel by pressing SPACE.'
        }));
    }
}, false);

window.addEventListener('app-event.exit_pointer_lock', () => {
    document.exitPointerLock();
}, false);

window.addEventListener('app-event.exiting_session', () => {
    prepareUi();
    getGlCanvasDeo().remove();
    visibility.hideInfoPanel();
}, false);

window.addEventListener('app-event.fps', event => {
    fpsCounterDeo.innerHTML = Math.round(event.detail);
}, false);

window.addEventListener('app-event.top_message', event => {
    const existingTopMessage = getTopMessageDeo();
    if (existingTopMessage) {
        existingTopMessage.remove();
    }
    const div = document.createElement('div');
    div.id = topMessageHtmlId;
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
    cameraPosXDeo.innerHTML = Math.round(event.detail[0] * 100) / 100;
    cameraPosYDeo.innerHTML = Math.round(event.detail[1] * 100) / 100;
    cameraPosZDeo.innerHTML = Math.round(event.detail[2] * 100) / 100;
    cameraDirXDeo.innerHTML = Math.round(event.detail[3] * 100) / 100;
    cameraDirYDeo.innerHTML = Math.round(event.detail[4] * 100) / 100;
    cameraDirZDeo.innerHTML = Math.round(event.detail[5] * 100) / 100;
    cameraAxisUpXDeo.innerHTML = Math.round(event.detail[6] * 100) / 100;
    cameraAxisUpYDeo.innerHTML = Math.round(event.detail[7] * 100) / 100;
    cameraAxisUpZDeo.innerHTML = Math.round(event.detail[8] * 100) / 100;
}, false);

window.addEventListener('app-event.change_pixel_scale_x', event => {
    pixelScaleXDeo.innerHTML = Math.round(event.detail * 1000.0) / 1000.0;
}, false);

window.addEventListener('app-event.change_pixel_scale_y', event => {
    pixelScaleYDeo.innerHTML = Math.round(event.detail * 1000.0) / 1000.0;
}, false);

window.addEventListener('app-event.change_pixel_gap', event => {
    pixelGapDeo.innerHTML = Math.round(event.detail * 1000.0) / 1000.0;
}, false);

infoHideDeo.onclick = () => {
    if (!getGlCanvasDeo()) {
        return;
    }
    visibility.hideInfoPanel();
    window.dispatchEvent(new CustomEvent('app-event.top_message', {
        detail: 'Show the Info Panel again by pressing SPACE.'
    }));
}
inputFileUploadDeo.onchange = () => {
  const file = inputFileUploadDeo.files[0];
  const url = (window.URL || window.webkitURL).createObjectURL(file);
  processFileToUpload(url);
};
dropZoneDeo.onclick = () => {
    inputFileUploadDeo.click();
}
dropZoneDeo.ondragover = event => {
    event.stopPropagation();
    event.preventDefault();
    event.dataTransfer.dropEffect = 'copy';
};
dropZoneDeo.ondrop = event => {
    event.stopPropagation();
    event.preventDefault();
    var file = event.dataTransfer.files[0];
    const url = (window.URL || window.webkitURL).createObjectURL(file);
    processFileToUpload(url);
};
document.form[scalingHtmlName].forEach(s => {
    s.onclick = function() {
        if (this.id === scalingCustomHtmlId) {
            visibility.showScaleCustomInputs();
        } else {
            visibility.hideScaleCustomInputs();
        }
    }
});

prepareUi();

function prepareUi() {
    const scalingSelectionInput = storage.getScalingInputElement();
    scalingSelectionInput.checked = true;
    const scalingSelectionId = scalingSelectionInput.id;
    if (scalingSelectionId === scalingCustomHtmlId) {
        visibility.showScaleCustomInputs();
    }
    scalingCustomXDeo.value = storage.getCustomScalingX();
    scalingCustomYDeo.value = storage.getCustomScalingY();
    
    antialiasDeo.checked = storage.getAntiAliasing();
    const powerPreferenceInput = storage.getPowerPreferenceInputElement();
    powerPreferenceInput.checked = true;
    
    visibility.showUi();
    visibility.hideLoading();
    
    const startPromise = new Promise((startResolve, startReject) => {
        startCustomDeo.onclick = () => {
            visibility.hideUi();
            visibility.showLoading();
            setTimeout(() => {
                const img = getPreviewDeo();
                const canvas = document.createElement('canvas');
                const ctx = canvas.getContext('2d');
                canvas.width = img.width;
                canvas.height = img.height;
                ctx.drawImage(img, 0, 0);
                var rawImg = ctx.getImageData(0, 0, img.width, img.height);
    
                startResolve([rawImg])
            }, 50);
        }
        startAnimationDeo.onclick = () => {
            visibility.hideUi();
            visibility.showLoading();

            const animationPromise = new Promise((imgResolve, imgReject) => {
                const img = new Image();
                img.src = 'assets/wwix_spritesheet.png';
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
        console.log(new Date().toISOString(), 'image readed');
        const dpi = window.devicePixelRatio;
        const width = window.screen.width;
        const height = window.screen.height;
    
        const checkedScalingInput = formDeo.querySelector('input[name=\''+scalingHtmlName+'\']:checked');
        let scaleX = 1;
        let stretch = false;
        storage.setScalingId(checkedScalingInput.id);

        const imageWidth = rawImgs[0].width;
        const imageHeight = rawImgs[0].height;
        switch(checkedScalingInput.id) {
            case scalingAutoHtmlId:
                let scalingSelectionIdPostfix = 'none';
                if (imageWidth == 256 && imageHeight == 224) {
                    scalingSelectionIdPostfix = '256x224';
                } else if (imageWidth == 256 && imageHeight == 240) {
                    scalingSelectionIdPostfix = '256x240';
                } else if (imageWidth == 320 && imageHeight == 224) {
                    scalingSelectionIdPostfix = '320x224';
                } else if (imageWidth == 160 && imageHeight == 144) {
                    scalingSelectionIdPostfix = '160x144';
                } else if (imageWidth == 240 && imageHeight == 160) {
                    scalingSelectionIdPostfix = '240x160';
                } else if (imageWidth == 320 && imageHeight == 200) {
                    scalingSelectionIdPostfix = '320x200';
                }
                scaleX = getScalingSelectionDeo(scalingSelectionIdPostfix).value;
                window.dispatchEvent(new CustomEvent('app-event.top_message', {
                    detail: 'Scaling auto detect applying: ' + scalingSelectionIdPostfix + (scalingSelectionIdPostfix == 'none' ? '' : ' on 4:3')
                }));
                break;
            case scalingStretchHtmlId:
                scaleX = (width/height)/(imageWidth/imageHeight);
                stretch = true;
                break;
            case scalingCustomHtmlId:
                scaleX = scalingCustomXDeo.value;
                const scaleY = scalingCustomYDeo.value;
                storage.setCustomScalingX(scaleX);
                storage.setCustomScalingY(scaleY);
                scaleX = +scaleX / +scaleY;
                break;
            default:
                scaleX = checkedScalingInput.value;
                break;
        }
    
        const canvas = document.createElement('canvas');
    
        canvas.id = glCanvasHtmlId;

        canvas.width = Math.floor(width * dpi / 80) * 80;
        canvas.height = Math.floor(height * dpi / 60) * 60;
    
        canvas.style.width = width;
        canvas.style.height = height;

        infoPanelDeo.style.setProperty('max-height', height - 50);
    
        document.body.appendChild(canvas);
    
        const checkedPowerPreferenceInput = formDeo.querySelector('input[name=\''+powerPreferenceHtmlName+'\']:checked');
    
        const ctxOptions = { 
            alpha: true, 
            antialias: antialiasDeo.checked, 
            depth: true, 
            failIfMajorPerformanceCaveat: false, 
            powerPreference: checkedPowerPreferenceInput.value,
            premultipliedAlpha: false, 
            preserveDrawingBuffer: false, 
            stencil: false 
        };
        storage.setAntiAliasing(ctxOptions.antialias);
        storage.setPowerPreference(checkedPowerPreferenceInput.id);
        console.log('gl context form', ctxOptions);
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
    
        if (!gl) {
            window.dispatchEvent(new CustomEvent('app-event.top_message', {
                detail: 'WebGL is not working on your browser, try restarting it! And remember, this works only on a PC with updated browser and graphics drivers.'
            }));
            console.error(new Error('Could not get webgl context.'));
            canvas.remove();
            prepareUi();
            return;
        }

        import(/* webpackPrefetch: true */'./crt_3d_sim').then(wasm => {
            const animation = new wasm.Animation_Source(rawImgs[0].width, rawImgs[0].height, canvas.width, canvas.height, 1 / 60, +scaleX, stretch, dpi);
            for (let i = 0; i < rawImgs.length; i++) {
                const rawImg = rawImgs[i];
                animation.add(rawImg.data.buffer);
            }
    
            console.log(new Date().toISOString(), 'calling wasm main');
            wasm.main(gl, animation);
            console.log(new Date().toISOString(), 'wasm main done');
        
            visibility.hideLoading();
            visibility.showInfoPanel();
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
    
    const preview = getPreviewDeo();
    if (preview) {
        preview.remove();
    }
    img.id = previewHtmlId;
    if (img.width > img.height) {
        img.style.width = '100px';
    } else {
        img.style.height = '100px';
    }
    dropZoneDeo.innerHTML = '';
    dropZoneDeo.appendChild(img);
    console.log(new Date().toISOString(), 'image loaded');
    startCustomDeo.disabled = false;
}

function makeStorage() {
    return {
        getScalingInputElement: () => geElementByStoredIdOrBackup('stored-scale', scalingAutoHtmlId),
        setScalingId: (scale) => localStorage.setItem('stored-scale', scale),
        getCustomScalingX: () => localStorage.getItem('stored-scale-x') || 1,
        setCustomScalingX: (scale) => localStorage.setItem('stored-scale-x', scale),
        getCustomScalingY: () => localStorage.getItem('stored-scale-y') || 1,
        setCustomScalingY: (scale) => localStorage.setItem('stored-scale-y', scale),
        getPowerPreferenceInputElement: () => geElementByStoredIdOrBackup('stored-powerPreference', 'powerPreference-1'),
        setPowerPreference: (powerPreference) => localStorage.setItem('stored-powerPreference', powerPreference),
        getAntiAliasing: () => localStorage.getItem('stored-antialias') === 'true',
        setAntiAliasing: (antiAliasing) => localStorage.setItem('stored-antialias', antiAliasing ? 'true' : 'false'),
    };
    function geElementByStoredIdOrBackup(storedId, backupId) {
        return document.getElementById(localStorage.getItem(storedId)) || document.getElementById(backupId);
    }
}

function makeVisibility() {
    return {
        showUi: () => showElement(uiDeo),
        hideUi: () => hideElement(uiDeo),
        showLoading: () => showElement(loadingDeo),
        hideLoading: () => hideElement(loadingDeo),
        showInfoPanel: () => showElement(infoPanelDeo),
        hideInfoPanel: () => hideElement(infoPanelDeo),
        isInfoPanelVisible: () => isVisible(infoPanelDeo),
        showScaleCustomInputs: () => showElement(scaleCustomInputsDeo),
        hideScaleCustomInputs: () => hideElement(scaleCustomInputsDeo),
    };
    function showElement(element) {
        element.classList.remove('display-none');
    }
    function hideElement(element) {
        element.classList.add('display-none');
    }
    function isVisible(element) {
        return element.classList.contains('display-none') == false;
    }
}
