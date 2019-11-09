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

import { LocalStorage } from '../../services/local_storage';
import { Navigator } from '../../services/navigator';
import { Messenger } from '../../services/messenger';
import { SimLauncher } from './sim_launcher';

const navigator = Navigator.make();
const simLauncher = SimLauncher.make();
const messenger = Messenger.make();

export default function (ctx) {
    initializePresetsSelection(ctx);
    initializeScreenshotListener(ctx);
    initializeSyncListeners(ctx);
    initializeCollapseMenu(ctx);
    initializeFpsIndicator(ctx);
    initializeExitActions(ctx);
    readInbox(ctx);
}

const FILTERS_PRESET_STORE_KEY = 'FiltersPreset';
function initializePresetsSelection (ctx) {
    const store = LocalStorage.make('sim_page/presets_selection');

    const preset = store.getItem(FILTERS_PRESET_STORE_KEY) || ctx.constants.PRESET_KIND_APERTURE_GRILLE_1;
    ctx.elements.filterPresetsButtonDeoList
        .filter(deo => deo.dataset.preset === preset)[0]
        .classList.add(ctx.constants.PRESET_ACTIVE_CLASS);

    ctx.elements.filterPresetsButtonDeoList.forEach(deo => {
        deo.onclick = function () {
            const preset = deo.dataset.preset;
            ctx.elements.filterPresetsButtonDeoList.forEach(otherDeo => {
                otherDeo.classList.remove(ctx.constants.PRESET_ACTIVE_CLASS);
            });
            deo.classList.add(ctx.constants.PRESET_ACTIVE_CLASS);
            if (preset !== ctx.constants.PRESET_KIND_CUSTOM) {
                store.setItem(FILTERS_PRESET_STORE_KEY, preset);
            }
            ctx.eventBus.dispatchEvent(new CustomEvent(ctx.constants.APP_EVENT_CUSTOM_INPUT, {
                detail: {
                    value: preset,
                    kind: 'event_kind:filter_presets_selected'
                }
            }));
        };
    });

    ctx.eventBus.addEventListener(ctx.constants.APP_EVENT_PRESET_SELECTED_NAME, event => {
        ctx.elements.filterPresetsButtonDeoList
            .forEach(deo => {
                if (deo.dataset.preset === event.detail) {
                    deo.classList.add(ctx.constants.PRESET_ACTIVE_CLASS);
                } else if (deo.classList.contains(ctx.constants.PRESET_ACTIVE_CLASS)) {
                    deo.classList.remove(ctx.constants.PRESET_ACTIVE_CLASS);
                }
            });
        if (event.detail === ctx.constants.PRESET_KIND_CUSTOM) {
            navigator.openTopMessage('Now you are in the Custom mode, you may change any filter value you want.');
        }
    });
}

