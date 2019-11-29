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

import Constants from '../../services/constants';
import { Observer } from '../../services/observer';

import { renderTemplate } from './sim_template';
import { data, View } from './sim_view_model';
import { Model } from './sim_model';

const state = data();

class SimPage extends HTMLElement {
    constructor () {
        super();

        this.future = setupPage(this.attachShadow({ mode: 'open' }), state, {
            front: Observer.make(),
            back: Observer.make()
        });

        document.body.style.setProperty('overflow', 'hidden');
        document.body.style.setProperty('background-color', 'black');
    }

    disconnectedCallback () {
        document.body.style.removeProperty('overflow');
        document.body.style.removeProperty('background-color');

        this.future.then(mess => mess.clean());
    }
}

window.customElements.define('sim-page', SimPage);

async function setupPage (root, state, observers) {
    const [view, canvas] = setupView(state, root, observers.front);
    const model = await setupModel(canvas, view, {
        subscribe: cb => observers.back.subscribe(cb),
        unsubscribe: cb => observers.back.unsubscribe(cb),
        fire: msg => observers.front.fire(msg)
    });
    return setupEventHandling(canvas.parentNode, view, model, {
        subscribe: cb => observers.front.subscribe(cb),
        fire: msg => observers.back.fire(msg)
    });
}

function setupView (state, root, frontendObserver) {
    const view = View.make(state, () => renderTemplate(state, fireEventOn(frontendObserver), root));

    // first frame, so there can be a canvas element rendered. We will need it in the following line.
    view.newFrame();

    return [view, root.getElementById('gl-canvas-id')];
}

