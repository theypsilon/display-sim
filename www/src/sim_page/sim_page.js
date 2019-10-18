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

import Constants from '../constants';
import Logger from '../logger';

import { prepareMainPage } from '../main_page/main_page';

import { Visibility } from '../visibility';
import { Storage } from '../storage';

let selectedInfoPanelDeo = Constants.infoPanelBasicDeo;

const getGlCanvasDeo = () => document.getElementById(Constants.glCanvasHtmlId);
const getTopMessageDeo = () => document.getElementById(Constants.topMessageHtmlId);

const visibility = Visibility.make();
const storage = Storage.make();

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
    Constants.fpsCounterDeo.innerHTML = Math.round(event.detail);
}, false);

window.addEventListener('app-event.top_message', event => {
    const existingTopMessage = getTopMessageDeo();
    if (existingTopMessage) {
        existingTopMessage.remove();
    }
    const div = document.createElement('div');
    div.id = Constants.topMessageHtmlId;
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
    Constants.cameraPosXDeo.value = Math.round(event.detail[0] * 100) / 100;
    Constants.cameraPosYDeo.value = Math.round(event.detail[1] * 100) / 100;
    Constants.cameraPosZDeo.value = Math.round(event.detail[2] * 100) / 100;
    Constants.cameraDirXDeo.value = Math.round(event.detail[3] * 100) / 100;
    Constants.cameraDirYDeo.value = Math.round(event.detail[4] * 100) / 100;
    Constants.cameraDirZDeo.value = Math.round(event.detail[5] * 100) / 100;
    Constants.cameraAxisUpXDeo.value = Math.round(event.detail[6] * 100) / 100;
    Constants.cameraAxisUpYDeo.value = Math.round(event.detail[7] * 100) / 100;
    Constants.cameraAxisUpZDeo.value = Math.round(event.detail[8] * 100) / 100;
}, false);

[
    { deo: Constants.cameraZoomDeo, eventId: 'app-event.change_camera_zoom' },
    { deo: Constants.cameraMovementModeDeo, eventId: 'app-event.change_camera_movement_mode' },
    { deo: Constants.pixelWidthDeo, eventId: 'app-event.change_pixel_width' },
    { deo: Constants.pixelHorizontalGapDeo, eventId: 'app-event.change_pixel_horizontal_gap' },
    { deo: Constants.pixelVerticalGapDeo, eventId: 'app-event.change_pixel_vertical_gap' },
    { deo: Constants.pixelSpreadDeo, eventId: 'app-event.change_pixel_spread' },
    { deo: Constants.pixelBrigthnessDeo, eventId: 'app-event.change_pixel_brightness' },
    { deo: Constants.pixelContrastDeo, eventId: 'app-event.change_pixel_contrast' },
    { deo: Constants.blurLevelDeo, eventId: 'app-event.change_blur_level' },
    { deo: Constants.verticalLppDeo, eventId: 'app-event.change_vertical_lpp' },
    { deo: Constants.horizontalLppDeo, eventId: 'app-event.change_horizontal_lpp' },
    { deo: Constants.lightColorDeo, eventId: 'app-event.change_light_color' },
    { deo: Constants.brightnessColorDeo, eventId: 'app-event.change_brightness_color' },
    { deo: Constants.featureChangeMoveSpeedDeo, eventId: 'app-event.change_movement_speed' },
    { deo: Constants.featureChangePixelSpeedDeo, eventId: 'app-event.change_pixel_speed' },
    { deo: Constants.featureChangeTurnSpeedDeo, eventId: 'app-event.change_turning_speed' },

    { deo: Constants.featureChangeColorRepresentationDeo, eventId: 'app-event.color_representation' },
    { deo: Constants.featureChangePixelGeometryDeo, eventId: 'app-event.pixel_geometry' },
    { deo: Constants.featureChangePixelShadowShapeDeo, eventId: 'app-event.pixel_shadow_shape' },
    { deo: Constants.featureChangePixelShadowHeightDeo, eventId: 'app-event.pixel_shadow_height' },
    { deo: Constants.featureBacklightPercentDeo, eventId: 'app-event.backlight_percent' },
    { deo: Constants.featureInternalResolutionDeo, eventId: 'app-event.internal_resolution' },
    { deo: Constants.featureInternalResolutionBasicDeo, eventId: 'app-event.internal_resolution' },
    { deo: Constants.featureTextureInterpolationDeo, eventId: 'app-event.texture_interpolation' },
    { deo: Constants.featureChangeScreenCurvatureDeo, eventId: 'app-event.screen_curvature' },
    { deo: Constants.featureChangeScreenCurvatureBasicDeo, eventId: 'app-event.screen_curvature' }
].forEach(({ deo, eventId }) => {
    if (!deo) throw new Error('Wrong deo on defining: ' + eventId);
    window.addEventListener(eventId, event => {
        deo.value = event.detail;
        if (event.id === 'app-event.change_camera_movement_mode') {
            switch (event.detail) {
            case 'Lock on Display':
                deo.title = 'The camera will move around the picture, always looking at it';
                Constants.freeModeControlsClas.forEach(deo => deo.classList.add(Constants.displayNoneClassName));
                break;
            case 'Free Flight':
                deo.title = 'The camera can move without any restriction in the whole 3D space with plane-like controls';
                Constants.freeModeControlsClas.forEach(deo => deo.classList.remove(Constants.displayNoneClassName));
                break;
            default:
                throw new Error('Unreachable!');
            }
        }
    }, false);
});

