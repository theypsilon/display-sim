import GIF from './third_party/gif';
import FontFaceObserver from 'fontfaceobserver';

const wasmPromise = import('./wasm/screen_sim');

const displayNoneClassName = 'display-none';
const scalingAutoHtmlId = 'scaling-auto';
const scaling43HtmlId = 'scaling-4:3';
const scalingCustomHtmlId = 'scaling-custom';
const scalingStretchToBothEdgesHtmlId = 'scaling-stretch-both';
const scalingStretchToNearestEdgeHtmlId = 'scaling-stretch-nearest';
const powerPreferenceDefaultHtml = 'default';
const glCanvasHtmlId = 'gl-canvas';
const topMessageHtmlId = 'top-message';
const firstPreviewImageId = 'first-preview-image';

const uiDeo = document.getElementById('ui');
const loadingDeo = document.getElementById('loading');
const inputFileUploadDeo = document.getElementById('file');
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
const selectImageList = document.getElementById('select-image-list');
const restoreDefaultOptionsDeo = document.getElementById('restore-default-options');

const optionPowerPreferenceSelect = document.getElementById('option-powerPreference');
const optionScalingSelect = document.getElementById('option-scaling');

const toggleInfoPanelClass = document.querySelectorAll('.toggle-info-panel');
const freeModeControlsClas = document.querySelectorAll('.free-mode-only-controls');
const simulationUiDeo = document.getElementById('simulation-ui');
const infoPanelDeo = document.getElementById('info-panel');
const infoPanelScrollAreaDeo = document.getElementById('info-panel-scroll-area');
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
const cameraMovementModeDeo = document.getElementById('camera-movement-mode');

const pixelWidthDeo = document.getElementById('pixel-width');
const pixelHorizontalGapDeo = document.getElementById('pixel-horizontal-gap');
const pixelVerticalGapDeo = document.getElementById('pixel-vertical-gap');
const pixelSpreadDeo = document.getElementById('pixel-spread');
const pixelBrigthnessDeo = document.getElementById('pixel-brightness');
const pixelContrastDeo = document.getElementById('pixel-contrast');
const blurLevelDeo = document.getElementById('blur-level');
const lppDeo = document.getElementById('lines-per-pixel');
const featureQuitDeo = document.getElementById('feature-quit');
const featureCaptureFramebufferDeo = document.getElementById('feature-capture-framebuffer');
const featureClosePanelDeo = document.getElementById('feature-close-panel');

const featureChangeColorRepresentationDeo = document.getElementById('feature-change-color-representation');
const featureChangePixelGeometryDeo = document.getElementById('feature-change-pixel-geometry');
const featureChangePixelShadowShapeDeo = document.getElementById('feature-change-pixel-shadow-shape');
const featureChangePixelShadowHeightDeo = document.getElementById('feature-change-pixel-shadow-height');
const featureChangeScreenLayeringTypeDeo = document.getElementById('feature-change-screen-layering-type');
const featureChangeScreenCurvatureDeo = document.getElementById('feature-change-screen-curvature');
const featureInternalResolutionDeo = document.getElementById('feature-internal-resolution');
const featureTextureInterpolationDeo = document.getElementById('feature-texture-interpolation');

const featureChangeMoveSpeedDeo = document.getElementById('feature-change-move-speed');
const featureChangeTurnSpeedDeo = document.getElementById('feature-change-turn-speed');
const featureChangePixelSpeedDeo = document.getElementById('feature-change-pixel-speed');
const featureCameraMovementsDeo = document.getElementById('feature-camera-movements');
const featureCameraTurnsDeo = document.getElementById('feature-camera-turns');
const resetCameraDeo = document.getElementById('reset-camera');
const resetFiltersDeo = document.getElementById('reset-filters');
const resetSpeedsDeo = document.getElementById('reset-speeds');

const getGlCanvasDeo = () => document.getElementById(glCanvasHtmlId);
const getTopMessageDeo = () => document.getElementById(topMessageHtmlId);

