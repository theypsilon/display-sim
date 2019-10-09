/* Copyright (c) 2019 José manuel Barroso Galindo <theypsilon@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>. */

import * as fastgif from 'fastgif/fastgif';
import FontFaceObserver from 'fontfaceobserver';
import Globals from './globals';

const displaySimPromise = import('./wasm/display_sim');
let selectedInfoPanelDeo = Globals.infoPanelBasicDeo;

const getGlCanvasDeo = () => document.getElementById(Globals.glCanvasHtmlId);
const getTopMessageDeo = () => document.getElementById(Globals.topMessageHtmlId);

const isRunningOnMobileDevice = mobileAndTabletCheck();
const visibility = makeVisibility();
const storage = makeStorage();
const gifCache = {};
window.gifCache = gifCache;
let previewDeo = document.getElementById(Globals.firstPreviewImageId);
let simulationResources;

window.ondrop = event => {
    event.preventDefault();
};

window.ondragover = event => {
    event.preventDefault();
    event.dataTransfer.dropEffect = 'none';
};

window.addEventListener('resize', fixCanvasSize);

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
    a.download = 'Display-Sim_' + new Date().toISOString() + '.png';
    a.click();
    setTimeout(() => {
        URL.revokeObjectURL(url);
        a.remove();
    }, 3000);
}, false);

window.addEventListener('app-event.fps', event => {
    Globals.fpsCounterDeo.innerHTML = Math.round(event.detail);
}, false);

window.addEventListener('app-event.top_message', event => {
    const existingTopMessage = getTopMessageDeo();
    if (existingTopMessage) {
        existingTopMessage.remove();
    }
    const div = document.createElement('div');
    div.id = Globals.topMessageHtmlId;
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
    Globals.cameraPosXDeo.value = Math.round(event.detail[0] * 100) / 100;
    Globals.cameraPosYDeo.value = Math.round(event.detail[1] * 100) / 100;
    Globals.cameraPosZDeo.value = Math.round(event.detail[2] * 100) / 100;
    Globals.cameraDirXDeo.value = Math.round(event.detail[3] * 100) / 100;
    Globals.cameraDirYDeo.value = Math.round(event.detail[4] * 100) / 100;
    Globals.cameraDirZDeo.value = Math.round(event.detail[5] * 100) / 100;
    Globals.cameraAxisUpXDeo.value = Math.round(event.detail[6] * 100) / 100;
    Globals.cameraAxisUpYDeo.value = Math.round(event.detail[7] * 100) / 100;
    Globals.cameraAxisUpZDeo.value = Math.round(event.detail[8] * 100) / 100;
}, false);

