/* Copyright (c) 2019-2024 José manuel Barroso Galindo <theypsilon@gmail.com>
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

import { Constants } from '../../services/constants';
import { Logger } from '../../services/logger';
import {PubSub, PubSubImpl} from '../../services/pubsub';

import {actions, DispatchKeyMessage, SimTemplate, SimTemplateEvents} from './sim_template';
import {data, SimViewModel, SimViewData} from './sim_view_model';
import { SimModel } from './sim_model';
import {throwOnNull} from "../../services/guards";
import {Observable, ObserverCb} from "../../services/observable";
import {BackendEvent} from "../../services/event_types";
import {Action} from "../../services/action";
import {Disposable} from "../../services/disposable";

interface Channels {
    front: PubSub<BackendMessage>;
    back: PubSub<BackendMessage>;
}

class SimPage extends HTMLElement {
    private _future: Promise<Disposable | void>;

    constructor () {
        super();

        this._future = setupPage(this.attachShadow({ mode: 'open' }), state)
            .catch(e => console.error(e));

        document.body.style.setProperty('overflow', 'hidden');
        document.body.style.setProperty('background-color', 'black');
    }

    disconnectedCallback () {
        document.body.style.removeProperty('overflow');
        document.body.style.removeProperty('background-color');

        this._future.then(mess => mess && mess.dispose());
    }
}

window.customElements.define('sim-page', SimPage);

interface BackendMessage {
    type: string;
    message: any;
}

const state = data();
const events = actions();
const channels: Channels = {
    front: PubSubImpl.make<BackendMessage>(),
    back: PubSubImpl.make<BackendMessage>()
};

async function setupPage (root: ShadowRoot, state: SimViewData): Promise<Disposable> {
    const template = SimTemplate.make(root, events);
    const view_model = SimViewModel.make(state, template);
    const backendBus = {
        subscribe: (cb: ObserverCb<BackendMessage>) => channels.back.subscribe(cb),
        fire: async (msg: BackendMessage) => await channels.front.fire(msg).catch(e => console.error(e))
    };
    const model = SimModel.make(template.getCanvas(state), backendBus);
    return show(template, view_model, model, events, channels.front as Observable<BackendMessage>, channels.back as Action<BackendMessage>)
}

async function show (template: SimTemplate, view_model: SimViewModel, model: SimModel, events: SimTemplateEvents, backendObservable: Observable<BackendMessage>, backendEmitter: Action<BackendMessage>): Promise<Disposable> {

    view_model.init(await model.load());

    async function fireBackendEvent (kind: string, message?: any) {
        const type = 'front2back:' + kind;
        const event = { message, type };
        await backendEmitter.fire(event);
        log_event(type, message);
    }

    async function fireKeyboardEvent ({ pressed, key, timeout }: {pressed: boolean, key: string, timeout?: number}) {
        await fireBackendEvent('keyboard', { pressed, key });
        if (pressed && timeout) {
            setTimeout(() => {
                Logger.log('Expired keydown for: ' + key);
                fireKeyboardEvent({ pressed: false, key });
            }, timeout);
        }
    }

    events.toggleControls.subscribe(() => view_model.toggleControls());
    events.toggleMenu.subscribe(m => view_model.toggleMenu(m));
    events.changeSyncedInput.subscribe(msg => fireBackendEvent(msg.kind, msg.value));
    events.clickPreset.subscribe(async preset => {
        view_model.clickPreset(preset);
        model.setPreset(preset);
        await fireBackendEvent(Constants.FILTER_PRESETS_SELECTED_EVENT_KIND, preset);
    });
    events.toggleCheckbox.subscribe(async (msg) => {
        if (msg.kind === 'webgl:antialias') {
            view_model.showLoading();
            await model.changeAntialiasing(msg.value);
            view_model.changeAntialias(msg.value);
        } else {
            return fireBackendEvent(msg.kind, msg.value);
        }
    });
    events.dispatchKey.subscribe(msg => {
        if (msg.key.startsWith('webgl:')) {
            return handleWebGLKeys(msg, model, view_model);
        }
        let pressed;
        let timeout;
        switch (msg.action) {
            case 'keyboth': timeout = 250; // fall through
            case 'keydown': pressed = true; break;
            case 'keyup': pressed = false; break;
        }
        return fireKeyboardEvent({ pressed, key: msg.key, timeout });
    });

    // Listening backend events
    backendObservable.subscribe(async e => {
        const msg = e.message;
        log_event(e.type, msg);
        switch (e.type) {
        case 'back2front:top_message': return view_model.openTopMessage(msg);
        case 'back2front:request_fullscreen': return view_model.setFullscreen();
        case 'back2front:request_pointer_lock': return view_model.requestPointerLock();
        case 'back2front:preset_selected_name': return view_model.presetSelectedName(msg);
        case 'back2front:screenshot': return model.fireScreenshot(msg);
        case 'back2front:camera_update': return view_model.updateCameraMatrix(msg);
        case 'back2front:toggle_info_panel': return view_model.toggleInfoPanel();
        case 'back2front:fps': return view_model.changeFps(msg);
        case 'back2front:exit_pointer_lock': return view_model.exitPointerLock();
        case 'back2front:exiting_session': return view_model.exitingSession();
        case 'back2front:change_camera_movement_mode': return view_model.changeCameraMovementMode(msg);
        case 'back2front:change_camera_zoom': return view_model.changeCameraZoom(msg);
        case 'back2front:change_pixel_width': return view_model.changePixelWidth(msg);
        case 'back2front:change_pixel_horizontal_gap': return view_model.changePixelHorizontalGap(msg);
        case 'back2front:change_pixel_vertical_gap': return view_model.changePixelVerticalGap(msg);
        case 'back2front:change_pixel_spread': return view_model.changePixelSpread(msg);
        case 'back2front:change_pixel_brightness': return view_model.changePixelBrightness(msg);
        case 'back2front:change_pixel_contrast': return view_model.changePixelContrast(msg);
        case 'back2front:change_blur_level': return view_model.changeBlurLevel(msg);
        case 'back2front:change_vertical_lpp': return view_model.changeVerticalLpp(msg);
        case 'back2front:change_horizontal_lpp': return view_model.changeHorizontalLpp(msg);
        case 'back2front:change_light_color': return view_model.changeLightColor(msg);
        case 'back2front:change_brightness_color': return view_model.changeBrightnessColor(msg);
        case 'back2front:change_movement_speed': return view_model.changeMovementSpeed(msg);
        case 'back2front:change_pixel_speed': return view_model.changePixelSpeed(msg);
        case 'back2front:change_turning_speed': return view_model.changeTurningSpeed(msg);
        case 'back2front:color_representation': return view_model.changeColorRepresentation(msg);
        case 'back2front:scaling_method': return view_model.changeScalingMethod(msg);
        case 'back2front:scaling_resolution_width': return view_model.changeCustomScalingResWidth(msg);
        case 'back2front:scaling_resolution_height': return view_model.changeCustomScalingResHeight(msg);
        case 'back2front:scaling_aspect_ratio_x': return view_model.changeCustomScalingArX(msg);
        case 'back2front:scaling_aspect_ratio_y': return view_model.changeCustomScalingArY(msg);
        case 'back2front:custom_scaling_stretch_nearest': return view_model.changeCustomScalingStretchNearest(msg);
        case 'back2front:pixel_geometry': return view_model.changePixelGeometry(msg);
        case 'back2front:pixel_shadow_shape': return view_model.changePixelShadowShape(msg);
        case 'back2front:pixel_shadow_height': return view_model.changePixelShadowHeight(msg);
        case 'back2front:backlight_percent': return view_model.changeBacklightPercent(msg);
        case 'back2front:internal_resolution': return view_model.changeInternalResolution(msg);
        case 'back2front:texture_interpolation': return view_model.changeTextureInterpolation(msg);
        case 'back2front:screen_curvature': return view_model.changeScreenCurvature(msg);
        case 'back2front:color_gamma': return view_model.changeColorGamma(msg);
        case 'back2front:color_noise': return view_model.changeColorNoise(msg);
        case 'back2front:rgb_red_r': return view_model.changeColorRgb(msg, 'red', 'r');
        case 'back2front:rgb_red_g': return view_model.changeColorRgb(msg, 'red', 'g');
        case 'back2front:rgb_red_b': return view_model.changeColorRgb(msg, 'red', 'b');
        case 'back2front:rgb_green_r': return view_model.changeColorRgb(msg, 'green', 'r');
        case 'back2front:rgb_green_g': return view_model.changeColorRgb(msg, 'green', 'g');
        case 'back2front:rgb_green_b': return view_model.changeColorRgb(msg, 'green', 'b');
        case 'back2front:rgb_blue_r': return view_model.changeColorRgb(msg, 'blue', 'r');
        case 'back2front:rgb_blue_g': return view_model.changeColorRgb(msg, 'blue', 'g');
        case 'back2front:rgb_blue_b': return view_model.changeColorRgb(msg, 'blue', 'b');
        default: throw new Error('Not covered following event: ' + e.type + ' ' + e.toString());
        }
    });

    const canvasListener = template.getCanvasListener(state);
    const windowListener = template.getWindowListener();

    // frame loop on frontend
    let newFrameId: number;
    (function requestNewFrame () {
        model.runFrame();
        view_model.newFrame();
        newFrameId = windowListener.requestAnimationFrame(requestNewFrame);
    })();

    const listeners: {eventBus: Node | Window, type: string, callback: EventListenerOrEventListenerObject, options: EventListenerOptions | boolean}[] = [];
    function addDomListener (eventBus: Node | Window, type: string, cb: BackendEvent, options?: (EventListenerOptions | boolean)) {
        options = options || false;
        const callback = cb as EventListenerOrEventListenerObject;
        eventBus.addEventListener(type, callback, options);
        listeners.push({ eventBus, type, callback, options });
    }

    // Forwarding other events so they can be readed by the backend
    addDomListener(windowListener, 'keydown', e => fireKeyboardEvent({ pressed: true, key: e.key }));
    addDomListener(windowListener, 'keyup', e => fireKeyboardEvent({ pressed: false, key: e.key }));
    addDomListener(canvasListener, 'mousedown', async e => {
        if (e.buttons === 1) {
            await fireBackendEvent('mouse-click', true);
            model.runFrame(); // Needed so Firefox can go fullscreen during the scope of this event handler, otherwise the request is rejected.
        }
    });
    addDomListener(windowListener, 'mouseup', () => fireBackendEvent('mouse-click', false)); // note this one goes to 'window'. It doesn't work with 'canvas' because of some obscure bug I didn't figure out yet.
    addDomListener(windowListener, 'mousemove', e => fireBackendEvent('mouse-move', { x: e.movementX, y: e.movementY }));
    addDomListener(canvasListener, 'mousewheel', e => fireBackendEvent('mouse-wheel', e.deltaY));
    addDomListener(canvasListener, 'blur', () => fireBackendEvent('blurred-window'));
    addDomListener(canvasListener, 'mouseover', () => fireKeyboardEvent({ pressed: true, key: 'canvas_focused' }));
    addDomListener(canvasListener, 'mouseout', () => fireKeyboardEvent({ pressed: false, key: 'canvas_focused' }));
    addDomListener(windowListener, 'resize', () => fireBackendEvent('viewport-resize', model.resizeCanvas()));

    return Disposable.make(() => {
        windowListener.cancelAnimationFrame(newFrameId);
        model.unloadSimulation();
        listeners.forEach(({ eventBus, type, callback, options }) => eventBus.removeEventListener(type, callback, options));
    });
}

async function handleWebGLKeys (msg: DispatchKeyMessage, model: SimModel, view_model: SimViewModel) {
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
            view_model.showLoading();
            const performance = await model.changePerformance(throwOnNull(msg.current), direction);
            view_model.changePerformance(performance);
        }
        break;
    }
    default: throw new Error('WebGL key not handled. ' + msg.key);
    }
}

const eventsIgnoringLogs = ['front2back:mouse-move', 'front2back:mouse-click', 'back2front:fps', 'back2front:top_message'];

function log_event(topic: string, msg: any) {
    if (eventsIgnoringLogs.includes(topic)) {
        return;
    }
    console.log(topic, msg)
}
