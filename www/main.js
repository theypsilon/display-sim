const scalingAutoHtmlId = 'scaling-auto';
const scaling43HtmlId = 'scaling-4:3';
const scalingCustomHtmlId = 'scaling-custom';
const scalingStretchToBothEdgesHtmlId = 'scaling-stretch-both';
const scalingStretchToNearestEdgeHtmlId = 'scaling-stretch-nearest';
const powerPreferenceDefaultId = 'powerPreference-1';
const glCanvasHtmlId = 'gl-canvas';
const topMessageHtmlId = 'top-message';
const firstPreviewImageId = 'first-preview-image';

const scalingHtmlName = 'scaling';
const powerPreferenceHtmlName = 'powerPreference';

const uiDeo = document.getElementById('ui');
const formDeo = document.getElementById('form');
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

const visibility = makeVisibility();
const storage = makeStorage();
const gifCache = {};
let previewDeo = document.getElementById(firstPreviewImageId);
let simulationResources = undefined;

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

[
    {deo: cameraZoomDeo, eventId: 'app-event.change_camera_zoom'},
    {deo: pixelWidthDeo, eventId: 'app-event.change_pixel_width'},
    {deo: pixelHorizontalGapDeo, eventId: 'app-event.change_pixel_horizontal_gap'},
    {deo: pixelVerticalGapDeo, eventId: 'app-event.change_pixel_vertical_gap'},
    {deo: pixelSpreadDeo, eventId: 'app-event.change_pixel_spread'},
    {deo: pixelBrigthnessDeo, eventId: 'app-event.change_pixel_brightness'},
    {deo: pixelContrastDeo, eventId: 'app-event.change_pixel_contrast'},
    {deo: blurLevelDeo, eventId: 'app-event.change_blur_level'},
    {deo: lppDeo, eventId: 'app-event.change_lines_per_pixel'},
    {deo: lightColorDeo, eventId: 'app-event.change_light_color'},
    {deo: brightnessColorDeo, eventId: 'app-event.change_brightness_color'},
    {deo: featureChangeMoveSpeedDeo, eventId: 'app-event.change_movement_speed'},
    {deo: featureChangePixelSpeedDeo, eventId: 'app-event.change_pixel_speed'},
    {deo: featureChangeTurnSpeedDeo, eventId: 'app-event.change_turning_speed'},

    {deo: featureChangeColorRepresentationDeo, eventId: "app-event.color_representation"},
    {deo: featureChangePixelGeometryDeo, eventId: "app-event.pixel_geometry"},
    {deo: featureChangePixelShadowShapeDeo, eventId: "app-event.pixel_shadow_shape"},
    {deo: featureChangePixelShadowHeightDeo, eventId: "app-event.pixel_shadow_height"},
    {deo: featureChangeScreenLayeringTypeDeo, eventId: "app-event.screen_layering_type"},
    {deo: featureInternalResolutionDeo, eventId: "app-event.internal_resolution"},
    {deo: featureTextureInterpolationDeo, eventId: "app-event.texture_interpolation"},
    {deo: featureChangeScreenCurvatureDeo, eventId: "app-event.screen_curvature"},
].forEach(({deo, eventId}) => {
    if (!deo) throw new Error("Wrong deo on defining: " + eventId);
    window.addEventListener(eventId, event => {
        deo.value = event.detail;
        deo.title = event.detail;
    }, false);
});

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
customEventOnChange(featureChangePixelShadowHeightDeo, "pixel_shadow_height", a => +a);
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
    featureChangePixelShadowShapeDeo,
    featureChangePixelShadowHeightDeo,
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

