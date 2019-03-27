const scalingAutoHtmlId = 'scaling-auto';
const scaling43HtmlId = 'scaling-4:3';
const scalingCustomHtmlId = 'scaling-custom';
const scalingStretchToBothEdgesHtmlId = 'scaling-stretch-both';
const scalingStretchToNearestEdgeHtmlId = 'scaling-stretch-nearest';
const powerPreferenceDefaultId = 'powerPreference-1';
const glCanvasHtmlId = 'gl-canvas';
const topMessageHtmlId = 'top-message';
const previewHtmlId = 'preview';

const scalingHtmlName = 'scaling';
const powerPreferenceHtmlName = 'powerPreference';

const uiDeo = document.getElementById('ui');
const formDeo = document.getElementById('form');
const loadingDeo = document.getElementById('loading');
const inputFileUploadDeo = document.getElementById('file');
const startCustomDeo = document.getElementById('start-custom');
const startAnimationDeo = document.getElementById('start-animation');
const antialiasDeo = document.getElementById('antialias');
const scalingCustomResWidthDeo = document.getElementById('scaling-custom-resolution-width');
const scalingCustomResHeightDeo = document.getElementById('scaling-custom-resolution-height');
const scalingCustomResButtonDeo = document.getElementById('scaling-custom-resolution-button');
const scalingCustomArXDeo = document.getElementById('scaling-custom-aspect-ratio-x');
const scalingCustomArYDeo = document.getElementById('scaling-custom-aspect-ratio-y');
const scalingCustomStretchNearestDeo = document.getElementById('scaling-custom-stretch-nearest');
const scalingCustomInputsDeo = document.getElementById('scaling-custom-inputs');
const dropZoneDeo = document.getElementById('drop-zone');
const dropZoneTextDeo = document.getElementById('drop-zone-text');
const previewSizeElementsDeo = document.getElementById('preview-size-elements');
const previewWidthDeo = document.getElementById('preview-width');
const previewHeightDeo = document.getElementById('preview-height');
const restoreDefaultOptionsDeo = document.getElementById('restore-default-options');

const infoHideDeo = document.getElementById('info-hide');
const infoPanelDeo = document.getElementById('info-panel');
const fpsCounterDeo = document.getElementById('fps-counter');
const lightColorDeo = document.getElementById('light-color');
const brightnessColorDeo = document.getElementById('brightness-color');

const cameraPosXDeo = document.getElementById('camera-pos-x');
const cameraPosYDeo = document.getElementById('camera-pos-y');
const cameraPosZDeo = document.getElementById('camera-pos-z');
const cameraDirXDeo = document.getElementById('camera-dir-x');
const cameraDirYDeo = document.getElementById('camera-dir-y');
const cameraDirZDeo = document.getElementById('camera-dir-z');
const cameraAxisUpXDeo = document.getElementById('camera-axis-up-x');
const cameraAxisUpYDeo = document.getElementById('camera-axis-up-y');
const cameraAxisUpZDeo = document.getElementById('camera-axis-up-z');
const cameraZoomDeo = document.getElementById('camera-zoom');

const pixelWidthDeo = document.getElementById('pixel-width');
const pixelHorizontalGapDeo = document.getElementById('pixel-horizontal-gap');
const pixelVerticalGapDeo = document.getElementById('pixel-vertical-gap');
const pixelSpreadDeo = document.getElementById('pixel-spread');
const pixelBrigthnessDeo = document.getElementById('pixel-brightness');
const pixelContrastDeo = document.getElementById('pixel-contrast');
const blurLevelDeo = document.getElementById('blur-level');
const lppDeo = document.getElementById('lines-per-pixel');
const featureQuitDeo = document.getElementById('feature-quit');