async function setupModel (canvas, view, backendBus) {
    const model = Model.make(canvas, backendBus);
    view.init(await model.load());
    return model;
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

function setupEventHandling (canvasParent, view, model, frontendBus) {
    function fireBackendEvent (kind, msg) {
        const event = {
            message: msg,
            type: 'front2back:' + kind
        };
        console.log(kind, msg);
        frontendBus.fire(event);
    }

    // Listening backend events
    frontendBus.subscribe(e => {
        const msg = e.message;
        switch (e.type) {
        case 'front2front:dispatchKey': {
            if (msg.key.startsWith('webgl:')) {    
                return handleWebGLKeys(msg, model, view, frontendBus);
            }
            let pressed;
            switch (msg.action) {
            case 'keydown': pressed = true; break;
            case 'keyup': pressed = false; break;
            case 'keyboth': {
                pressed = true;
                setTimeout(() => fireBackendEvent('keyboard', { pressed: false, key: msg.key }), 250);
                break;
            }
            default: throw new Error('Incorrect action for dispatchKey', msg.action);
            }
            fireBackendEvent('keyboard', { pressed, key: msg.key });
            break;
        }
        case 'front2front:toggleCheckbox': {
            if (msg.kind === 'webgl:antialias') {
                view.showLoading();
                return model.changeAntialiasing(msg.value).then(() => view.changeAntialias(msg.value));
            } else {
                return fireBackendEvent(msg.kind, msg.value);
            }
        }
        case 'front2front:changeSyncedInput': return fireBackendEvent(msg.kind, msg.value);
        case 'front2front:toggleControls': return view.toggleControls();
        case 'front2front:toggleMenu': return view.toggleMenu(msg);
        case 'back2front:top_message': return view.openTopMessage(msg);
        case 'back2front:request_pointer_lock': return view.requestPointerLock(msg);
        case 'back2front:preset_selected_name': return view.presetSelectedName(msg);
        case 'back2front:screenshot': return model.fireScreenshot(msg);
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
        case 'back2front:scaling_method': return view.changeScalingMethod(msg);
        case 'back2front:scaling_resolution_width': return view.changeCustomScalingResWidth(msg);
        case 'back2front:scaling_resolution_height': return view.changeCustomScalingResHeight(msg);
        case 'back2front:scaling_aspect_ratio_x': return view.changeCustomScalingArX(msg);
        case 'back2front:scaling_aspect_ratio_y': return view.changeCustomScalingArY(msg);
        case 'back2front:custom_scaling_stretch_nearest': return view.changeCustomScalingStretchNearest(msg);
        case 'back2front:pixel_geometry': return view.changePixelGeometry(msg);
        case 'back2front:pixel_shadow_shape': return view.changePixelShadowShape(msg);
        case 'back2front:pixel_shadow_height': return view.changePixelShadowHeight(msg);
        case 'back2front:backlight_percent': return view.changeBacklightPercent(msg);
        case 'back2front:internal_resolution': return view.changeInternalResolution(msg);
        case 'back2front:texture_interpolation': return view.changeTextureInterpolation(msg);
        case 'back2front:screen_curvature': return view.changeScreenCurvature(msg);
        case 'front2front:clickPreset': {
            view.clickPreset(msg);
            model.setPreset(msg);
            fireBackendEvent(Constants.FILTER_PRESETS_SELECTED_EVENT_KIND, msg);
            break;
        }
        default: throw new Error('Not covered following event: ', e.type, e);
        }
    });

    // frame loop on frontend
    let newFrameId;
    (function requestNewFrame () {
        model.runFrame();
        view.newFrame();
        newFrameId = window.requestAnimationFrame(requestNewFrame);
    })();

    const listeners = [];
    function addDomListener (eventBus, type, callback, options) {
        options = options || false;
        eventBus.addEventListener(type, callback, options);
        listeners.push({ eventBus, type, callback, options });
    }

    // Forwarding other events so they can be readed by the backend
    addDomListener(window, 'keydown', e => fireBackendEvent('keyboard', { pressed: true, key: e.key }));
    addDomListener(window, 'keyup', e => fireBackendEvent('keyboard', { pressed: false, key: e.key }));
    addDomListener(canvasParent, 'mousedown', e => {
        if (e.buttons === 1) {
            fireBackendEvent('mouse-click', true);
            model.runFrame(); // Needed so Firefox can go fullscreen during the scope of this event handler, otherwise the request is rejected.
        }
    });
    addDomListener(window, 'mouseup', () => fireBackendEvent('mouse-click', false)); // note this one goes to 'window'. It doesn't work with 'canvas' because of some obscure bug I didn't figure out yet.
    addDomListener(canvasParent, 'mousemove', e => fireBackendEvent('mouse-move', { x: e.movementX, y: e.movementY }));
    addDomListener(canvasParent, 'mousewheel', e => fireBackendEvent('mouse-wheel', e.deltaY));
    addDomListener(canvasParent, 'blur', () => fireBackendEvent('blurred-window'));
    addDomListener(canvasParent, 'mouseover', () => fireBackendEvent('keyboard', { pressed: true, key: 'canvas_focused' }));
    addDomListener(canvasParent, 'mouseout', () => fireBackendEvent('keyboard', { pressed: false, key: 'canvas_focused' }));
    addDomListener(window, 'resize', () => fireBackendEvent('viewport-resize', model.resizeCanvas()));

    return {
        clean: () => {
            window.cancelAnimationFrame(newFrameId);
            model.unloadSimulation();
            listeners.forEach(({ eventBus, type, callback, options }) => eventBus.removeEventListener(type, callback, options));
        }
    };
}

function handleWebGLKeys (msg, model, view) {
    let direction;
    if (msg.key.endsWith('-dec')) {
        direction = 'dec';
    } else if (msg.key.endsWith('-inc')) {
        direction = 'inc';
    } else {
        throw new Error('Wrong key direction.');
    }
    switch (msg.key) {
    case 'webgl:performance-inc':
    case 'webgl:performance-dec': {
        if (msg.action === 'keydown') {
            view.showLoading();
            model.changePerformance(msg.current, direction).then(performance => view.changePerformance(performance));
        }
        break;
    }
    default: throw new Error('WebGL key not handled.', msg.key);
    }
}