[
    { deo: Globals.cameraZoomDeo, eventId: 'app-event.change_camera_zoom' },
    { deo: Globals.cameraMovementModeDeo, eventId: 'app-event.change_camera_movement_mode' },
    { deo: Globals.pixelWidthDeo, eventId: 'app-event.change_pixel_width' },
    { deo: Globals.pixelHorizontalGapDeo, eventId: 'app-event.change_pixel_horizontal_gap' },
    { deo: Globals.pixelVerticalGapDeo, eventId: 'app-event.change_pixel_vertical_gap' },
    { deo: Globals.pixelSpreadDeo, eventId: 'app-event.change_pixel_spread' },
    { deo: Globals.pixelBrigthnessDeo, eventId: 'app-event.change_pixel_brightness' },
    { deo: Globals.pixelContrastDeo, eventId: 'app-event.change_pixel_contrast' },
    { deo: Globals.blurLevelDeo, eventId: 'app-event.change_blur_level' },
    { deo: Globals.verticalLppDeo, eventId: 'app-event.change_vertical_lpp' },
    { deo: Globals.horizontalLppDeo, eventId: 'app-event.change_horizontal_lpp' },
    { deo: Globals.lightColorDeo, eventId: 'app-event.change_light_color' },
    { deo: Globals.brightnessColorDeo, eventId: 'app-event.change_brightness_color' },
    { deo: Globals.featureChangeMoveSpeedDeo, eventId: 'app-event.change_movement_speed' },
    { deo: Globals.featureChangePixelSpeedDeo, eventId: 'app-event.change_pixel_speed' },
    { deo: Globals.featureChangeTurnSpeedDeo, eventId: 'app-event.change_turning_speed' },

    { deo: Globals.featureChangeColorRepresentationDeo, eventId: 'app-event.color_representation' },
    { deo: Globals.featureChangePixelGeometryDeo, eventId: 'app-event.pixel_geometry' },
    { deo: Globals.featureChangePixelShadowShapeDeo, eventId: 'app-event.pixel_shadow_shape' },
    { deo: Globals.featureChangePixelShadowHeightDeo, eventId: 'app-event.pixel_shadow_height' },
    { deo: Globals.featureBacklightPercentDeo, eventId: 'app-event.backlight_percent' },
    { deo: Globals.featureInternalResolutionDeo, eventId: 'app-event.internal_resolution' },
    { deo: Globals.featureInternalResolutionBasicDeo, eventId: 'app-event.internal_resolution' },
    { deo: Globals.featureTextureInterpolationDeo, eventId: 'app-event.texture_interpolation' },
    { deo: Globals.featureChangeScreenCurvatureDeo, eventId: 'app-event.screen_curvature' },
    { deo: Globals.featureChangeScreenCurvatureBasicDeo, eventId: 'app-event.screen_curvature' }
].forEach(({ deo, eventId }) => {
    if (!deo) throw new Error('Wrong deo on defining: ' + eventId);
    window.addEventListener(eventId, event => {
        deo.value = event.detail;
        if (event.id === 'app-event.change_camera_movement_mode') {
            switch (event.detail) {
            case 'Lock on Display':
                deo.title = 'The camera will move around the picture, always looking at it';
                Globals.freeModeControlsClas.forEach(deo => deo.classList.add(Globals.displayNoneClassName));
                break;
            case 'Free Flight':
                deo.title = 'The camera can move without any restriction in the whole 3D space with plane-like controls';
                Globals.freeModeControlsClas.forEach(deo => deo.classList.remove(Globals.displayNoneClassName));
                break;
            default:
                throw new Error('Unreachable!');
            }
        }
    }, false);
});

customEventOnButtonPressed(Globals.featureCameraMovementsDeo);
customEventOnButtonPressed(Globals.featureCameraTurnsDeo);
function customEventOnButtonPressed (deo) {
    deo.querySelectorAll('.activate-button').forEach(button => {
        const eventOptions = { key: button.innerHTML.toLowerCase() };
        button.onmousedown = () => document.dispatchEvent(new KeyboardEvent('keydown', eventOptions));
        button.onmouseup = () => document.dispatchEvent(new KeyboardEvent('keyup', eventOptions));
    });
}

customEventOnChange(Globals.cameraPosXDeo, 'camera_pos_x', a => +a);
customEventOnChange(Globals.cameraPosYDeo, 'camera_pos_y', a => +a);
customEventOnChange(Globals.cameraPosZDeo, 'camera_pos_z', a => +a);
customEventOnChange(Globals.cameraAxisUpXDeo, 'camera_axis_up_x', a => +a);
customEventOnChange(Globals.cameraAxisUpYDeo, 'camera_axis_up_y', a => +a);
customEventOnChange(Globals.cameraAxisUpZDeo, 'camera_axis_up_z', a => +a);
customEventOnChange(Globals.cameraDirXDeo, 'camera_direction_x', a => +a);
customEventOnChange(Globals.cameraDirYDeo, 'camera_direction_y', a => +a);
customEventOnChange(Globals.cameraDirZDeo, 'camera_direction_z', a => +a);
customEventOnChange(Globals.cameraZoomDeo, 'camera_zoom', a => +a);
customEventOnChange(Globals.cameraMovementModeDeo, 'camera_movement_mode', a => +a);