const featureChangeColorRepresentationDeo = document.getElementById('feature-change-color-representation');
const featureChangePixelGeometryDeo = document.getElementById('feature-change-pixel-geometry');
const featureChangePixelShadowDeo = document.getElementById('feature-change-pixel-shadow');
const featureChangeScreenLayeringTypeDeo = document.getElementById('feature-change-screen-layering-type');
const featureChangeScreenCurvatureDeo = document.getElementById('feature-change-screen-curvature');

const featureChangeMoveSpeedDeo = document.getElementById('feature-change-move-speed');
const featureChangeTurnSpeedDeo = document.getElementById('feature-change-turn-speed');
const featureChangePixelSpeedDeo = document.getElementById('feature-change-pixel-speed');
const featureCameraMovementsDeo = document.getElementById('feature-camera-movements');
const featureCameraTurnsDeo = document.getElementById('feature-camera-turns');
const resetCameraDeo = document.getElementById('reset-camera');
const resetFiltersDeo = document.getElementById('reset-filters');
const resetSpeedsDeo = document.getElementById('reset-speeds');

const getGlCanvasDeo = () => document.getElementById(glCanvasHtmlId);
const getPreviewDeo = () => document.getElementById(previewHtmlId);
const getTopMessageDeo = () => document.getElementById(topMessageHtmlId);

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
            detail: 'Toggle the Sim Panel by pressing SPACE.'
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

window.addEventListener('app-event.screenshot', event => {
    const arrayBuffer = event.detail[0];
    const multiplier = event.detail[1];

    const width = 1920 * 2 * multiplier;
    const height = 1080 * 2 * multiplier;
    var canvas = document.createElement('canvas');
    canvas.width = width;
    canvas.height = height;
    var ctx = canvas.getContext('2d');

    var imageData = ctx.createImageData(width, height);
    imageData.data.set(arrayBuffer);
    ctx.putImageData(imageData, 0, 0);
    ctx.globalCompositeOperation = 'copy';
    ctx.scale(1,-1); // Y flip
    ctx.translate(0, -imageData.height);
    ctx.drawImage(canvas, 0,0);
    ctx.setTransform(1,0,0,1,0,0);
    ctx.globalCompositeOperation = 'source-over';

    const a = document.createElement('a');
    document.body.appendChild(a);
    a.classList.add('no-display');
    a.href = canvas.toDataURL();
    a.download = 'CRT-3D-Sim_' + new Date().toISOString() + '.png';
    a.click();
    setTimeout(() => a.remove(), 1000);
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
    cameraPosXDeo.value = Math.round(event.detail[0] * 100) / 100;
    cameraPosYDeo.value = Math.round(event.detail[1] * 100) / 100;
    cameraPosZDeo.value = Math.round(event.detail[2] * 100) / 100;
    cameraDirXDeo.value = Math.round(event.detail[3] * 100) / 100;
    cameraDirYDeo.value = Math.round(event.detail[4] * 100) / 100;
    cameraDirZDeo.value = Math.round(event.detail[5] * 100) / 100;
    cameraAxisUpXDeo.value = Math.round(event.detail[6] * 100) / 100;
    cameraAxisUpYDeo.value = Math.round(event.detail[7] * 100) / 100;
    cameraAxisUpZDeo.value = Math.round(event.detail[8] * 100) / 100;
}, false);