document.querySelectorAll('.selectable-image').forEach(deo => {
    const img = deo.querySelector('img');
    img.isGif = img.src.includes(".gif");
    img.previewUrl = img.src;
    makeImageSelectable(deo);
});
function makeImageSelectable(deo) {
    deo.onclick = () => {
        previewDeo.classList.remove('selected-image');
        previewDeo = deo;
        previewDeo.classList.add('selected-image');
    }
}

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
        startAnimationDeo.onclick = () => {
            visibility.hideUi();
            visibility.showLoading();
            setTimeout(async () => {
                if (previewDeo.id === firstPreviewImageId) {
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
                                rawImgs.push({raw: ctx.getImageData(x * 256, y * 224, 256, 224), delay: 16});
                            }
                            imgResolve(rawImgs);
                        }
                        img.onerror = (e) => imgReject(e);
                    });
                    const rawImgs = await animationPromise;
                    startResolve(rawImgs)
                } else {
                    const img = previewDeo.querySelector('img');
                    const canvases = await loadCanvases(img);
                    const rawImgs = canvases.map(({ctx, delay}) => ({raw: ctx.getImageData(0, 0, img.width, img.height), delay}));
                    startResolve(rawImgs)
                }
            }, 50);

            async function loadCanvases(preview) {
                const canvas = document.createElement('canvas');
                const ctx = canvas.getContext('2d');
                canvas.width = preview.width;
                canvas.height = preview.height;
                ctx.drawImage(preview, 0, 0);
                if (!preview.isGif) {
                    return [{ctx, delay: 16}];
                }
                benchmark("loading gif");
                const gifKey = canvas.toDataURL();
                let gif = gifCache[gifKey];
                if (!gif) {
                    gif = GIF();
                    gif.load(preview.previewUrl);
                    await new Promise(resolve => gif.onload = () => resolve());
                    gifCache[gifKey] = gif;
                }
                benchmark("gif loaded", gif);
                return gif.frames.map(frame => ({ctx: frame.image.getContext('2d'), delay: frame.delay}));
            }
        }
    });

    startPromise.then((rawImgs) => {
        benchmark('image readed');
        const dpi = window.devicePixelRatio;
        const width = window.screen.width;
        const height = window.screen.height;

        const checkedScalingInput = formDeo.querySelector('input[name=\''+scalingHtmlName+'\']:checked');
        let scaleX = 1;
        let stretch = false;
        storage.setScalingId(checkedScalingInput.id);

        const imageWidth = rawImgs[0].raw.width;
        const imageHeight = rawImgs[0].raw.height;
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
        benchmark('gl context form', ctxOptions);
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
                +scaleX, stretch
            );
            for (let i = 0; i < rawImgs.length; i++) {
                const rawImg = rawImgs[i];
                animation.add(rawImg.raw.data.buffer, rawImg.delay);
            }

            if (simulationResources === undefined) {
                benchmark('calling wasm load_simulation_resources');
                simulationResources = wasm.load_simulation_resources();
                benchmark('wasm load_simulation_resources done');
            }
            benchmark('calling wasm run_program');
            wasm.run_program(gl, simulationResources, animation);
            benchmark('wasm run_program done');

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
    img.isGif = xhr.response.type === 'image/gif';
    img.previewUrl = previewUrl;

    await new Promise(resolve => img.onload = () => resolve());

    const width = img.width;
    const height = img.height;
    scalingCustomResButtonDeo.value = "Set to " + width + " ✕ " + height;
    scalingCustomResButtonDeo.onclick = () => {
        scalingCustomResWidthDeo.value = width;
        scalingCustomResHeightDeo.value = height;
    };
    const span = document.createElement('span');
    span.innerHTML = width + " ✕ " + height;
    const li = document.createElement('li');
    li.appendChild(img);
    li.appendChild(document.createElement('br'));
    li.appendChild(span);
    li.classList.add('selectable-image');
    makeImageSelectable(li);
    li.click();
    selectImageList.insertBefore(li, dropZoneDeo);
    visibility.showScalingCustomResButton();
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

function benchmark(message, ctx) {
    if (!window.screen_sim_bench && !window.localStorage.getItem('screen_sim_bench')) return;
    const date = new Date().toISOString();
    if (ctx) {
        console.log(date, message, ctx);
    } else {
        console.log(date, message);
    }
}

// This is a library found in StackOverflow: https://stackoverflow.com/questions/48234696/how-to-put-a-gif-with-canvas
/*============================================================================
  Gif Decoder and player for use with Canvas API's

**NOT** for commercial use.

To use

    var myGif = GIF();                  // creates a new gif  
    var myGif = new GIF();              // will work as well but not needed as GIF() returns the correct reference already.    
    myGif.load("myGif.gif");            // set URL and load
    myGif.onload = function(event){     // fires when loading is complete
                                        //event.type   = "load"
                                        //event.path   array containing a reference to the gif
    }
    myGif.onprogress = function(event){ // Note this function is not bound to myGif
                                        //event.bytesRead    bytes decoded
                                        //event.totalBytes   total bytes
                                        //event.frame        index of last frame decoded
    }
    myGif.onerror = function(event){    // fires if there is a problem loading. this = myGif
                                        //event.type   a description of the error
                                        //event.path   array containing a reference to the gif
    }

Once loaded the gif can be displayed
    if(!myGif.loading){
        ctx.drawImage(myGif.image,0,0); 
    }
You can display the last frame loaded during loading

    if(myGif.lastFrame !== null){
        ctx.drawImage(myGif.lastFrame.image,0,0); 
    }


To access all the frames
    var gifFrames = myGif.frames; // an array of frames.

A frame holds various frame associated items.
    myGif.frame[0].image; // the first frames image
    myGif.frame[0].delay; // time in milliseconds frame is displayed for




Gifs use various methods to reduce the file size. The loaded frames do not maintain the optimisations and hold the full resolution frames as DOM images. This mean the memory footprint of a decode gif will be many time larger than the Gif file.
 */
const GIF = function () {
    // **NOT** for commercial use.
    var timerID;                          // timer handle for set time out usage
    var st;                               // holds the stream object when loading.
    var interlaceOffsets  = [0, 4, 2, 1]; // used in de-interlacing.
    var interlaceSteps    = [8, 8, 4, 2];
    var interlacedBufSize;  // this holds a buffer to de interlace. Created on the first frame and when size changed
    var deinterlaceBuf;
    var pixelBufSize;    // this holds a buffer for pixels. Created on the first frame and when size changed
    var pixelBuf;
    const GIF_FILE = { // gif file data headers
        GCExt   : 0xF9,
        COMMENT : 0xFE,
        APPExt  : 0xFF,
        UNKNOWN : 0x01, // not sure what this is but need to skip it in parser
        IMAGE   : 0x2C,
        EOF     : 59,   // This is entered as decimal
        EXT     : 0x21,
    };      
    // simple buffered stream used to read from the file 
    var Stream = function (data) { 
        this.data = new Uint8ClampedArray(data);
        this.pos  = 0;
        var len   = this.data.length;
        this.getString = function (count) { // returns a string from current pos of len count
            var s = "";
            while (count--) { s += String.fromCharCode(this.data[this.pos++]) }
            return s;
        };
        this.readSubBlocks = function () { // reads a set of blocks as a string
            var size, count, data  = "";
            do {
                count = size = this.data[this.pos++];
                while (count--) { data += String.fromCharCode(this.data[this.pos++]) }
            } while (size !== 0 && this.pos < len);
            return data;
        }
        this.readSubBlocksB = function () { // reads a set of blocks as binary
            var size, count, data = [];
            do {
                count = size = this.data[this.pos++];
                while (count--) { data.push(this.data[this.pos++]);}
            } while (size !== 0 && this.pos < len);
            return data;
        }
    };
    // LZW decoder uncompressed each frames pixels
    // this needs to be optimised.
    // minSize is the min dictionary as powers of two
    // size and data is the compressed pixels
    function lzwDecode(minSize, data) {
        var i, pixelPos, pos, clear, eod, size, done, dic, code, last, d, len;
        pos = pixelPos = 0;
        dic      = [];
        clear    = 1 << minSize;
        eod      = clear + 1;
        size     = minSize + 1;
        done     = false;
        while (!done) { // JavaScript optimisers like a clear exit though I never use 'done' apart from fooling the optimiser
            last = code;
            code = 0;
            for (i = 0; i < size; i++) {
                if (data[pos >> 3] & (1 << (pos & 7))) { code |= 1 << i }
                pos++;
            }
            if (code === clear) { // clear and reset the dictionary
                dic = [];
                size = minSize + 1;
                for (i = 0; i < clear; i++) { dic[i] = [i] }
                dic[clear] = [];
                dic[eod] = null;
            } else {
                if (code === eod) {  done = true; return }
                if (code >= dic.length) { dic.push(dic[last].concat(dic[last][0])) }
                else if (last !== clear) { dic.push(dic[last].concat(dic[code][0])) }
                d = dic[code];
                len = d.length;
                for (i = 0; i < len; i++) { pixelBuf[pixelPos++] = d[i] }
                if (dic.length === (1 << size) && size < 12) { size++ }
            }
        }
    };
    function parseColourTable(count) { // get a colour table of length count  Each entry is 3 bytes, for RGB.
        var colours = [];
        for (var i = 0; i < count; i++) { colours.push([st.data[st.pos++], st.data[st.pos++], st.data[st.pos++]]) }
        return colours;
    }
    function parse (){        // read the header. This is the starting point of the decode and async calls parseBlock
        var bitField;
        st.pos                += 6;  
        gif.width             = (st.data[st.pos++]) + ((st.data[st.pos++]) << 8);
        gif.height            = (st.data[st.pos++]) + ((st.data[st.pos++]) << 8);
        bitField              = st.data[st.pos++];
        gif.colorRes          = (bitField & 0b1110000) >> 4;
        gif.globalColourCount = 1 << ((bitField & 0b111) + 1);
        gif.bgColourIndex     = st.data[st.pos++];
        st.pos++;                    // ignoring pixel aspect ratio. if not 0, aspectRatio = (pixelAspectRatio + 15) / 64
        if (bitField & 0b10000000) { gif.globalColourTable = parseColourTable(gif.globalColourCount) } // global colour flag
        setTimeout(parseBlock, 0);
    }
    function parseAppExt() { // get application specific data. Netscape added iterations and terminator. Ignoring that
        st.pos += 1;
        if ('NETSCAPE' === st.getString(8)) { st.pos += 8 }  // ignoring this data. iterations (word) and terminator (byte)
        else {
            st.pos += 3;            // 3 bytes of string usually "2.0" when identifier is NETSCAPE
            st.readSubBlocks();     // unknown app extension
        }
    };
    function parseGCExt() { // get GC data
        var bitField;
        st.pos++;
        bitField              = st.data[st.pos++];
        gif.disposalMethod    = (bitField & 0b11100) >> 2;
        gif.transparencyGiven = bitField & 0b1 ? true : false; // ignoring bit two that is marked as  userInput???
        gif.delayTime         = (st.data[st.pos++]) + ((st.data[st.pos++]) << 8);
        gif.transparencyIndex = st.data[st.pos++];
        st.pos++;
    };
    function parseImg() {                           // decodes image data to create the indexed pixel image
        var deinterlace, frame, bitField;
        deinterlace = function (width) {                   // de interlace pixel data if needed
            var lines, fromLine, pass, toline;
            lines = pixelBufSize / width;
            fromLine = 0;
            if (interlacedBufSize !== pixelBufSize) {      // create the buffer if size changed or undefined.
                deinterlaceBuf = new Uint8Array(pixelBufSize);
                interlacedBufSize = pixelBufSize;
            }
            for (pass = 0; pass < 4; pass++) {
                for (toLine = interlaceOffsets[pass]; toLine < lines; toLine += interlaceSteps[pass]) {
                    deinterlaceBuf.set(pixelBuf.subArray(fromLine, fromLine + width), toLine * width);
                    fromLine += width;
                }
            }
        };
        frame                = {}
        gif.frames.push(frame);
        frame.disposalMethod = gif.disposalMethod;
        frame.time           = gif.length;
        frame.delay          = gif.delayTime * 10;
        gif.length          += frame.delay;
        if (gif.transparencyGiven) { frame.transparencyIndex = gif.transparencyIndex }
        else { frame.transparencyIndex = undefined }
        frame.leftPos = (st.data[st.pos++]) + ((st.data[st.pos++]) << 8);
        frame.topPos  = (st.data[st.pos++]) + ((st.data[st.pos++]) << 8);
        frame.width   = (st.data[st.pos++]) + ((st.data[st.pos++]) << 8);
        frame.height  = (st.data[st.pos++]) + ((st.data[st.pos++]) << 8);
        bitField      = st.data[st.pos++];
        frame.localColourTableFlag = bitField & 0b10000000 ? true : false; 
        if (frame.localColourTableFlag) { frame.localColourTable = parseColourTable(1 << ((bitField & 0b111) + 1)) }
        if (pixelBufSize !== frame.width * frame.height) { // create a pixel buffer if not yet created or if current frame size is different from previous
            pixelBuf     = new Uint8Array(frame.width * frame.height);
            pixelBufSize = frame.width * frame.height;
        }
        lzwDecode(st.data[st.pos++], st.readSubBlocksB()); // decode the pixels
        if (bitField & 0b1000000) {                        // de interlace if needed
            frame.interlaced = true;
            deinterlace(frame.width);
        } else { frame.interlaced = false }
        processFrame(frame);                               // convert to canvas image
    };
    function processFrame(frame) { // creates a RGBA canvas image from the indexed pixel data.
        var ct, cData, dat, pixCount, ind, useT, i, pixel, pDat, col, frame, ti;
        frame.image        = document.createElement('canvas');
        frame.image.width  = gif.width;
        frame.image.height = gif.height;
        frame.image.ctx    = frame.image.getContext("2d");
        ct = frame.localColourTableFlag ? frame.localColourTable : gif.globalColourTable;
        if (gif.lastFrame === null) { gif.lastFrame = frame }
        useT = (gif.lastFrame.disposalMethod === 2 || gif.lastFrame.disposalMethod === 3) ? true : false;
        if (!useT) { frame.image.ctx.drawImage(gif.lastFrame.image, 0, 0, gif.width, gif.height) }
        cData = frame.image.ctx.getImageData(frame.leftPos, frame.topPos, frame.width, frame.height);
        ti  = frame.transparencyIndex;
        dat = cData.data;
        if (frame.interlaced) { pDat = deinterlaceBuf }
        else { pDat = pixelBuf }
        pixCount = pDat.length;
        ind = 0;
        for (i = 0; i < pixCount; i++) {
            pixel = pDat[i];
            col   = ct[pixel];
            if (ti !== pixel) {
                dat[ind++] = col[0];
                dat[ind++] = col[1];
                dat[ind++] = col[2];
                dat[ind++] = 255;      // Opaque.
            } else
                if (useT) {
                    dat[ind + 3] = 0; // Transparent.
                    ind += 4;
                } else { ind += 4 }
        }
        frame.image.ctx.putImageData(cData, frame.leftPos, frame.topPos);
        gif.lastFrame = frame;
        if (!gif.waitTillDone && typeof gif.onload === "function") { doOnloadEvent() }// if !waitTillDone the call onload now after first frame is loaded
    };
    // **NOT** for commercial use.
    function finnished() { // called when the load has completed
        gif.loading           = false;
        gif.frameCount        = gif.frames.length;
        gif.lastFrame         = null;
        st                    = undefined;
        gif.complete          = true;
        gif.disposalMethod    = undefined;
        gif.transparencyGiven = undefined;
        gif.delayTime         = undefined;
        gif.transparencyIndex = undefined;
        gif.waitTillDone      = undefined;
        pixelBuf              = undefined; // dereference pixel buffer
        deinterlaceBuf        = undefined; // dereference interlace buff (may or may not be used);
        pixelBufSize          = undefined;
        deinterlaceBuf        = undefined;
        gif.currentFrame      = 0;
        if (gif.frames.length > 0) { gif.image = gif.frames[0].image }
        doOnloadEvent();
        if (typeof gif.onloadall === "function") {
            (gif.onloadall.bind(gif))({   type : 'loadall', path : [gif] });
        }
        if (gif.playOnLoad) { gif.play() }
    }
    function canceled () { // called if the load has been cancelled
        finnished();
        if (typeof gif.cancelCallback === "function") { (gif.cancelCallback.bind(gif))({ type : 'canceled', path : [gif] }) }
    }
    function parseExt() {              // parse extended blocks
        const blockID = st.data[st.pos++];
        if(blockID === GIF_FILE.GCExt) { parseGCExt() }
        else if(blockID === GIF_FILE.COMMENT) { gif.comment += st.readSubBlocks() }
        else if(blockID === GIF_FILE.APPExt) { parseAppExt() }
        else {
            if(blockID === GIF_FILE.UNKNOWN) { st.pos += 13; } // skip unknow block
            st.readSubBlocks();
        }

    }
    function parseBlock() { // parsing the blocks
        if (gif.cancel !== undefined && gif.cancel === true) { canceled(); return }

        const blockId = st.data[st.pos++];
        if(blockId === GIF_FILE.IMAGE ){ // image block
            parseImg();
            if (gif.firstFrameOnly) { finnished(); return }
        }else if(blockId === GIF_FILE.EOF) { finnished(); return }
        else { parseExt() }
        if (typeof gif.onprogress === "function") {
            gif.onprogress({ bytesRead  : st.pos, totalBytes : st.data.length, frame : gif.frames.length });
        }
        setTimeout(parseBlock, 0); // parsing frame async so processes can get some time in.
    };
    function cancelLoad(callback) { // cancels the loading. This will cancel the load before the next frame is decoded
        if (gif.complete) { return false }
        gif.cancelCallback = callback;
        gif.cancel         = true;
        return true;
    }
    function error(type) {
        if (typeof gif.onerror === "function") { (gif.onerror.bind(this))({ type : type, path : [this] }) }
        gif.onload  = gif.onerror = undefined;
        gif.loading = false;
    }
    function doOnloadEvent() { // fire onload event if set
        gif.currentFrame = 0;
        gif.nextFrameAt  = gif.lastFrameAt  = new Date().valueOf(); // just sets the time now
        if (typeof gif.onload === "function") { (gif.onload.bind(gif))({ type : 'load', path : [gif] }) }
        gif.onerror = gif.onload  = undefined;
    }
    function dataLoaded(data) { // Data loaded create stream and parse
        st = new Stream(data);
        parse();
    }
    function loadGif(filename) { // starts the load
        var ajax = new XMLHttpRequest();
        ajax.responseType = "arraybuffer";
        ajax.onload = function (e) {
            if (e.target.status === 404) { error("File not found") }
            else if(e.target.status >= 200 && e.target.status < 300 ) { dataLoaded(ajax.response) }
            else { error("Loading error : " + e.target.status) }
        };
        ajax.open('GET', filename, true);
        ajax.send();
        ajax.onerror = function (e) { error("File error") };
        this.src = filename;
        this.loading = true;
    }
    function play() { // starts play if paused
        if (!gif.playing) {
            gif.paused  = false;
            gif.playing = true;
            playing();
        }
    }
    function pause() { // stops play
        gif.paused  = true;
        gif.playing = false;
        clearTimeout(timerID);
    }
    function togglePlay(){
        if(gif.paused || !gif.playing){ gif.play() }
        else{ gif.pause() }
    }
    function seekFrame(frame) { // seeks to frame number.
        clearTimeout(timerID);
        gif.currentFrame = frame % gif.frames.length;
        if (gif.playing) { playing() }
        else { gif.image = gif.frames[gif.currentFrame].image }
    }
    function seek(time) { // time in Seconds  // seek to frame that would be displayed at time
        clearTimeout(timerID);
        if (time < 0) { time = 0 }
        time *= 1000; // in ms
        time %= gif.length;
        var frame = 0;
        while (time > gif.frames[frame].time + gif.frames[frame].delay && frame < gif.frames.length) {  frame += 1 }
        gif.currentFrame = frame;
        if (gif.playing) { playing() }
        else { gif.image = gif.frames[gif.currentFrame].image}
    }
    function playing() {
        var delay;
        var frame;
        if (gif.playSpeed === 0) {
            gif.pause();
            return;
        } else {
            if (gif.playSpeed < 0) {
                gif.currentFrame -= 1;
                if (gif.currentFrame < 0) {gif.currentFrame = gif.frames.length - 1 }
                frame = gif.currentFrame;
                frame -= 1;
                if (frame < 0) {  frame = gif.frames.length - 1 }
                delay = -gif.frames[frame].delay * 1 / gif.playSpeed;
            } else {
                gif.currentFrame += 1;
                gif.currentFrame %= gif.frames.length;
                delay = gif.frames[gif.currentFrame].delay * 1 / gif.playSpeed;
            }
            gif.image = gif.frames[gif.currentFrame].image;
            timerID = setTimeout(playing, delay);
        }
    }
    var gif = {                      // the gif image object
        onload         : null,       // fire on load. Use waitTillDone = true to have load fire at end or false to fire on first frame
        onerror        : null,       // fires on error
        onprogress     : null,       // fires a load progress event
        onloadall      : null,       // event fires when all frames have loaded and gif is ready
        paused         : false,      // true if paused
        playing        : false,      // true if playing
        waitTillDone   : true,       // If true onload will fire when all frames loaded, if false, onload will fire when first frame has loaded
        loading        : false,      // true if still loading
        firstFrameOnly : false,      // if true only load the first frame
        width          : null,       // width in pixels
        height         : null,       // height in pixels
        frames         : [],         // array of frames
        comment        : "",         // comments if found in file. Note I remember that some gifs have comments per frame if so this will be all comment concatenated
        length         : 0,          // gif length in ms (1/1000 second)
        currentFrame   : 0,          // current frame. 
        frameCount     : 0,          // number of frames
        playSpeed      : 1,          // play speed 1 normal, 2 twice 0.5 half, -1 reverse etc...
        lastFrame      : null,       // temp hold last frame loaded so you can display the gif as it loads
        image          : null,       // the current image at the currentFrame
        playOnLoad     : true,       // if true starts playback when loaded
        // functions
        load           : loadGif,    // call this to load a file
        cancel         : cancelLoad, // call to stop loading
        play           : play,       // call to start play
        pause          : pause,      // call to pause
        seek           : seek,       // call to seek to time
        seekFrame      : seekFrame,  // call to seek to frame
        togglePlay     : togglePlay, // call to toggle play and pause state
    };
    return gif;
}
















/*=========================================================================
End of gif reader

*/