customEventOnButtonPressed(Constants.featureCameraMovementsDeo);
customEventOnButtonPressed(Constants.featureCameraTurnsDeo);
function customEventOnButtonPressed (deo) {
    deo.querySelectorAll('.activate-button').forEach(button => {
        const eventOptions = { key: button.innerHTML.toLowerCase() };
        button.onmousedown = () => document.dispatchEvent(new KeyboardEvent('keydown', eventOptions));
        button.onmouseup = () => document.dispatchEvent(new KeyboardEvent('keyup', eventOptions));
    });
}

customEventOnChange(Constants.cameraPosXDeo, 'camera_pos_x', a => +a);
customEventOnChange(Constants.cameraPosYDeo, 'camera_pos_y', a => +a);
customEventOnChange(Constants.cameraPosZDeo, 'camera_pos_z', a => +a);
customEventOnChange(Constants.cameraAxisUpXDeo, 'camera_axis_up_x', a => +a);
customEventOnChange(Constants.cameraAxisUpYDeo, 'camera_axis_up_y', a => +a);
customEventOnChange(Constants.cameraAxisUpZDeo, 'camera_axis_up_z', a => +a);
customEventOnChange(Constants.cameraDirXDeo, 'camera_direction_x', a => +a);
customEventOnChange(Constants.cameraDirYDeo, 'camera_direction_y', a => +a);
customEventOnChange(Constants.cameraDirZDeo, 'camera_direction_z', a => +a);
customEventOnChange(Constants.cameraZoomDeo, 'camera_zoom', a => +a);
customEventOnChange(Constants.cameraMovementModeDeo, 'camera_movement_mode', a => +a);

customEventOnChange(Constants.pixelWidthDeo, 'pixel_width', a => +a);
customEventOnChange(Constants.pixelSpreadDeo, 'pixel_spread', a => +a);
customEventOnChange(Constants.pixelHorizontalGapDeo, 'pixel_horizontal_gap', a => +a);
customEventOnChange(Constants.pixelVerticalGapDeo, 'pixel_vertical_gap', a => +a);
customEventOnChange(Constants.blurLevelDeo, 'blur_level', a => +a);
customEventOnChange(Constants.verticalLppDeo, 'vertical_lpp', a => +a);
customEventOnChange(Constants.horizontalLppDeo, 'horizontal_lpp', a => +a);
customEventOnChange(Constants.pixelBrigthnessDeo, 'pixel_brightness', a => +a);
customEventOnChange(Constants.pixelContrastDeo, 'pixel_contrast', a => +a);
customEventOnChange(Constants.featureChangePixelShadowHeightDeo, 'pixel_shadow_height', a => +a);
customEventOnChange(Constants.featureBacklightPercentDeo, 'backlight_percent', a => +a);
const parseColor = (value) => parseInt('0x' + value.substring(1));
customEventOnChange(Constants.lightColorDeo, 'light_color', parseColor);
customEventOnChange(Constants.brightnessColorDeo, 'brightness_color', parseColor);
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
    Constants.featureChangeColorRepresentationDeo,
    Constants.featureChangePixelGeometryDeo,
    Constants.featureChangePixelShadowShapeDeo,
    Constants.featureChangePixelShadowHeightDeo,
    Constants.featureBacklightPercentDeo,
    Constants.featureChangeScreenCurvatureDeo,
    Constants.featureQuitDeo,
    Constants.featureCaptureFramebufferDeo,
    Constants.featureClosePanelDeo
].forEach(deo => {
    deo.onmousedown = () => document.dispatchEvent(new KeyboardEvent('keydown', { key: deo.id }));
    deo.onmouseup = () => document.dispatchEvent(new KeyboardEvent('keyup', { key: deo.id }));
});

[
    Constants.resetCameraDeo,
    Constants.resetSpeedsDeo,
    Constants.resetFiltersDeo
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

Constants.toggleInfoPanelClass.forEach(deo => {
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
            selectedInfoPanelDeo = Constants.infoPanelBasicDeo;
            break;
        case 'panel-advanced':
            selectedInfoPanelDeo = Constants.infoPanelAdvancedDeo;
            break;
        default:
            console.error('Unknown clicked tab: ' + clickedTab.id);
            break;
        }
        selectedInfoPanelDeo.classList.remove('display-none');
    });
});

Constants.filterPresetsButtonDeoList.forEach(deo => {
    deo.onclick = function () {
        Constants.filterPresetsButtonDeoList.forEach(otherDeo => {
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

const presetsDeoAvailable = [Constants.filterPresetsDeo];
window.addEventListener('app-event.preset_selected_name', event => {
    const presetValue = event.detail.toLowerCase().replace(/\s/g, '-');
    if (!Constants.properPresets.includes(presetValue)) {
        throw new Error('Wrong preset value: ' + presetValue);
    }
    presetsDeoAvailable.forEach(presetsDeo => {
        presetsDeo.value = presetValue;
    });
    Constants.filterPresetsButtonDeoList.forEach(deo => {
        if (deo.dataset.preset === presetValue) {
            deo.classList.add('active-preset');
        } else {
            deo.classList.remove('active-preset');
        }
    });
}, false);

presetsDeoAvailable.forEach(presetsDeo => {
    presetsDeo.onchange = () => {
        if (presetsDeo.value === Constants.presetCustom) {
            visibility.showFilterOptionMainList();
        } else if (Constants.properPresets.includes(presetsDeo.value)) {
            visibility.showFilterOptionMainList();
        } else {
            presetsDeo.value = Constants.presetApertureGrille1;
        }
        storage.setFilterPresets(presetsDeo.value);
        window.dispatchEvent(new CustomEvent('app-event.custom_input_event', {
            detail: {
                value: presetsDeo.value,
                kind: 'event_kind:filter_presets_selected'
            }
        }));
    };
});