updateInnerHtmlWithEventNumber(cameraZoomDeo, 'app-event.change_camera_zoom');
updateInnerHtmlWithEventNumber(pixelWidthDeo, 'app-event.change_pixel_width');
updateInnerHtmlWithEventNumber(pixelHorizontalGapDeo, 'app-event.change_pixel_horizontal_gap');
updateInnerHtmlWithEventNumber(pixelVerticalGapDeo, 'app-event.change_pixel_vertical_gap');
updateInnerHtmlWithEventNumber(pixelSpreadDeo, 'app-event.change_pixel_spread');
updateInnerHtmlWithEventNumber(pixelBrigthnessDeo, 'app-event.change_pixel_brightness');
updateInnerHtmlWithEventNumber(pixelContrastDeo, 'app-event.change_pixel_contrast');
updateInnerHtmlWithEventNumber(blurLevelDeo, 'app-event.change_blur_level');
updateInnerHtmlWithEventNumber(lppDeo, 'app-event.change_lines_per_pixel');
updateInnerHtmlWithEventNumber(lightColorDeo, 'app-event.change_light_color');
updateInnerHtmlWithEventNumber(brightnessColorDeo, 'app-event.change_brightness_color');
updateInnerHtmlWithEventNumber(featureChangeMoveSpeedDeo, 'app-event.change_movement_speed');
updateInnerHtmlWithEventNumber(featureChangePixelSpeedDeo, 'app-event.change_pixel_speed');
updateInnerHtmlWithEventNumber(featureChangeTurnSpeedDeo, 'app-event.change_turning_speed');
function updateInnerHtmlWithEventNumber(deo, eventId) {
    if (!deo) throw new Error("Wrong deo on defining: " + eventId);
    window.addEventListener(eventId, event => {
        switch (eventId) {
            case 'app-event.change_camera_zoom':
            case 'app-event.change_pixel_width':
            case 'app-event.change_pixel_horizontal_gap':
            case 'app-event.change_pixel_vertical_gap':
            case 'app-event.change_pixel_spread':
            case 'app-event.change_blur_level':
            case 'app-event.change_lines_per_pixel':
            case 'app-event.change_pixel_contrast':
            case 'app-event.change_movement_speed':
            case 'app-event.change_pixel_speed':
            case 'app-event.change_turning_speed':
                deo.value = Math.round(event.detail * 1000.0) / 1000.0;
                break;
            case 'app-event.change_pixel_brightness':
                deo.value = Math.round(event.detail * 100.0) / 100.0;
                break;
            case 'app-event.change_light_color':
            case 'app-event.change_brightness_color':
                deo.value = '#' + event.detail.toString(16);
                break;
            default: throw new Error("Unreachable!");
        }
    }, false);
}

customEventOnButtonPressed(featureCameraMovementsDeo);
customEventOnButtonPressed(featureCameraTurnsDeo);
function customEventOnButtonPressed(deo) {
    deo.querySelectorAll('.activate-button').forEach((button) => {
        const eventOptions = {key: button.innerHTML.toLowerCase()};
        button.onmousedown = () => document.dispatchEvent(new KeyboardEvent('keydown', eventOptions));
        button.onmouseup = () => document.dispatchEvent(new KeyboardEvent('keyup', eventOptions));
    });
}

customEventOnChange(cameraPosXDeo, "camera_pos_x", a => +a);
customEventOnChange(cameraPosYDeo, "camera_pos_y", a => +a);
customEventOnChange(cameraPosZDeo, "camera_pos_z", a => +a);
customEventOnChange(cameraAxisUpXDeo, "camera_axis_up_x", a => +a);
customEventOnChange(cameraAxisUpYDeo, "camera_axis_up_y", a => +a);
customEventOnChange(cameraAxisUpZDeo, "camera_axis_up_z", a => +a);
customEventOnChange(cameraDirXDeo, "camera_direction_x", a => +a);
customEventOnChange(cameraDirYDeo, "camera_direction_y", a => +a);
customEventOnChange(cameraDirZDeo, "camera_direction_z", a => +a);
customEventOnChange(cameraZoomDeo, "camera_zoom", a => +a);

customEventOnChange(pixelWidthDeo, "pixel_width", a => +a);
customEventOnChange(pixelSpreadDeo, "pixel_spread", a => +a);
customEventOnChange(pixelHorizontalGapDeo, "pixel_horizontal_gap", a => +a);
customEventOnChange(pixelVerticalGapDeo, "pixel_vertical_gap", a => +a);
customEventOnChange(blurLevelDeo, "blur_level", a => +a);
customEventOnChange(lppDeo, "lines_per_pixel", a => +a);
customEventOnChange(pixelBrigthnessDeo, "pixel_brightness", a => +a);
customEventOnChange(pixelContrastDeo, "pixel_brightness", a => +a);

