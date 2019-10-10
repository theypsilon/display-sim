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

import Globals from '../globals';
import Logger from '../logger';

import { prepareMainPage } from '../main_page/main_page';

import { makeVisibility } from '../visibility';
import { makeStorage } from '../storage';

let selectedInfoPanelDeo = Globals.infoPanelBasicDeo;

const getGlCanvasDeo = () => document.getElementById(Globals.glCanvasHtmlId);
const getTopMessageDeo = () => document.getElementById(Globals.topMessageHtmlId);

const visibility = makeVisibility();
const storage = makeStorage();

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
    prepareMainPage();
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
    Logger.log('top_message: ' + event.detail);
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
        throw new Error('Wrong preset value: ' + presetValue);
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
            throw new Error('Wrong preset value: ' + presetValue);
        }
        presetsDeo.value = presetValue;
    }, false);
}