customEventOnChange(Globals.pixelWidthDeo, 'pixel_width', a => +a);
customEventOnChange(Globals.pixelSpreadDeo, 'pixel_spread', a => +a);
customEventOnChange(Globals.pixelHorizontalGapDeo, 'pixel_horizontal_gap', a => +a);
customEventOnChange(Globals.pixelVerticalGapDeo, 'pixel_vertical_gap', a => +a);
customEventOnChange(Globals.blurLevelDeo, 'blur_level', a => +a);
customEventOnChange(Globals.verticalLppDeo, 'vertical_lpp', a => +a);
customEventOnChange(Globals.horizontalLppDeo, 'horizontal_lpp', a => +a);
customEventOnChange(Globals.pixelBrigthnessDeo, 'pixel_brightness', a => +a);
customEventOnChange(Globals.pixelContrastDeo, 'pixel_contrast', a => +a);
customEventOnChange(Globals.featureChangePixelShadowHeightDeo, 'pixel_shadow_height', a => +a);
customEventOnChange(Globals.featureBacklightPercentDeo, 'backlight_percent', a => +a);
const parseColor = (value) => parseInt('0x' + value.substring(1));
customEventOnChange(Globals.lightColorDeo, 'light_color', parseColor);
customEventOnChange(Globals.brightnessColorDeo, 'brightness_color', parseColor);
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
    Globals.featureChangeColorRepresentationDeo,
    Globals.featureChangePixelGeometryDeo,
    Globals.featureChangePixelShadowShapeDeo,
    Globals.featureChangePixelShadowHeightDeo,
    Globals.featureBacklightPercentDeo,
    Globals.featureChangeScreenCurvatureDeo,
    Globals.featureQuitDeo,
    Globals.featureCaptureFramebufferDeo,
    Globals.featureClosePanelDeo
].forEach(deo => {
    deo.onmousedown = () => document.dispatchEvent(new KeyboardEvent('keydown', { key: deo.id }));
    deo.onmouseup = () => document.dispatchEvent(new KeyboardEvent('keyup', { key: deo.id }));
});