const parseColor = (value) => parseInt('0x' +value.substring(1));
customEventOnChange(lightColorDeo, "light_color", parseColor);
customEventOnChange(brightnessColorDeo, "brightness_color", parseColor);
function customEventOnChange(deo, kind, parse) {
    const changed = () => {
        window.dispatchEvent(new CustomEvent('app-event.custom_input_event', {
            detail: {
                value: parse(deo.value),
                kind: "event_kind:"+kind,
            },
        }));
    };
    deo.onclick = changed;
    deo.onchange = changed;
}

[
    featureChangeColorRepresentationDeo,
    featureChangePixelGeometryDeo,
    featureChangePixelShadowDeo,
    featureChangeScreenLayeringTypeDeo,
    featureChangeScreenCurvatureDeo,
    featureQuitDeo,
    resetCameraDeo,
    resetSpeedsDeo,
    resetFiltersDeo,
].forEach(deo => {
    deo.onmousedown = () => document.dispatchEvent(new KeyboardEvent('keydown', {key: deo.id}));
    deo.onmouseup = () => document.dispatchEvent(new KeyboardEvent('keyup', {key: deo.id}));
});

document.querySelectorAll('.number-input').forEach(deo => {
    [{button_text: "↑", mode: "inc", placement: "before"}, {button_text: "↓", mode: "dec", placement: "after"}].forEach(o => {
        const button = document.createElement('button');
        button.innerText = o.button_text;
        button.classList.add("button-inc-dec");
        const eventOptions = {key: deo.id + "-" + o.mode};
        button.onmousedown = () => document.dispatchEvent(new KeyboardEvent('keydown', eventOptions));
        button.onmouseup = () => document.dispatchEvent(new KeyboardEvent('keyup', eventOptions));
        deo.parentNode.insertBefore(button, o.placement === "before" ? deo : deo.nextSibling);
    });
});

document.querySelectorAll('input').forEach(deo => {
    const eventOptions = {key: 'input_focused'};
    deo.addEventListener('focus', () => document.dispatchEvent(new KeyboardEvent('keydown', eventOptions)));
    deo.addEventListener('blur', () => document.dispatchEvent(new KeyboardEvent('keyup', eventOptions)));
});

infoHideDeo.onclick = () => {
    if (!getGlCanvasDeo()) {
        return;
    }
    visibility.hideInfoPanel();
    window.dispatchEvent(new CustomEvent('app-event.top_message', {
        detail: 'Show the Sim Panel again by pressing SPACE.'
    }));
};

inputFileUploadDeo.onchange = () => {
  const file = inputFileUploadDeo.files[0];
  const url = (window.URL || window.webkitURL).createObjectURL(file);
  processFileToUpload(url);
};
dropZoneDeo.onclick = () => {
    inputFileUploadDeo.click();
};
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
restoreDefaultOptionsDeo.onclick = () => {
    storage.removeAllOptions();
    const scalingSelectionInput = storage.getScalingInputElement();
    scalingSelectionInput.checked = false;
    const powerPreferenceInput = storage.getPowerPreferenceInputElement();
    powerPreferenceInput.checked = false;
    loadInputValuesFromStorage();
};

prepareUi();