function initializeScreenshotListener (ctx) {
    ctx.eventBus.addEventListener(ctx.constants.APP_EVENT_SCREENSHOT, async event => {
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
}

function initializeSyncListeners (ctx) {
    ctx.eventBus.addEventListener(ctx.constants.APP_EVENT_CAMERA_UPDATE, event => {
        ctx.elements.cameraPosXDeo.value = Math.round(event.detail[0] * 100) / 100;
        ctx.elements.cameraPosYDeo.value = Math.round(event.detail[1] * 100) / 100;
        ctx.elements.cameraPosZDeo.value = Math.round(event.detail[2] * 100) / 100;
        ctx.elements.cameraDirXDeo.value = Math.round(event.detail[3] * 100) / 100;
        ctx.elements.cameraDirYDeo.value = Math.round(event.detail[4] * 100) / 100;
        ctx.elements.cameraDirZDeo.value = Math.round(event.detail[5] * 100) / 100;
        ctx.elements.cameraAxisUpXDeo.value = Math.round(event.detail[6] * 100) / 100;
        ctx.elements.cameraAxisUpYDeo.value = Math.round(event.detail[7] * 100) / 100;
        ctx.elements.cameraAxisUpZDeo.value = Math.round(event.detail[8] * 100) / 100;
    }, false);

    [
        { deo: ctx.elements.cameraZoomDeo, eventId: ctx.constants.APP_EVENT_CHANGE_CAMERA_ZOOM },
        { deo: ctx.elements.cameraMovementModeDeo, eventId: ctx.constants.APP_EVENT_CHANGE_CAMERA_MOVEMENT_MODE },
        { deo: ctx.elements.pixelWidthDeo, eventId: ctx.constants.APP_EVENT_CHANGE_PIXEL_WIDTH },
        { deo: ctx.elements.pixelHorizontalGapDeo, eventId: ctx.constants.APP_EVENT_CHANGE_PIXEL_HORIZONTAL_GAP },
        { deo: ctx.elements.pixelVerticalGapDeo, eventId: ctx.constants.APP_EVENT_CHANGE_PIXEL_VERTICAL_GAP },
        { deo: ctx.elements.pixelSpreadDeo, eventId: ctx.constants.APP_EVENT_CHANGE_PIXEL_SPREAD },
        { deo: ctx.elements.pixelBrigthnessDeo, eventId: ctx.constants.APP_EVENT_CHANGE_PIXEL_BRIGHTNESS },
        { deo: ctx.elements.pixelContrastDeo, eventId: ctx.constants.APP_EVENT_CHANGE_PIXEL_CONTRAST },
        { deo: ctx.elements.blurLevelDeo, eventId: ctx.constants.APP_EVENT_CHANGE_BLUR_LEVEL },
        { deo: ctx.elements.verticalLppDeo, eventId: ctx.constants.APP_EVENT_CHANGE_VERTICAL_LPP },
        { deo: ctx.elements.horizontalLppDeo, eventId: ctx.constants.APP_EVENT_CHANGE_HORIZONTAL_LPP },
        { deo: ctx.elements.lightColorDeo, eventId: ctx.constants.APP_EVENT_CHANGE_LIGHT_COLOR },
        { deo: ctx.elements.brightnessColorDeo, eventId: ctx.constants.APP_EVENT_CHANGE_BRIGHTNESS_COLOR },
        { deo: ctx.elements.featureChangeMoveSpeedDeo, eventId: ctx.constants.APP_EVENT_CHANGE_MOVEMENT_SPEED },
        { deo: ctx.elements.featureChangePixelSpeedDeo, eventId: ctx.constants.APP_EVENT_CHANGE_PIXEL_SPEED },
        { deo: ctx.elements.featureChangeTurnSpeedDeo, eventId: ctx.constants.APP_EVENT_CHANGE_TURNING_SPEED },

        { deo: ctx.elements.featureChangeColorRepresentationDeo, eventId: ctx.constants.APP_EVENT_COLOR_REPRESENTATION },
        { deo: ctx.elements.featureChangePixelGeometryDeo, eventId: ctx.constants.APP_EVENT_PIXEL_GEOMETRY },
        { deo: ctx.elements.featureChangePixelShadowShapeDeo, eventId: ctx.constants.APP_EVENT_PIXEL_SHADOW_SHAPE },
        { deo: ctx.elements.featureChangePixelShadowHeightDeo, eventId: ctx.constants.APP_EVENT_PIXEL_SHADOW_HEIGHT },
        { deo: ctx.elements.featureBacklightPercentDeo, eventId: ctx.constants.APP_EVENT_BACKLIGHT_PERCENT },
        { deo: ctx.elements.featureInternalResolutionDeo, eventId: ctx.constants.APP_EVENT_INTERNAL_RESOLUTION },
        { deo: ctx.elements.featureTextureInterpolationDeo, eventId: ctx.constants.APP_EVENT_TEXTURE_INTERPOLATION },
        { deo: ctx.elements.featureChangeScreenCurvatureDeo, eventId: ctx.constants.APP_EVENT_SCREEN_CURVATURE }
    ].forEach(({ deo, eventId }) => {
        if (!deo) throw new Error('Wrong deo on defining: ' + eventId);
        ctx.eventBus.addEventListener(eventId, event => {
            deo.value = event.detail;
            if (eventId === ctx.constants.APP_EVENT_CHANGE_CAMERA_MOVEMENT_MODE) {
                switch (event.detail) {
                case 'Lock on Display':
                    deo.title = 'The camera will move around the picture, always looking at it';
                    ctx.visibility.showFreeModeCameraControls();
                    break;
                case 'Free Flight':
                    deo.title = 'The camera can move without any restriction in the whole 3D space with plane-like controls';
                    ctx.visibility.hideFreeModeCameraControls();
                    break;
                default:
                    throw new Error('Unreachable!');
                }
            }
        }, false);
    });

    customEventOnButtonPressed(ctx.elements.featureCameraMovementsDeo);
    customEventOnButtonPressed(ctx.elements.featureCameraTurnsDeo);
    function customEventOnButtonPressed (deo) {
        deo.querySelectorAll('.activate-button').forEach(button => {
            const eventOptions = { key: button.value.toLowerCase() };
            button.onmousedown = () => ctx.eventBus.dispatchEvent(new KeyboardEvent('keydown', eventOptions));
            button.onmouseup = () => ctx.eventBus.dispatchEvent(new KeyboardEvent('keyup', eventOptions));
        });
    }

    customEventOnChange(ctx.elements.cameraPosXDeo, ctx.constants.EVENT_KIND_CAMERA_POS_X, a => +a);
    customEventOnChange(ctx.elements.cameraPosYDeo, ctx.constants.EVENT_KIND_CAMERA_POS_Y, a => +a);
    customEventOnChange(ctx.elements.cameraPosZDeo, ctx.constants.EVENT_KIND_CAMERA_POS_Z, a => +a);
    customEventOnChange(ctx.elements.cameraAxisUpXDeo, ctx.constants.EVENT_KIND_CAMERA_AXIS_UP_X, a => +a);
    customEventOnChange(ctx.elements.cameraAxisUpYDeo, ctx.constants.EVENT_KIND_CAMERA_AXIS_UP_Y, a => +a);
    customEventOnChange(ctx.elements.cameraAxisUpZDeo, ctx.constants.EVENT_KIND_CAMERA_AXIS_UP_Z, a => +a);
    customEventOnChange(ctx.elements.cameraDirXDeo, ctx.constants.EVENT_KIND_CAMERA_DIRECTION_X, a => +a);
    customEventOnChange(ctx.elements.cameraDirYDeo, ctx.constants.EVENT_KIND_CAMERA_DIRECTION_Y, a => +a);
    customEventOnChange(ctx.elements.cameraDirZDeo, ctx.constants.EVENT_KIND_CAMERA_DIRECTION_Z, a => +a);
    customEventOnChange(ctx.elements.cameraZoomDeo, ctx.constants.EVENT_KIND_CAMERA_ZOOM, a => +a);
    customEventOnChange(ctx.elements.cameraMovementModeDeo, ctx.constants.EVENT_KIND_CAMERA_MOVEMENT_MODE, a => +a);

    customEventOnChange(ctx.elements.pixelWidthDeo, ctx.constants.EVENT_KIND_PIXEL_WIDTH, a => +a);
    customEventOnChange(ctx.elements.pixelSpreadDeo, ctx.constants.EVENT_KIND_PIXEL_SPREAD, a => +a);
    customEventOnChange(ctx.elements.pixelHorizontalGapDeo, ctx.constants.EVENT_KIND_PIXEL_HORIZONTAL_GAP, a => +a);
    customEventOnChange(ctx.elements.pixelVerticalGapDeo, ctx.constants.EVENT_KIND_PIXEL_VERTICAL_GAP, a => +a);
    customEventOnChange(ctx.elements.blurLevelDeo, ctx.constants.EVENT_KIND_BLUR_LEVEL, a => +a);
    customEventOnChange(ctx.elements.verticalLppDeo, ctx.constants.EVENT_KIND_VERTICAL_LPP, a => +a);
    customEventOnChange(ctx.elements.horizontalLppDeo, ctx.constants.EVENT_KIND_HORIZONTAL_LPP, a => +a);
    customEventOnChange(ctx.elements.pixelBrigthnessDeo, ctx.constants.EVENT_KIND_PIXEL_BRIGHTNESS, a => +a);
    customEventOnChange(ctx.elements.pixelContrastDeo, ctx.constants.EVENT_KIND_PIXEL_CONTRAST, a => +a);
    customEventOnChange(ctx.elements.featureChangePixelShadowHeightDeo, ctx.constants.EVENT_KIND_PIXEL_SHADOW_HEIGHT, a => +a);
    customEventOnChange(ctx.elements.featureBacklightPercentDeo, ctx.constants.EVENT_KIND_BACKLIGHT_PERCENT, a => +a);

    const parseColor = (value) => parseInt('0x' + value.substring(1));
    customEventOnChange(ctx.elements.lightColorDeo, ctx.constants.EVENT_KIND_LIGHT_COLOR, parseColor);
    customEventOnChange(ctx.elements.brightnessColorDeo, ctx.constants.EVENT_KIND_BRIGHTNESS_COLOR, parseColor);
    function customEventOnChange (deo, kind, parse) {
        const changed = () => {
            ctx.eventBus.dispatchEvent(new CustomEvent(ctx.constants.APP_EVENT_CUSTOM_INPUT, {
                detail: {
                    value: parse(deo.value),
                    kind: ctx.constants.EVENT_KIND_PREFIX + kind
                }
            }));
        };
        deo.addEventListener('change', changed);
    }

    window.addEventListener('keydown', e => ctx.eventBus.dispatchEvent(new KeyboardEvent('keydown', { key: e.key })), false);
    window.addEventListener('keyup', e => ctx.eventBus.dispatchEvent(new KeyboardEvent('keyup', { key: e.key })), false);
}

function initializeCollapseMenu (ctx) {
    ctx.root.querySelectorAll('.collapse-button').forEach(deo => {
        const open = deo.dataset.openText;
        const close = deo.dataset.closeText;
        const target = ctx.root.getElementById(deo.dataset.collapseTarget);
        deo.onclick = () => {
            if (target.classList.contains('display-none')) {
                target.classList.remove('display-none');
                deo.classList.remove('collapsed');
                deo.classList.add('not-collapsed');
                if (close) {
                    deo.innerText = close;
                }
            } else {
                target.classList.add('display-none');
                deo.classList.add('collapsed');
                deo.classList.remove('not-collapsed');
                if (open) {
                    deo.innerText = open;
                }
            }
        };
        deo.click();
        deo.click();
    });

    ctx.root.querySelectorAll('.number-input').forEach(deo => {
        let button;
        [{ button_text: '↓', mode: 'dec', placement: 'after' }, { button_text: '↑', mode: 'inc', placement: 'before' }].forEach(o => {
            button = document.createElement('button');
            button.innerText = o.button_text;
            button.classList.add('button-inc-dec');
            const eventOptions = { key: deo.id + '-' + o.mode };
            button.onmousedown = () => ctx.eventBus.dispatchEvent(new KeyboardEvent('keydown', eventOptions));
            button.onmouseup = () => ctx.eventBus.dispatchEvent(new KeyboardEvent('keyup', eventOptions));
            deo.parentNode.insertBefore(button, o.placement === 'before' ? deo : deo.nextSibling);
        });
        if (deo.classList.contains('feature-readonly-input')) {
            deo.onmousedown = e => { e.preventDefault(); ctx.eventBus.dispatchEvent(new KeyboardEvent('keydown', { key: deo.id + '-inc' })); };
            deo.onmouseup = e => { e.preventDefault(); ctx.eventBus.dispatchEvent(new KeyboardEvent('keyup', { key: deo.id + '-inc' })); };
            deo.onmouseenter = () => button.classList.add('hover');
            deo.onmouseleave = () => button.classList.remove('hover');
            button.onmouseenter = () => deo.classList.add('hover');
            button.onmouseleave = () => deo.classList.remove('hover');
        }
    });
    
    ctx.root.querySelectorAll('input[type="number"], input[type="text"]').forEach(deo => {
        const eventOptions = { key: 'input_focused' };
        deo.addEventListener('focus', () => ctx.eventBus.dispatchEvent(new KeyboardEvent('keydown', eventOptions)));
        deo.addEventListener('blur', () => ctx.eventBus.dispatchEvent(new KeyboardEvent('keyup', eventOptions)));
        deo.onkeypress = e => e.charCode === 13 /* ENTER */ && deo.blur();
    });

    ctx.root.querySelectorAll('.hk-inc').forEach(deo => deo.setAttribute('title', 'Press \'' + deo.innerText + '\' to increse the value of this field'));
    ctx.root.querySelectorAll('.hk-dec').forEach(deo => deo.setAttribute('title', 'Press \'' + deo.innerText + '\' to decrease the value of this field'));
    
    ctx.root.querySelectorAll('.menu-button').forEach(deo => {
        deo.onclick = () => ctx.eventBus.dispatchEvent(new KeyboardEvent('keydown', { key: deo.id }));
    });

    ctx.eventBus.addEventListener(ctx.constants.APP_EVENT_TOGGLE_INFO_PANEL, () => {
        ctx.elements.infoPanelToggleDeo.click();
    }, false);
}

function initializeFpsIndicator (ctx) {
    ctx.eventBus.addEventListener(ctx.constants.APP_EVENT_FPS, event => {
        ctx.elements.fpsCounterDeo.innerHTML = Math.round(event.detail);
    }, false);        
}

function initializeExitActions (ctx) {
    ctx.eventBus.addEventListener(ctx.constants.APP_EVENT_EXIT_POINTER_LOCK, () => {
        document.exitPointerLock();
    }, false);

    ctx.eventBus.addEventListener(ctx.constants.APP_EVENT_EXITING_SESSION, () => {
        ctx.visibility.showLoading();
        navigator.goToLandingPage();
    }, false);
}

function readInbox (ctx) {
    messenger.consumeInbox('sim-page').forEach(async msg => {
        switch (msg.topic) {
        case 'launch': {
            ctx.elements.lightColorDeo.value = '#FFFFFF';
            ctx.elements.brightnessColorDeo.value = '#FFFFFF';

            const filteredPresets = ctx.elements.filterPresetsButtonDeoList.filter(deo => deo.classList.contains('active-preset'));
            msg.launcherParams.activePreset = filteredPresets.length > 0 ? filteredPresets[0].dataset.preset : ctx.constants.PRESET_KIND_APERTURE_GRILLE_1;

            const result = await simLauncher.launch(ctx, msg.launcherParams);

            if (result.glError) {
                ctx.visibility.showLoading();

                navigator.openTopMessage('WebGL2 is not working on your browser, try restarting it! And remember, this works only on a PC with updated browser and graphics drivers.');
                navigator.goToLandingPage();
                return;
            }
            
            ctx.visibility.hideLoading();
            ctx.visibility.showSimulationUi();

            if (msg.hasBackgroundUi) {
                ctx.visibility.showSimulationUi();
            }
            
            if (msg.hasControllerUi) {
                ctx.visibility.showSimulationUi();
                ctx.visibility.showInfoPanel();
            }
            
            if (msg.fullscreen) {
                document.body.requestFullscreen();
            }

            break;
        }
        default: throw new Error('Wrong topic: ' + msg.topic);
        }
    });
}