[
    Globals.resetCameraDeo,
    Globals.resetSpeedsDeo,
    Globals.resetFiltersDeo
].forEach(deo => {
    deo.onclick = () => document.dispatchEvent(new KeyboardEvent('keydown', { key: deo.id }));
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

Globals.toggleInfoPanelClass.forEach(deo => {
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

const settingsTabs = document.querySelectorAll('.tabs > li');
settingsTabs.forEach(clickedTab => {
    clickedTab.addEventListener('click', () => {
        settingsTabs.forEach(tab => {
            tab.classList.remove('active');
        });
        clickedTab.classList.add('active');
        selectedInfoPanelDeo.classList.add('display-none');
        switch (clickedTab.id) {
        case 'panel-basic':
            selectedInfoPanelDeo = Globals.infoPanelBasicDeo;
            break;
        case 'panel-advanced':
            selectedInfoPanelDeo = Globals.infoPanelAdvancedDeo;
            break;
        default:
            console.error('Unknown clicked tab: ' + clickedTab.id);
            break;
        }
        selectedInfoPanelDeo.classList.remove('display-none');
    });
});

Globals.filterPresetsButtonDeoList.forEach(deo => {
    deo.onclick = function () {
        Globals.filterPresetsButtonDeoList.forEach(otherDeo => {
            otherDeo.classList.remove('active-preset');
        });
        deo.classList.add('active-preset');
        window.dispatchEvent(new CustomEvent('app-event.custom_input_event', {
            detail: {
                value: deo.dataset.preset,
                kind: 'event_kind:filter_presets_selected'
            }
        }));
    };
});

window.addEventListener('app-event.preset_selected_name', event => {
    const presetValue = event.detail.toLowerCase().replace(/\s/g, '-');
    if (!Globals.properPresets.includes(presetValue)) {
        throw new Error('Wrong preset value:', presetValue);
    }
    Globals.filterPresetsButtonDeoList.forEach(deo => {
        if (deo.dataset.preset === presetValue) {
            deo.classList.add('active-preset');
        } else {
            deo.classList.remove('active-preset');
        }
    });
}, false);

configurePresetsDeo(Globals.filterPresetsDeo);
// configurePresetsDeo(Globals.filterPresetsBasicDeo);
function configurePresetsDeo (presetsDeo) {
    presetsDeo.onchange = () => {
        if (presetsDeo.value === Globals.presetCustom) {
            visibility.showFilterOptionMainList();
        } else if (Globals.properPresets.includes(presetsDeo.value)) {
            visibility.showFilterOptionMainList();
        } else {
            presetsDeo.value = Globals.presetApertureGrille1;
        }
        storage.setFilterPresets(presetsDeo.value);
        window.dispatchEvent(new CustomEvent('app-event.custom_input_event', {
            detail: {
                value: presetsDeo.value,
                kind: 'event_kind:filter_presets_selected'
            }
        }));
    };

    window.addEventListener('app-event.preset_selected_name', event => {
        const presetValue = event.detail.toLowerCase().replace(/\s/g, '-');
        if (!Globals.properPresets.includes(presetValue)) {
            throw new Error('Wrong preset value:', presetValue);
        }
        presetsDeo.value = presetValue;
    }, false);
}

Globals.inputFileUploadDeo.onchange = () => {
    const file = Globals.inputFileUploadDeo.files[0];
    const url = (window.URL || window.webkitURL).createObjectURL(file);
    processFileToUpload(url);
};

Globals.dropZoneDeo.onclick = () => {
    Globals.inputFileUploadDeo.click();
};
Globals.dropZoneDeo.ondragover = event => {
    event.stopPropagation();
    event.preventDefault();
    event.dataTransfer.dropEffect = 'copy';
};
Globals.dropZoneDeo.ondrop = event => {
    event.stopPropagation();
    event.preventDefault();
    var file = event.dataTransfer.files[0];
    const url = (window.URL || window.webkitURL).createObjectURL(file);
    processFileToUpload(url);
};
Globals.optionScalingSelect.onchange = () => {
    if (Globals.optionScalingSelect.value === Globals.scalingCustomHtmlId) {
        visibility.showScaleCustomInputs();
    } else {
        visibility.hideScaleCustomInputs();
    }
};

Globals.restoreDefaultOptionsDeo.onclick = () => {
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
        Globals.startAnimationDeo.disabled = true;
        Globals.startAnimationDeo.title = 'You need a PC with NVIDIA or ATI graphics card with updated drivers and a WebGL2 compatible browser (Firefox, Opera or Chrome) in order to run this without problems.';
        return;
    }

    await new Promise(resolve => {
        Globals.startAnimationDeo.onclick = resolve;
    });

    visibility.hideUi();
    visibility.showLoading();

    await new Promise(resolve => setTimeout(resolve, 50));

    const rawImgs = await (async function () {
        if (previewDeo.id === Globals.firstPreviewImageId) {
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
                benchmark('decoding...');
                const decoder = new fastgif.Decoder();
                gif = await window.fetch(img.src)
                    .then(response => response.arrayBuffer())
                    .then(buffer => decoder.decode(buffer));
                gifCache[gifKey] = gif;
            }
            benchmark('gif loaded', gif);
            return gif.map(frame => ({
                raw: frame.imageData,
                delay: frame.delay
            }));
        }
    }());

    benchmark('image readed');

    let scaleX = 1;
    let stretch = false;
    storage.setScalingSelectOption(Globals.optionScalingSelect.value);

    const imageWidth = rawImgs[0].raw.width;
    const imageHeight = rawImgs[0].raw.height;
    let backgroundWidth = imageWidth;
    let backgroundHeight = imageHeight;

    switch (Globals.optionScalingSelect.value) {
    case Globals.scalingAutoHtmlId:
        const autoScaling = calculateAutoScaling(imageWidth, imageHeight);
        scaleX = autoScaling.scaleX;
        window.dispatchEvent(new CustomEvent('app-event.top_message', {
            detail: 'Scaling auto detect: ' + autoScaling.message
        }));
        break;
    case Globals.scaling43HtmlId:
        scaleX = (4 / 3) / (imageWidth / imageHeight);
        break;
    case Globals.scalingStretchToBothEdgesHtmlId:
        scaleX = (window.screen.width / window.screen.height) / (imageWidth / imageHeight);
        stretch = true;
        break;
    case Globals.scalingStretchToNearestEdgeHtmlId:
        stretch = true;
        break;
    case Globals.scalingCustomHtmlId:
        stretch = Globals.scalingCustomStretchNearestDeo.checked;
        storage.setCustomResWidth(Globals.scalingCustomResWidthDeo.value);
        storage.setCustomResHeight(Globals.scalingCustomResHeightDeo.value);
        storage.setCustomArX(Globals.scalingCustomArXDeo.value);
        storage.setCustomArY(Globals.scalingCustomArYDeo.value);
        storage.setCustomStretchNearest(stretch);
        backgroundWidth = +Globals.scalingCustomResWidthDeo.value;
        backgroundHeight = +Globals.scalingCustomResHeightDeo.value;
        scaleX = (+Globals.scalingCustomArXDeo.value / +Globals.scalingCustomArYDeo.value) / (backgroundWidth / backgroundHeight);
        break;
    }

    Globals.lightColorDeo.value = '#FFFFFF';
    Globals.brightnessColorDeo.value = '#FFFFFF';

    const canvas = document.createElement('canvas');

    canvas.id = Globals.glCanvasHtmlId;

    fixCanvasSize(canvas);

    canvas.onfocus = () => document.dispatchEvent(new KeyboardEvent('keydown', { key: 'canvas_focused' }));
    canvas.onblur = () => document.dispatchEvent(new KeyboardEvent('keyup', { key: 'canvas_focused' }));

    document.body.appendChild(canvas);

    const ctxOptions = {
        alpha: false,
        antialias: Globals.antialiasDeo.checked,
        depth: true,
        failIfMajorPerformanceCaveat: false,
        powerPreference: Globals.optionPowerPreferenceSelect.value,
        premultipliedAlpha: false,
        preserveDrawingBuffer: false,
        stencil: false
    };

    storage.setAntiAliasing(ctxOptions.antialias);
    storage.setPowerPreferenceSelectOption(Globals.optionPowerPreferenceSelect.value);

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

    const displaySim = await displaySimPromise;

    const videoInput = new displaySim.VideoInputWasm(
        imageWidth, imageHeight, // to read the image pixels
        canvas.width, canvas.height // gl.viewport
    );
    if (backgroundWidth !== imageWidth) {
        videoInput.set_background_size(backgroundWidth, backgroundHeight); // to calculate model distance to the camera
    }
    videoInput.set_pixel_width(scaleX);
    if (stretch === true) {
        videoInput.stretch();
    }
    videoInput.set_max_texture_size(gl.getParameter(gl.MAX_TEXTURE_SIZE));
    for (let i = 0; i < rawImgs.length; i++) {
        const rawImg = rawImgs[i];
        videoInput.add_picture_frame(new Uint8Array(rawImg.raw.data.buffer), rawImg.delay);
    }

    if (simulationResources === undefined) {
        benchmark('calling wasm load_simulation_resources');
        simulationResources = displaySim.load_simulation_resources();
        benchmark('wasm load_simulation_resources done');
    }
    benchmark('calling wasm run_program');
    displaySim.run_program(gl, simulationResources, videoInput);
    benchmark('wasm run_program done');

    Globals.filterPresetsDeo.value = storage.getFilterPresets();
    Globals.filterPresetsDeo.onchange();

    //    Globals.filterPresetsBasicDeo.value = storage.getFilterPresets();
    //    Globals.filterPresetsBasicDeo.onchange();

    visibility.hideLoading();
    visibility.showSimulationUi();
}

function loadInputValuesFromStorage () {
    Globals.optionScalingSelect.value = storage.getScalingSelectOption();
    Globals.optionPowerPreferenceSelect.value = storage.getPowerPreferenceSelectOption();
    if (Globals.optionScalingSelect.value === Globals.scalingCustomHtmlId) {
        visibility.showScaleCustomInputs();
    } else {
        visibility.hideScaleCustomInputs();
    }
    Globals.scalingCustomResWidthDeo.value = storage.getCustomResWidth();
    Globals.scalingCustomResHeightDeo.value = storage.getCustomResHeight();
    Globals.scalingCustomArXDeo.value = storage.getCustomArX();
    Globals.scalingCustomArYDeo.value = storage.getCustomArY();
    Globals.scalingCustomStretchNearestDeo.checked = storage.getCustomStretchNearest();
    Globals.antialiasDeo.checked = storage.getAntiAliasing();
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
    Globals.scalingCustomResButtonDeo.value = 'Set to ' + width + ' ✕ ' + height;
    Globals.scalingCustomResButtonDeo.onclick = () => {
        Globals.scalingCustomResWidthDeo.value = width;
        Globals.scalingCustomResHeightDeo.value = height;
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
    Globals.selectImageList.insertBefore(li, Globals.dropZoneDeo);
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
    const optionFilterPresets = 'option-filter-presets';
    return {
        getScalingSelectOption: () => localStorage.getItem(optionScalingSelect) || Globals.scalingAutoHtmlId,
        setScalingSelectOption: option => localStorage.setItem(Globals.optionScalingSelect, option),
        getPowerPreferenceSelectOption: () => localStorage.getItem(optionPowerPreferenceSelect) || Globals.powerPreferenceDefaultHtml,
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
        getFilterPresets: () => localStorage.getItem(optionFilterPresets) || Globals.presetApertureGrille1,
        setFilterPresets: filterPresets => localStorage.setItem(optionFilterPresets, filterPresets),
        removeAllOptions: () => {
            localStorage.removeItem(Globals.optionScalingSelect);
            localStorage.removeItem(Globals.optionPowerPreferenceSelect);
            localStorage.removeItem(optionScalingCustomResWidth);
            localStorage.removeItem(optionScalingCustomResHeight);
            localStorage.removeItem(optionScalingCustomArX);
            localStorage.removeItem(optionScalingCustomArY);
            localStorage.removeItem(optionScalingCustomStretchNearest);
            localStorage.removeItem(optionAntialias);
            localStorage.removeItem(optionFilterPresets);
        }
    };
}

function makeVisibility () {
    return {
        showUi: () => showElement(Globals.uiDeo),
        hideUi: () => hideElement(Globals.uiDeo),
        showLoading: () => showElement(Globals.loadingDeo),
        hideLoading: () => hideElement(Globals.loadingDeo),
        showSimulationUi: () => {
            document.body.style.setProperty('overflow', 'hidden');
            document.body.style.setProperty('background-color', 'black');
            showElement(Globals.simulationUiDeo);
        },
        hideSimulationUi: () => {
            document.body.style.removeProperty('overflow');
            document.body.style.removeProperty('background-color');
            hideElement(Globals.simulationUiDeo);
        },
        showInfoPanel: () => showElement(Globals.infoPanelDeo),
        hideInfoPanel: () => hideElement(Globals.infoPanelDeo),
        isInfoPanelVisible: () => isVisible(Globals.infoPanelDeo),
        showFilterOptionMainList: () => showElement(Globals.filterOptionMainListDeo),
        hideFilterOptionMainList: () => hideElement(Globals.filterOptionMainListDeo),
        showScalingCustomResButton: () => showElement(Globals.scalingCustomResButtonDeo),
        showScaleCustomInputs: () => showElement(Globals.scalingCustomInputsDeo),
        hideScaleCustomInputs: () => hideElement(Globals.scalingCustomInputsDeo)
    };
    function showElement (element) {
        element.classList.remove(Globals.displayNoneClassName);
    }
    function hideElement (element) {
        element.classList.add(Globals.displayNoneClassName);
    }
    function isVisible (element) {
        return element.classList.contains(Globals.displayNoneClassName) === false;
    }
}

function fixCanvasSize (canvas) {
    canvas = canvas instanceof HTMLCanvasElement ? canvas : document.getElementById(Globals.glCanvasHtmlId);
    if (!canvas) return;

    const dpi = window.devicePixelRatio;
    const width = window.screen.width;
    const height = window.screen.height;
    const zoom = window.outerWidth / window.innerWidth;

    canvas.width = Math.round(width * dpi / zoom / 80) * 80;
    canvas.height = Math.round(height * dpi / zoom / 60) * 60;

    canvas.style.width = window.innerWidth;
    canvas.style.height = window.innerHeight;

    benchmark('resolution:', canvas.width, canvas.height, width, height);

    const infoPanelContentHeight = (window.innerHeight - 18) * 0.95;
    Globals.infoPanelContentDeo.style.setProperty('max-height', infoPanelContentHeight);
    Globals.infoPanelAdvancedSettingsDeo.style.setProperty('max-height', infoPanelContentHeight - 60);
}

function mobileAndTabletCheck () {
    var check = false;
    // eslint-disable-next-line no-useless-escape
    (function (a) { if (/(android|bb\d+|meego).+mobile|avantgo|bada\/|blackberry|blazer|compal|elaine|fennec|hiptop|iemobile|ip(hone|od)|iris|kindle|lge |maemo|midp|mmp|mobile.+firefox|netfront|opera m(ob|in)i|palm( os)?|phone|p(ixi|re)\/|plucker|pocket|psp|series(4|6)0|symbian|treo|up\.(browser|link)|vodafone|wap|windows ce|xda|xiino|android|ipad|playbook|silk/i.test(a) || /1207|6310|6590|3gso|4thp|50[1-6]i|770s|802s|a wa|abac|ac(er|oo|s\-)|ai(ko|rn)|al(av|ca|co)|amoi|an(ex|ny|yw)|aptu|ar(ch|go)|as(te|us)|attw|au(di|\-m|r |s )|avan|be(ck|ll|nq)|bi(lb|rd)|bl(ac|az)|br(e|v)w|bumb|bw\-(n|u)|c55\/|capi|ccwa|cdm\-|cell|chtm|cldc|cmd\-|co(mp|nd)|craw|da(it|ll|ng)|dbte|dc\-s|devi|dica|dmob|do(c|p)o|ds(12|\-d)|el(49|ai)|em(l2|ul)|er(ic|k0)|esl8|ez([4-7]0|os|wa|ze)|fetc|fly(\-|_)|g1 u|g560|gene|gf\-5|g\-mo|go(\.w|od)|gr(ad|un)|haie|hcit|hd\-(m|p|t)|hei\-|hi(pt|ta)|hp( i|ip)|hs\-c|ht(c(\-| |_|a|g|p|s|t)|tp)|hu(aw|tc)|i\-(20|go|ma)|i230|iac( |\-|\/)|ibro|idea|ig01|ikom|im1k|inno|ipaq|iris|ja(t|v)a|jbro|jemu|jigs|kddi|keji|kgt( |\/)|klon|kpt |kwc\-|kyo(c|k)|le(no|xi)|lg( g|\/(k|l|u)|50|54|\-[a-w])|libw|lynx|m1\-w|m3ga|m50\/|ma(te|ui|xo)|mc(01|21|ca)|m\-cr|me(rc|ri)|mi(o8|oa|ts)|mmef|mo(01|02|bi|de|do|t(\-| |o|v)|zz)|mt(50|p1|v )|mwbp|mywa|n10[0-2]|n20[2-3]|n30(0|2)|n50(0|2|5)|n7(0(0|1)|10)|ne((c|m)\-|on|tf|wf|wg|wt)|nok(6|i)|nzph|o2im|op(ti|wv)|oran|owg1|p800|pan(a|d|t)|pdxg|pg(13|\-([1-8]|c))|phil|pire|pl(ay|uc)|pn\-2|po(ck|rt|se)|prox|psio|pt\-g|qa\-a|qc(07|12|21|32|60|\-[2-7]|i\-)|qtek|r380|r600|raks|rim9|ro(ve|zo)|s55\/|sa(ge|ma|mm|ms|ny|va)|sc(01|h\-|oo|p\-)|sdk\/|se(c(\-|0|1)|47|mc|nd|ri)|sgh\-|shar|sie(\-|m)|sk\-0|sl(45|id)|sm(al|ar|b3|it|t5)|so(ft|ny)|sp(01|h\-|v\-|v )|sy(01|mb)|t2(18|50)|t6(00|10|18)|ta(gt|lk)|tcl\-|tdg\-|tel(i|m)|tim\-|t\-mo|to(pl|sh)|ts(70|m\-|m3|m5)|tx\-9|up(\.b|g1|si)|utst|v400|v750|veri|vi(rg|te)|vk(40|5[0-3]|\-v)|vm40|voda|vulc|vx(52|53|60|61|70|80|81|83|85|98)|w3c(\-| )|webc|whit|wi(g |nc|nw)|wmlb|wonu|x700|yas\-|your|zeto|zte\-/i.test(a.substr(0, 4))) check = true; })(navigator.userAgent || navigator.vendor || window.opera);
    return check;
}

function calculateAutoScaling (imageWidth, imageHeight) {
    if (imageHeight > 540) {
        return {
            scaleX: 1,
            message: 'none.'
        };
    } else if (imageHeight === 144) {
        return {
            scaleX: (11 / 10) / (imageWidth / imageHeight),
            message: '11:10 (Game Boy) on full image.'
        };
    } else if (imageHeight === 160) {
        return {
            scaleX: (3 / 2) / (imageWidth / imageHeight),
            message: '3:2 (Game Boy Advance) on full image.'
        };
    } else {
        return {
            scaleX: (4 / 3) / (imageWidth / imageHeight),
            message: '4:3 on full image.'
        };
    }
}

function benchmark () {
    if (!window.display_sim_bench && !window.localStorage.getItem('display_sim_bench')) return;
    console.log(new Date().toISOString(), ...arguments);
}