function prepareUi() {
    loadInputValuesFromStorage();

    visibility.showUi();
    visibility.hideLoading();

    const startPromise = new Promise((startResolve, startReject) => {
        startCustomDeo.onclick = () => {
            visibility.hideUi();
            visibility.showLoading();
            setTimeout(() => {
                const preview = getPreviewDeo();
                const canvas = document.createElement('canvas');
                const ctx = canvas.getContext('2d');
                canvas.width = preview.width;
                canvas.height = preview.height;
                ctx.drawImage(preview, 0, 0);
                var rawImg = ctx.getImageData(0, 0, preview.width, preview.height);

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
        let backgroundWidth = imageWidth;
        let backgroundHeight = imageHeight;

        switch(checkedScalingInput.id) {
            case scalingAutoHtmlId:
                scaleX = imageWidth <= 1024 ? (4/3)/(imageWidth/imageHeight) : 1;
                window.dispatchEvent(new CustomEvent('app-event.top_message', {
                    detail: 'Scaling auto detect: ' + (scaleX === 1 ? 'none.' : '4:3 on full image.')
                }));
                break;
            case scaling43HtmlId:
                scaleX = (4/3)/(imageWidth/imageHeight);
                break;
            case scalingStretchToBothEdgesHtmlId:
                scaleX = (width/height)/(imageWidth/imageHeight);
                stretch = true;
                break;
            case scalingStretchToNearestEdgeHtmlId:
                stretch = true;
                break;
            case scalingCustomHtmlId:
                stretch = scalingCustomStretchNearestDeo.checked;
                storage.setCustomResWidth(scalingCustomResWidthDeo.value);
                storage.setCustomResHeight(scalingCustomResHeightDeo.value);
                storage.setCustomArX(scalingCustomArXDeo.value);
                storage.setCustomArY(scalingCustomArYDeo.value);
                storage.setCustomStretchNearest(stretch);
                backgroundWidth = +scalingCustomResWidthDeo.value;
                backgroundHeight = +scalingCustomResHeightDeo.value;
                scaleX = (+scalingCustomArXDeo.value / +scalingCustomArYDeo.value)/(backgroundWidth/backgroundHeight);
                break;
        }

        lightColorDeo.value = '#FFFFFF';
        brightnessColorDeo.value = '#FFFFFF';

        const canvas = document.createElement('canvas');

        canvas.id = glCanvasHtmlId;

        canvas.width = Math.floor(width * dpi / 80) * 80;
        canvas.height = Math.floor(height * dpi / 60) * 60;

        canvas.style.width = width;
        canvas.style.height = height;

        infoPanelDeo.style.setProperty('max-height', height - 36);

        document.body.appendChild(canvas);

        const checkedPowerPreferenceInput = formDeo.querySelector('input[name=\''+powerPreferenceHtmlName+'\']:checked');

        const ctxOptions = {
            alpha: false,
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
            alert("Error! WebGL context could not be created!");
            console.error(new Error('Could not get webgl context.'));
            canvas.remove();
            prepareUi();
            return;
        }

        import(/* webpackPrefetch: true */'./crt_3d_sim').then(wasm => {
            const animation = new wasm.AnimationWasm(
                imageWidth, imageHeight, // to read the image pixels
                backgroundWidth, backgroundHeight, // to calculate model distance to the camera
                canvas.width, canvas.height, // gl.viewport
                1 / 60, +scaleX, stretch
            );
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

function loadInputValuesFromStorage() {
    const scalingSelectionInput = storage.getScalingInputElement();
    scalingSelectionInput.checked = true;
    const scalingSelectionId = scalingSelectionInput.id;
    if (scalingSelectionId === scalingCustomHtmlId) {
        visibility.showScaleCustomInputs();
    } else {
        visibility.hideScaleCustomInputs();
    }
    scalingCustomResWidthDeo.value = storage.getCustomResWidth();
    scalingCustomResHeightDeo.value = storage.getCustomResHeight();
    scalingCustomArXDeo.value = storage.getCustomArX();
    scalingCustomArYDeo.value = storage.getCustomArY();
    scalingCustomStretchNearestDeo.checked = storage.getCustomStretchNearest();
    antialiasDeo.checked = storage.getAntiAliasing();
    const powerPreferenceInput = storage.getPowerPreferenceInputElement();
    powerPreferenceInput.checked = true;
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
    const width = img.width;
    const height = img.height;
    if (width > height) {
        img.style.width = '100px';
    } else {
        img.style.height = '100px';
    }
    scalingCustomResButtonDeo.value = "Set to " + width + " ✕ " + height;
    scalingCustomResButtonDeo.onclick = () => {
        scalingCustomResWidthDeo.value = width;
        scalingCustomResHeightDeo.value = height;
    };
    previewWidthDeo.innerHTML = width;
    previewHeightDeo.innerHTML = height;
    dropZoneTextDeo.remove();
    dropZoneDeo.insertBefore(img, previewSizeElementsDeo);
    visibility.showPreviewSizeElements();
    visibility.showScalingCustomResButton();
    startCustomDeo.disabled = false;
}

function makeStorage() {
    const optionScalingId = 'option-scaling-id';
    const optionScalingCustomResWidth = 'option-scaling-custom-resolution-width';
    const optionScalingCustomResHeight = 'option-scaling-custom-resolution-height';
    const optionScalingCustomArX = 'option-scaling-custom-aspect-ratio-x';
    const optionScalingCustomArY = 'option-scaling-custom-aspect-ratio-y';
    const optionScalingCustomStretchNearest = 'option-scaling-custom-stretch-nearest';
    const optionPowerPreferenceId = 'option-powerPreference-id';
    const optionAntialias = 'option-antialias';
    return {
        getScalingInputElement: () => geElementByStoredIdOrBackup(optionScalingId, scalingAutoHtmlId),
        setScalingId: (scalingId) => localStorage.setItem(optionScalingId, scalingId),
        getCustomResWidth: () => localStorage.getItem(optionScalingCustomResWidth) || 256,
        setCustomResWidth: width => localStorage.setItem(optionScalingCustomResWidth, width),
        getCustomResHeight: () => localStorage.getItem(optionScalingCustomResHeight) || 224,
        setCustomResHeight: height => localStorage.setItem(optionScalingCustomResHeight, height),
        getCustomArX: () => localStorage.getItem(optionScalingCustomArX) || 4,
        setCustomArX: x => localStorage.setItem(optionScalingCustomArX, x),
        getCustomArY: () => localStorage.getItem(optionScalingCustomArY) || 3,
        setCustomArY: y => localStorage.setItem(optionScalingCustomArY, y),
        getCustomStretchNearest: () => localStorage.getItem(optionScalingCustomStretchNearest) === 'true',
        setCustomStretchNearest: stretch => localStorage.setItem(optionScalingCustomStretchNearest, stretch ? 'true' : 'false'),
        getPowerPreferenceInputElement: () => geElementByStoredIdOrBackup(optionPowerPreferenceId, powerPreferenceDefaultId),
        setPowerPreference: (powerPreference) => localStorage.setItem(optionPowerPreferenceId, powerPreference),
        getAntiAliasing: () => localStorage.getItem(optionAntialias) !== 'false',
        setAntiAliasing: antiAliasing => localStorage.setItem(optionAntialias, antiAliasing ? 'true' : 'false'),
        removeAllOptions: ()  => {
            localStorage.removeItem(optionScalingId);
            localStorage.removeItem(optionScalingCustomResWidth);
            localStorage.removeItem(optionScalingCustomResHeight);
            localStorage.removeItem(optionScalingCustomArX);
            localStorage.removeItem(optionScalingCustomArY);
            localStorage.removeItem(optionScalingCustomStretchNearest);
            localStorage.removeItem(optionPowerPreferenceId);
            localStorage.removeItem(optionAntialias);
        }
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
        showPreviewSizeElements: () => showElement(previewSizeElementsDeo),
        showScalingCustomResButton: () => showElement(scalingCustomResButtonDeo),
        showScaleCustomInputs: () => showElement(scalingCustomInputsDeo),
        hideScaleCustomInputs: () => hideElement(scalingCustomInputsDeo),
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
