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
import { Messenger } from '../../services/messenger';
import { Observer } from '../../services/observer';
import { Launcher } from './sim_launcher';
import Constants from '../../services/constants';

import { renderTemplate } from './sim_template';
import { model, View } from './sim_view_model';

const store = LocalStorage.make('sim_page/presets_selection');
const state = model(store);

class SimPage extends HTMLElement {
    constructor () {
        super();

        this.mess = setupPage(this.attachShadow({ mode: 'open' }), state, store, Launcher.make(), Messenger.getInstance(), {
            front: Observer.make(),
            back: Observer.make()
        });

        document.body.style.setProperty('overflow', 'hidden');
        document.body.style.setProperty('background-color', 'black');
    }

    disconnectedCallback () {
        document.body.style.removeProperty('overflow');
        document.body.style.removeProperty('background-color');

        this.mess.clean();
    }
}

window.customElements.define('sim-page', SimPage);

function setupPage (root, state, store, launcher, messenger, observers) {
    const [view, canvas] = setupView(state, root, observers.front);
    setupSimulation(canvas, messenger, launcher, view, observers);
    return setupEventHandling(canvas, observers, view, store);
}

function setupView (state, root, frontendObserver) {
    const view = View.make(state, () => renderTemplate(state, fireEventOn(frontendObserver), root));

    // first frame, so there can be a canvas element rendered. We will need it in the following line.
    view.newFrame();

    return [view, root.getElementById('gl-canvas-id')];
}

function setupSimulation (canvas, messenger, launcher, view, observers) {
    messenger.consumeInbox('sim-page').forEach(async msg => {
        switch (msg.topic) {
        case 'launch': {
            const result = await launcher.launch(canvas, observers, msg.launcherParams);
            if (result.glError) {
                view.showFatalError('WebGL2 is not working on your browser, try restarting it! And remember, this works only on a PC with updated browser and graphics drivers.');
                return;
            }
            view.showScreen();
            if (msg.skipControllerUi) {
                view.setUiNotVisible();
            }
            if (msg.fullscreen) {
                view.setFullscreen();
            }
            break;
        }
        default: throw new Error('Wrong topic: ' + msg.topic);
        }
    });
}

function fireEventOn (observer) {
    return (topic, message) => {
        const event = {
            message,
            type: 'front2front:' + topic
        };
        observer.fire(event);
    };
}