const isRunningOnMobileDevice = mobileAndTabletCheck();
const visibility = makeVisibility();
const storage = makeStorage();
const gifCache = {};
window.gifCache = gifCache;
let previewDeo = document.getElementById(firstPreviewImageId);
let simulationResources;

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
    visibility.hideSimulationUi();
}, false);

window.addEventListener('app-event.screenshot', async event => {
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
    ctx.scale(1, -1); // Y flip
    ctx.translate(0, -imageData.height);
    ctx.drawImage(canvas, 0, 0);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.globalCompositeOperation = 'source-over';

    const a = document.createElement('a');
    document.body.appendChild(a);
    a.classList.add('no-display');
    const blob = await new Promise(resolve => canvas.toBlob(resolve));
    const url = URL.createObjectURL(blob);
    a.href = url;
    a.download = 'CRT-3D-Sim_' + new Date().toISOString() + '.png';
    a.click();
    setTimeout(() => {
        URL.revokeObjectURL(url);
        a.remove();
    }, 3000);
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
    benchmark('top_message: ' + event.detail);
    div.appendChild(span);
    document.body.appendChild(div);
    let opacity = 0.75;
    div.style.opacity = opacity;
    setTimeout(() => {
        function fade () {
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

[
    { deo: cameraZoomDeo, eventId: 'app-event.change_camera_zoom' },
    { deo: cameraMovementModeDeo, eventId: 'app-event.change_camera_movement_mode' },
    { deo: pixelWidthDeo, eventId: 'app-event.change_pixel_width' },
    { deo: pixelHorizontalGapDeo, eventId: 'app-event.change_pixel_horizontal_gap' },
    { deo: pixelVerticalGapDeo, eventId: 'app-event.change_pixel_vertical_gap' },
    { deo: pixelSpreadDeo, eventId: 'app-event.change_pixel_spread' },
    { deo: pixelBrigthnessDeo, eventId: 'app-event.change_pixel_brightness' },
    { deo: pixelContrastDeo, eventId: 'app-event.change_pixel_contrast' },
    { deo: blurLevelDeo, eventId: 'app-event.change_blur_level' },
    { deo: lppDeo, eventId: 'app-event.change_lines_per_pixel' },
    { deo: lightColorDeo, eventId: 'app-event.change_light_color' },
    { deo: brightnessColorDeo, eventId: 'app-event.change_brightness_color' },
    { deo: featureChangeMoveSpeedDeo, eventId: 'app-event.change_movement_speed' },
    { deo: featureChangePixelSpeedDeo, eventId: 'app-event.change_pixel_speed' },
    { deo: featureChangeTurnSpeedDeo, eventId: 'app-event.change_turning_speed' },

    { deo: featureChangeColorRepresentationDeo, eventId: 'app-event.color_representation' },
    { deo: featureChangePixelGeometryDeo, eventId: 'app-event.pixel_geometry' },
    { deo: featureChangePixelShadowShapeDeo, eventId: 'app-event.pixel_shadow_shape' },
    { deo: featureChangePixelShadowHeightDeo, eventId: 'app-event.pixel_shadow_height' },
    { deo: featureChangeScreenLayeringTypeDeo, eventId: 'app-event.screen_layering_type' },
    { deo: featureInternalResolutionDeo, eventId: 'app-event.internal_resolution' },
    { deo: featureTextureInterpolationDeo, eventId: 'app-event.texture_interpolation' },
    { deo: featureChangeScreenCurvatureDeo, eventId: 'app-event.screen_curvature' }
].forEach(({ deo, eventId }) => {
    if (!deo) throw new Error('Wrong deo on defining: ' + eventId);
    window.addEventListener(eventId, event => {
        deo.value = event.detail;
        if (event.id === 'app-event.change_camera_movement_mode') {
            switch (event.detail) {
            case 'Lock on Display':
                deo.title = 'The camera will move around the picture, always looking at it';
                freeModeControlsClas.forEach(deo => deo.classList.add(displayNoneClassName));
                break;
            case 'Free Flight':
                deo.title = 'The camera can move without any restriction in the whole 3D space with plane-like controls';
                freeModeControlsClas.forEach(deo => deo.classList.remove(displayNoneClassName));
                break;
            default:
                throw new Error('Unreachable!');
            }
        }
    }, false);
});

customEventOnButtonPressed(featureCameraMovementsDeo);
customEventOnButtonPressed(featureCameraTurnsDeo);
function customEventOnButtonPressed (deo) {
    deo.querySelectorAll('.activate-button').forEach(button => {
        const eventOptions = { key: button.innerHTML.toLowerCase() };
        button.onmousedown = () => document.dispatchEvent(new KeyboardEvent('keydown', eventOptions));
        button.onmouseup = () => document.dispatchEvent(new KeyboardEvent('keyup', eventOptions));
    });
}

customEventOnChange(cameraPosXDeo, 'camera_pos_x', a => +a);
customEventOnChange(cameraPosYDeo, 'camera_pos_y', a => +a);
customEventOnChange(cameraPosZDeo, 'camera_pos_z', a => +a);
customEventOnChange(cameraAxisUpXDeo, 'camera_axis_up_x', a => +a);
customEventOnChange(cameraAxisUpYDeo, 'camera_axis_up_y', a => +a);
customEventOnChange(cameraAxisUpZDeo, 'camera_axis_up_z', a => +a);
customEventOnChange(cameraDirXDeo, 'camera_direction_x', a => +a);
customEventOnChange(cameraDirYDeo, 'camera_direction_y', a => +a);
customEventOnChange(cameraDirZDeo, 'camera_direction_z', a => +a);
customEventOnChange(cameraZoomDeo, 'camera_zoom', a => +a);
customEventOnChange(cameraMovementModeDeo, 'camera_movement_mode', a => +a);

customEventOnChange(pixelWidthDeo, 'pixel_width', a => +a);
customEventOnChange(pixelSpreadDeo, 'pixel_spread', a => +a);
customEventOnChange(pixelHorizontalGapDeo, 'pixel_horizontal_gap', a => +a);
customEventOnChange(pixelVerticalGapDeo, 'pixel_vertical_gap', a => +a);
customEventOnChange(blurLevelDeo, 'blur_level', a => +a);
customEventOnChange(lppDeo, 'lines_per_pixel', a => +a);
customEventOnChange(pixelBrigthnessDeo, 'pixel_brightness', a => +a);
customEventOnChange(pixelContrastDeo, 'pixel_contrast', a => +a);
customEventOnChange(featureChangePixelShadowHeightDeo, 'pixel_shadow_height', a => +a);
const parseColor = (value) => parseInt('0x' + value.substring(1));
customEventOnChange(lightColorDeo, 'light_color', parseColor);
customEventOnChange(brightnessColorDeo, 'brightness_color', parseColor);
function customEventOnChange (deo, kind, parse) {
    const changed = () => {
        window.dispatchEvent(new CustomEvent('app-event.custom_input_event', {
            detail: {
                value: parse(deo.value),
                kind: 'event_kind:' + kind
            }
        }));
    };
    deo.onchange = changed;
}

[
    featureChangeColorRepresentationDeo,
    featureChangePixelGeometryDeo,
    featureChangePixelShadowShapeDeo,
    featureChangePixelShadowHeightDeo,
    featureChangeScreenLayeringTypeDeo,
    featureChangeScreenCurvatureDeo,
    featureQuitDeo,
    featureCaptureFramebufferDeo,
    featureClosePanelDeo,
    resetCameraDeo,
    resetSpeedsDeo,
    resetFiltersDeo
].forEach(deo => {
    deo.onmousedown = () => document.dispatchEvent(new KeyboardEvent('keydown', { key: deo.id }));
    deo.onmouseup = () => document.dispatchEvent(new KeyboardEvent('keyup', { key: deo.id }));
});

document.querySelectorAll('.selectable-image').forEach(deo => {
    const img = deo.querySelector('img');
    img.isGif = img.src.includes('.gif');
    img.isAsset = true;
    makeImageSelectable(deo);
});
function makeImageSelectable (deo) {
    deo.onclick = () => {
        previewDeo.classList.remove('selected-image');
        previewDeo = deo;
        previewDeo.classList.add('selected-image');
    };
}

document.querySelectorAll('.number-input').forEach(deo => {
    [{ button_text: '↑', mode: 'inc', placement: 'before' }, { button_text: '↓', mode: 'dec', placement: 'after' }].forEach(o => {
        const button = document.createElement('button');
        button.innerText = o.button_text;
        button.classList.add('button-inc-dec');
        const eventOptions = { key: deo.id + '-' + o.mode };
        button.onmousedown = () => document.dispatchEvent(new KeyboardEvent('keydown', eventOptions));
        button.onmouseup = () => document.dispatchEvent(new KeyboardEvent('keyup', eventOptions));
        deo.parentNode.insertBefore(button, o.placement === 'before' ? deo : deo.nextSibling);
    });
});

document.querySelectorAll('input').forEach(deo => {
    const eventOptions = { key: 'input_focused' };
    deo.addEventListener('focus', () => document.dispatchEvent(new KeyboardEvent('keydown', eventOptions)));
    deo.addEventListener('blur', () => document.dispatchEvent(new KeyboardEvent('keyup', eventOptions)));
});

toggleInfoPanelClass.forEach(deo => {
    deo.onclick = () => {
        if (!getGlCanvasDeo()) {
            return;
        }
        if (visibility.isInfoPanelVisible()) {
            visibility.hideInfoPanel();
            window.dispatchEvent(new CustomEvent('app-event.top_message', {
                detail: 'Show the Sim Panel again by pressing SPACE.'
            }));
        } else {
            visibility.showInfoPanel();
        }
    };
});

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
optionScalingSelect.onchange = () => {
    if (optionScalingSelect.value === scalingCustomHtmlId) {
        visibility.showScaleCustomInputs();
    } else {
        visibility.hideScaleCustomInputs();
    }
};

restoreDefaultOptionsDeo.onclick = () => {
    storage.removeAllOptions();
    loadInputValuesFromStorage();
};

Promise.all([
    new FontFaceObserver('Archivo Black', { weight: 400 }).load(null, 10000),
    new FontFaceObserver('Lato', { weight: 400 }).load(null, 10000),
    new FontFaceObserver('Lato', { weight: 700 }).load(null, 10000)
]).then(prepareUi).catch(e => {
    console.error(e);
    prepareUi();
});

async function prepareUi () {
    loadInputValuesFromStorage();

    visibility.showUi();
    visibility.hideLoading();

    if (isRunningOnMobileDevice) {
        startAnimationDeo.disabled = true;
        startAnimationDeo.title = 'You need a PC with NVIDIA or ATI graphics card with updated drivers and a WebGL2 compatible browser (Firefox, Opera or Chrome) in order to run this without problems.';
        return;
    }

    await new Promise(resolve => {
        startAnimationDeo.onclick = resolve;
    });

    visibility.hideUi();
    visibility.showLoading();

    await new Promise(resolve => setTimeout(resolve, 50));

    const rawImgs = await (async function () {
        if (previewDeo.id === firstPreviewImageId) {
            const img = new Image();
            img.src = require('../assets/pics/wwix_spritesheet.png');
            await new Promise((resolve, reject) => {
                img.onload = resolve;
                img.onerror = reject;
            });
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
                rawImgs.push({ raw: ctx.getImageData(x * 256, y * 224, 256, 224), delay: 16 });
            }
            return rawImgs;
        } else {
            let img = previewDeo.querySelector('img');
            const isAsset = !!img.isAsset;
            const isGif = !!img.isGif;
            if (isAsset) {
                const imgHqSrc = img.dataset.hq;
                img = new Image();
                img.src = imgHqSrc;
                await new Promise((resolve, reject) => {
                    img.onload = resolve;
                    img.onerror = reject;
                });
            }
            const canvas = document.createElement('canvas');
            const ctx = canvas.getContext('2d');
            canvas.width = img.width;
            canvas.height = img.height;
            ctx.drawImage(img, 0, 0);
            if (!isGif) {
                return [{ raw: ctx.getImageData(0, 0, img.width, img.height), delay: 16 }];
            }
            benchmark('loading gif');
            const gifKey = isAsset ? img.src : canvas.toDataURL();
            let gif = gifCache[gifKey];
            if (!gif) {
                gif = new GIF();
                gif.load(img.src);
                await new Promise(resolve => {
                    gif.onload = () => resolve();
                });
                gifCache[gifKey] = gif;
            }
            benchmark('gif loaded', gif);
            return gif.frames.map(frame => ({
                raw: frame.image.getContext('2d').getImageData(0, 0, img.width, img.height),
                delay: frame.delay
            }));
        }
    }());

    benchmark('image readed');

    let scaleX = 1;
    let stretch = false;
    storage.setScalingSelectOption(optionScalingSelect.value);

    const imageWidth = rawImgs[0].raw.width;
    const imageHeight = rawImgs[0].raw.height;
    let backgroundWidth = imageWidth;
    let backgroundHeight = imageHeight;

    switch (optionScalingSelect.value) {
    case scalingAutoHtmlId:
        scaleX = imageWidth <= 1024 ? (4 / 3) / (imageWidth / imageHeight) : 1;
        window.dispatchEvent(new CustomEvent('app-event.top_message', {
            detail: 'Scaling auto detect: ' + (scaleX === 1 ? 'none.' : '4:3 on full image.')
        }));
        break;
    case scaling43HtmlId:
        scaleX = (4 / 3) / (imageWidth / imageHeight);
        break;
    case scalingStretchToBothEdgesHtmlId:
        scaleX = (window.screen.width / window.screen.height) / (imageWidth / imageHeight);
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
        scaleX = (+scalingCustomArXDeo.value / +scalingCustomArYDeo.value) / (backgroundWidth / backgroundHeight);
        break;
    }

    lightColorDeo.value = '#FFFFFF';
    brightnessColorDeo.value = '#FFFFFF';

    const canvas = document.createElement('canvas');

    canvas.id = glCanvasHtmlId;

    fixCanvasSize(canvas);

    canvas.onfocus = () => document.dispatchEvent(new KeyboardEvent('keydown', { key: 'canvas_focused' }));
    canvas.onblur = () => document.dispatchEvent(new KeyboardEvent('keyup', { key: 'canvas_focused' }));

    document.body.appendChild(canvas);

    const ctxOptions = {
        alpha: false,
        antialias: antialiasDeo.checked,
        depth: true,
        failIfMajorPerformanceCaveat: false,
        powerPreference: optionPowerPreferenceSelect.value,
        premultipliedAlpha: false,
        preserveDrawingBuffer: false,
        stencil: false
    };

    storage.setAntiAliasing(ctxOptions.antialias);
    storage.setPowerPreferenceSelectOption(optionPowerPreferenceSelect.value);

    benchmark('gl context form', ctxOptions);
    const gl = canvas.getContext('webgl2', ctxOptions);

    var documentElement = document.documentElement;
    documentElement.requestFullscreen = documentElement.requestFullscreen ||
        documentElement.webkitRequestFullScreen ||
        documentElement['mozRequestFullScreen'] ||
        documentElement.msRequestFullscreen;

    canvas.onmousedown = (e) => {
        if (e.buttons !== 1) return;
        canvas.requestPointerLock();
        if (window.screen.width !== window.innerWidth && window.screen.height !== window.innerHeight) {
            documentElement.requestFullscreen();
        }
    };

    canvas.requestPointerLock = canvas.requestPointerLock || canvas.mozRequestPointerLock;
    document.exitPointerLock = document.exitPointerLock || document.mozExitPointerLock;

    if (!gl) {
        window.dispatchEvent(new CustomEvent('app-event.top_message', {
            detail: 'WebGL2 is not working on your browser, try restarting it! And remember, this works only on a PC with updated browser and graphics drivers.'
        }));
        console.error(new Error('Could not get webgl2 context.'));
        canvas.remove();
        prepareUi();
        return;
    }

    const wasm = await wasmPromise;

    const videoInput = wasm.new_video_input_wasm(
        imageWidth, imageHeight, // to read the image pixels
        backgroundWidth, backgroundHeight, // to calculate model distance to the camera
        canvas.width, canvas.height, // gl.viewport
        +scaleX, stretch, gl.getParameter(gl.MAX_TEXTURE_SIZE)
    );
    for (let i = 0; i < rawImgs.length; i++) {
        const rawImg = rawImgs[i];
        wasm.add_buffer_to_video_input(videoInput, new Uint8Array(rawImg.raw.data.buffer), rawImg.delay);
    }

    if (simulationResources === undefined) {
        benchmark('calling wasm load_simulation_resources');
        simulationResources = wasm.load_simulation_resources();
        benchmark('wasm load_simulation_resources done');
    }
    benchmark('calling wasm run_program');
    wasm.run_program(gl, simulationResources, videoInput);
    benchmark('wasm run_program done');

    visibility.hideLoading();
    visibility.showInfoPanel();
    visibility.showSimulationUi();
}

function loadInputValuesFromStorage () {
    optionScalingSelect.value = storage.getScalingSelectOption();
    optionPowerPreferenceSelect.value = storage.getPowerPreferenceSelectOption();
    if (optionScalingSelect.value === scalingCustomHtmlId) {
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
}

async function processFileToUpload (url) {
    var xhr = new XMLHttpRequest();
    xhr.open('GET', url, true);
    xhr.responseType = 'blob';
    xhr.send(null);

    await new Promise(resolve => {
        xhr.onload = () => resolve();
    });

    const previewUrl = URL.createObjectURL(xhr.response);
    const img = new Image();
    img.src = previewUrl;

    await new Promise((resolve, reject) => {
        img.onload = resolve;
        img.onerror = reject;
    });

    img.isGif = xhr.response.type === 'image/gif';

    const width = img.width;
    const height = img.height;
    scalingCustomResButtonDeo.value = 'Set to ' + width + ' ✕ ' + height;
    scalingCustomResButtonDeo.onclick = () => {
        scalingCustomResWidthDeo.value = width;
        scalingCustomResHeightDeo.value = height;
    };
    const span = document.createElement('span');
    span.innerHTML = width + ' ✕ ' + height;
    const div = document.createElement('div');
    div.appendChild(img);
    div.appendChild(span);
    const li = document.createElement('li');
    li.classList.add('selectable-image');
    li.appendChild(div);
    makeImageSelectable(li);
    li.click();
    selectImageList.insertBefore(li, dropZoneDeo);
    visibility.showScalingCustomResButton();
}

function makeStorage () {
    const optionScalingSelect = 'option-scaling';
    const optionPowerPreferenceSelect = 'option-powerPreference';
    const optionScalingCustomResWidth = 'option-scaling-custom-resolution-width';
    const optionScalingCustomResHeight = 'option-scaling-custom-resolution-height';
    const optionScalingCustomArX = 'option-scaling-custom-aspect-ratio-x';
    const optionScalingCustomArY = 'option-scaling-custom-aspect-ratio-y';
    const optionScalingCustomStretchNearest = 'option-scaling-custom-stretch-nearest';
    const optionAntialias = 'option-antialias';
    return {
        getScalingSelectOption: () => localStorage.getItem(optionScalingSelect) || scalingAutoHtmlId,
        setScalingSelectOption: option => localStorage.setItem(optionScalingSelect, option),
        getPowerPreferenceSelectOption: () => localStorage.getItem(optionPowerPreferenceSelect) || powerPreferenceDefaultHtml,
        setPowerPreferenceSelectOption: option => localStorage.setItem(optionPowerPreferenceSelect, option),
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
        getAntiAliasing: () => localStorage.getItem(optionAntialias) !== 'false',
        setAntiAliasing: antiAliasing => localStorage.setItem(optionAntialias, antiAliasing ? 'true' : 'false'),
        removeAllOptions: () => {
            localStorage.removeItem(optionScalingSelect);
            localStorage.removeItem(optionPowerPreferenceSelect);
            localStorage.removeItem(optionScalingCustomResWidth);
            localStorage.removeItem(optionScalingCustomResHeight);
            localStorage.removeItem(optionScalingCustomArX);
            localStorage.removeItem(optionScalingCustomArY);
            localStorage.removeItem(optionScalingCustomStretchNearest);
            localStorage.removeItem(optionAntialias);
        }
    };
}

function makeVisibility () {
    return {
        showUi: () => showElement(uiDeo),
        hideUi: () => hideElement(uiDeo),
        showLoading: () => showElement(loadingDeo),
        hideLoading: () => hideElement(loadingDeo),
        showSimulationUi: () => {
            document.body.style.setProperty('overflow', 'hidden');
            showElement(simulationUiDeo);
        },
        hideSimulationUi: () => {
            document.body.style.removeProperty('overflow');
            hideElement(simulationUiDeo);
        },
        showInfoPanel: () => showElement(infoPanelDeo),
        hideInfoPanel: () => hideElement(infoPanelDeo),
        isInfoPanelVisible: () => isVisible(infoPanelDeo),
        showScalingCustomResButton: () => showElement(scalingCustomResButtonDeo),
        showScaleCustomInputs: () => showElement(scalingCustomInputsDeo),
        hideScaleCustomInputs: () => hideElement(scalingCustomInputsDeo)
    };
    function showElement (element) {
        element.classList.remove(displayNoneClassName);
    }
    function hideElement (element) {
        element.classList.add(displayNoneClassName);
    }
    function isVisible (element) {
        return element.classList.contains(displayNoneClassName) === false;
    }
}

function fixCanvasSize (canvas) {
    canvas = canvas instanceof HTMLCanvasElement ? canvas : document.getElementById(glCanvasHtmlId);
    if (!canvas) return;

    const dpi = window.devicePixelRatio;
    const width = window.screen.width;
    const height = window.screen.height;
    const zoom = window.outerWidth / window.innerWidth;

    canvas.width = Math.floor(width * dpi / 80) * 80;
    canvas.height = Math.floor(height * dpi / 60) * 60;

    if (Math.abs(window.innerWidth / window.outerWidth - 1.0) < 0.05) {
        canvas.style.width = width;
        canvas.style.height = height;
    } else {
        canvas.style.width = window.innerWidth;
        canvas.style.height = window.innerHeight;
    }

    infoPanelScrollAreaDeo.style.setProperty('max-height', height * 0.95 / zoom);
}

function mobileAndTabletCheck () {
    var check = false;
    // eslint-disable-next-line no-useless-escape
    (function (a) { if (/(android|bb\d+|meego).+mobile|avantgo|bada\/|blackberry|blazer|compal|elaine|fennec|hiptop|iemobile|ip(hone|od)|iris|kindle|lge |maemo|midp|mmp|mobile.+firefox|netfront|opera m(ob|in)i|palm( os)?|phone|p(ixi|re)\/|plucker|pocket|psp|series(4|6)0|symbian|treo|up\.(browser|link)|vodafone|wap|windows ce|xda|xiino|android|ipad|playbook|silk/i.test(a) || /1207|6310|6590|3gso|4thp|50[1-6]i|770s|802s|a wa|abac|ac(er|oo|s\-)|ai(ko|rn)|al(av|ca|co)|amoi|an(ex|ny|yw)|aptu|ar(ch|go)|as(te|us)|attw|au(di|\-m|r |s )|avan|be(ck|ll|nq)|bi(lb|rd)|bl(ac|az)|br(e|v)w|bumb|bw\-(n|u)|c55\/|capi|ccwa|cdm\-|cell|chtm|cldc|cmd\-|co(mp|nd)|craw|da(it|ll|ng)|dbte|dc\-s|devi|dica|dmob|do(c|p)o|ds(12|\-d)|el(49|ai)|em(l2|ul)|er(ic|k0)|esl8|ez([4-7]0|os|wa|ze)|fetc|fly(\-|_)|g1 u|g560|gene|gf\-5|g\-mo|go(\.w|od)|gr(ad|un)|haie|hcit|hd\-(m|p|t)|hei\-|hi(pt|ta)|hp( i|ip)|hs\-c|ht(c(\-| |_|a|g|p|s|t)|tp)|hu(aw|tc)|i\-(20|go|ma)|i230|iac( |\-|\/)|ibro|idea|ig01|ikom|im1k|inno|ipaq|iris|ja(t|v)a|jbro|jemu|jigs|kddi|keji|kgt( |\/)|klon|kpt |kwc\-|kyo(c|k)|le(no|xi)|lg( g|\/(k|l|u)|50|54|\-[a-w])|libw|lynx|m1\-w|m3ga|m50\/|ma(te|ui|xo)|mc(01|21|ca)|m\-cr|me(rc|ri)|mi(o8|oa|ts)|mmef|mo(01|02|bi|de|do|t(\-| |o|v)|zz)|mt(50|p1|v )|mwbp|mywa|n10[0-2]|n20[2-3]|n30(0|2)|n50(0|2|5)|n7(0(0|1)|10)|ne((c|m)\-|on|tf|wf|wg|wt)|nok(6|i)|nzph|o2im|op(ti|wv)|oran|owg1|p800|pan(a|d|t)|pdxg|pg(13|\-([1-8]|c))|phil|pire|pl(ay|uc)|pn\-2|po(ck|rt|se)|prox|psio|pt\-g|qa\-a|qc(07|12|21|32|60|\-[2-7]|i\-)|qtek|r380|r600|raks|rim9|ro(ve|zo)|s55\/|sa(ge|ma|mm|ms|ny|va)|sc(01|h\-|oo|p\-)|sdk\/|se(c(\-|0|1)|47|mc|nd|ri)|sgh\-|shar|sie(\-|m)|sk\-0|sl(45|id)|sm(al|ar|b3|it|t5)|so(ft|ny)|sp(01|h\-|v\-|v )|sy(01|mb)|t2(18|50)|t6(00|10|18)|ta(gt|lk)|tcl\-|tdg\-|tel(i|m)|tim\-|t\-mo|to(pl|sh)|ts(70|m\-|m3|m5)|tx\-9|up(\.b|g1|si)|utst|v400|v750|veri|vi(rg|te)|vk(40|5[0-3]|\-v)|vm40|voda|vulc|vx(52|53|60|61|70|80|81|83|85|98)|w3c(\-| )|webc|whit|wi(g |nc|nw)|wmlb|wonu|x700|yas\-|your|zeto|zte\-/i.test(a.substr(0, 4))) check = true; })(navigator.userAgent || navigator.vendor || window.opera);
    return check;
}

function benchmark (message, ctx) {
    if (!window.screen_sim_bench && !window.localStorage.getItem('screen_sim_bench')) return;
    const date = new Date().toISOString();
    if (ctx) {
        console.log(date, message, ctx);
    } else {
        console.log(date, message);
    }
}