function setupEventHandling (canvas, observers, view, store) {
    function fireBackendEvent (kind, msg) {
        //console.log('observers.back.fire', kind, msg);
        const event = {
            message: msg,
            type: 'front2back:' + kind
        };
        observers.back.fire(event);
    }

    // Listening backend events
    observers.front.subscribe(e => {
        const msg = e.message;
        //console.log('observers.front.subscribe', e.type, msg);
        switch (e.type) {
        case 'front2front:toggleControls': return view.toggleControls();
        case 'front2front:toggleMenu': return view.toggleMenu(msg);
        case 'front2front:clickPreset': {
            view.clickPreset(msg);
            if (msg !== Constants.PRESET_KIND_CUSTOM) {
                store.setItem(Constants.FILTERS_PRESET_STORE_KEY, msg);
            }
            fireBackendEvent(Constants.FILTER_PRESETS_SELECTED_EVENT_KIND, msg);
            break;
        }
        case 'front2front:dispatchKey': return fireBackendEvent('keyboard', { pressed: msg.action === 'keydown', key: msg.key });
        case 'front2front:changeSyncedInput': return fireBackendEvent(msg.value, msg.kind);
        case 'back2front:new_frame': return view.newFrame(msg);
        case 'back2front:top_message': return view.openTopMessage(msg);
        case 'back2front:request_pointer_lock': return view.requestPointerLock(msg);
        case 'back2front:preset_selected_name': return view.presetSelectedName(msg);
        case 'back2front:screenshot': return view.fireScreenshot(msg);
        case 'back2front:camera_update': return view.updateCameraMatrix(msg);
        case 'back2front:toggle_info_panel': return view.toggleInfoPanel(msg);
        case 'back2front:fps': return view.changeFps(msg);
        case 'back2front:exit_pointer_lock': return view.exitPointerLock(msg);
        case 'back2front:exiting_session': return view.exitingSession(msg);
        case 'back2front:change_camera_movement_mode': return view.changeCameraMovementMode(msg);
        case 'back2front:change_camera_zoom': return view.changeCameraZoom(msg);
        case 'back2front:change_pixel_width': return view.changePixelWidth(msg);
        case 'back2front:change_pixel_horizontal_gap': return view.changePixelHorizontalGap(msg);
        case 'back2front:change_pixel_vertical_gap': return view.changePixelVerticalGap(msg);
        case 'back2front:change_pixel_spread': return view.changePixelSpread(msg);
        case 'back2front:change_pixel_brightness': return view.changePixelBrightness(msg);
        case 'back2front:change_pixel_contrast': return view.changePixelContrast(msg);
        case 'back2front:change_blur_level': return view.changeBlurLevel(msg);
        case 'back2front:change_vertical_lpp': return view.changeVerticalLpp(msg);
        case 'back2front:change_horizontal_lpp': return view.changeHorizontalLpp(msg);
        case 'back2front:change_light_color': return view.changeLightColor(msg);
        case 'back2front:change_brightness_color': return view.changeBrightnessColor(msg);
        case 'back2front:change_movement_speed': return view.changeMovementSpeed(msg);
        case 'back2front:change_pixel_speed': return view.changePixelSpeed(msg);
        case 'back2front:change_turning_speed': return view.changeTurningSpeed(msg);
        case 'back2front:color_representation': return view.changeColorRepresentation(msg);
        case 'back2front:pixel_geometry': return view.changePixelGeometry(msg);
        case 'back2front:pixel_shadow_shape': return view.changePixelShadowShape(msg);
        case 'back2front:pixel_shadow_height': return view.changePixelShadowHeight(msg);
        case 'back2front:backlight_percent': return view.changeBacklightPercent(msg);
        case 'back2front:internal_resolution': return view.changeInternalResolution(msg);
        case 'back2front:texture_interpolation': return view.changeTextureInterpolation(msg);
        case 'back2front:screen_curvature': return view.changeScreenCurvature(msg);
        default: throw new Error('Not covered following event: ', e.type, e);
        }
    });

    const listeners = [];
    function addDomListener (eventBus, type, callback, options) {
        options = options || false;
        eventBus.addEventListener(type, callback, options);
        listeners.push({ eventBus, type, callback, options });
    }

    // Forwarding keyboard events so it can be readed by the backend
    addDomListener(canvas, 'keydown', e => fireBackendEvent('keyboard', { pressed: true, key: e.key }));
    addDomListener(canvas, 'keyup', e => fireBackendEvent('keyboard', { pressed: false, key: e.key }));
    addDomListener(canvas, 'mousedown', e => e.buttons === 1 && fireBackendEvent('mouse_click', true));
    addDomListener(window, 'mouseup', () => fireBackendEvent('mouse_click', false)); // note this one goes to 'window'. It doesn't work with 'canvas' because of some obscure bug I didn't figure out yet.
    addDomListener(canvas, 'mousemove', e => fireBackendEvent('mouse_move', { x: e.movementX, y: e.movementY }));
    addDomListener(canvas, 'mousewheel', e => fireBackendEvent('mouse_wheel', e.deltaY));
    addDomListener(canvas, 'blur', () => fireBackendEvent('blurred_window'));
    addDomListener(canvas, 'mouseover', () => fireBackendEvent('keyboard', { pressed: true, key: 'canvas_focused' }));
    addDomListener(canvas, 'mouseout', () => fireBackendEvent('keyboard', { pressed: false, key: 'canvas_focused' }));

    return {
        clean: () => listeners.forEach(({ eventBus, type, callback, options }) => eventBus.removeEventListener(type, callback, options))
    };
}