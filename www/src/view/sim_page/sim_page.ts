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
import {
    BrowserKeyState,
    isHeldActionKey,
    isSimulationKeyboardEventHandled,
    normalizeWheelDelta,
    OneFramePulseState,
    PrimaryPointerState
} from './interaction_contract';

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

    const initDto = await model.load();
    view_model.init(initDto);
    model.setWebglUiEnabled(view_model.webglUiEnabled());
    const currentCanvas = () => model.getCanvas();
    const pulses = new OneFramePulseState();

    async function fireBackendEvent (kind: string, message?: any) {
        const type = 'front2back:' + kind;
        const event = { message, type };
        await backendEmitter.fire(event);
        log_event(type, message);
    }

    async function fireKeyboardEvent ({ pressed, key }: {pressed: boolean, key: string}) {
        await fireBackendEvent('keyboard', { pressed, key });
    }

    async function fireKeyboardPulse (key: string) {
        const alreadyPressed = pulses.isActive(key);
        const generation = pulses.begin(key);
        if (!alreadyPressed) {
            await fireKeyboardEvent({ pressed: true, key });
        }
        window.requestAnimationFrame(() => {
            if (!pulses.finish(key, generation)) {
                return;
            }
            Logger.log('Released one-frame keydown for: ' + key);
            void fireKeyboardEvent({ pressed: false, key });
        });
    }

    const subscriptions: Disposable[] = [];
    subscriptions.push(events.toggleControls.subscribe(() => view_model.toggleControls()));
    subscriptions.push(events.toggleMenu.subscribe(m => view_model.toggleMenu(m)));
    subscriptions.push(events.changeSyncedInput.subscribe(msg => fireBackendEvent(msg.kind, msg.value)));
    subscriptions.push(events.clickPreset.subscribe(async preset => {
        view_model.clickPreset(preset);
        model.setPreset(preset);
        await fireBackendEvent(Constants.FILTER_PRESETS_SELECTED_EVENT_KIND, preset);
    }));
    subscriptions.push(events.toggleCheckbox.subscribe(async (msg) => {
        if (msg.kind === 'webgl:antialias') {
            view_model.showLoading();
            await model.changeAntialiasing(msg.value);
            view_model.changeAntialias(msg.value);
        } else {
            return fireBackendEvent(msg.kind, msg.value);
        }
    }));
    subscriptions.push(events.dispatchKey.subscribe(msg => {
        if (msg.key.startsWith('webgl:')) {
            return handleWebGLKeys(msg, model, view_model);
        }
        let pressed;
        switch (msg.action) {
            case 'keyboth': return fireKeyboardPulse(msg.key);
            case 'keydown': pressed = true; break;
            case 'keyup': pressed = false; break;
        }
        return fireKeyboardEvent({ pressed, key: msg.key });
    }));

    // Listening backend events
    subscriptions.push(backendObservable.subscribe(async e => {
        const msg = e.message;
        log_event(e.type, msg);
        switch (e.type) {
        case 'back2front:request_fullscreen': return view_model.setFullscreen();
        case 'back2front:request_pointer_lock': return view_model.requestPointerLock();
        case 'back2front:preset_selected_name': {
            view_model.presetSelectedName(msg);
            return;
        }
        case 'back2front:screenshot': return model.fireScreenshot(msg);
        case 'back2front:camera_update': return view_model.updateCameraMatrix(msg);
        case 'back2front:toggle_info_panel': return view_model.toggleInfoPanel();
        case 'back2front:fps': return view_model.changeFps(msg);
        case 'back2front:exit_pointer_lock': return view_model.exitPointerLock();
        case 'back2front:exiting_session': return Logger.log('Simulation session ended.');
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
        case 'back2front:ui-copy': return writeSystemClipboard(msg);
        case 'back2front:ui-cursor': currentCanvas().style.setProperty('cursor', msg);
        case 'back2front:ui-ime': return updateImeAgent(msg);
        default: throw new Error('Not covered following event: ' + e.type + ' ' + e.toString());
        }
    }));

    const canvasListener = template.getCanvasListener(state);
    const windowListener = template.getWindowListener();
    const imeAgent = template.getImeAgent(state);
    let imeActive = false;
    let composing = false;
    let pendingKeyText: string | null = null;

    subscriptions.push(events.toggleUiMode.subscribe(() => {
        const mode = view_model.toggleUiMode();
        if (mode === 'html') {
            imeActive = false;
            composing = false;
            pendingKeyText = null;
            imeAgent.value = '';
            imeAgent.blur();
            currentCanvas().style.removeProperty('cursor');
        }
        view_model.newFrame();
        model.setWebglUiEnabled(mode === 'webgl');
    }));

    async function writeSystemClipboard (text: string) {
        try {
            await windowListener.navigator.clipboard.writeText(text);
        } catch (error) {
            // Clipboard permission can be denied outside a secure context.
            // Keep a standards-compatible fallback for local development.
            const priorValue = imeAgent.value;
            imeAgent.value = text;
            imeAgent.select();
            document.execCommand('copy');
            imeAgent.value = priorValue;
            Logger.log('Clipboard API fallback used:', error);
        }
    }

    function updateImeAgent ({active, x = 0, y = 0}: {active: boolean, x?: number, y?: number}) {
        imeActive = active;
        imeAgent.style.setProperty('left', `${x}px`);
        imeAgent.style.setProperty('top', `${y}px`);
        if (active) {
            if (document.activeElement !== imeAgent) {
                imeAgent.focus({preventScroll: true});
            }
        } else if (document.activeElement === imeAgent) {
            imeAgent.blur();
            currentCanvas().focus({preventScroll: true});
        }
    }

    // frame loop on frontend
    let newFrameId: number;
    (function requestNewFrame () {
        model.runFrame();
        view_model.newFrame();
        newFrameId = windowListener.requestAnimationFrame(requestNewFrame);
    })();

    const listeners: {eventBus: Node | Window, type: string, callback: EventListenerOrEventListenerObject, options: AddEventListenerOptions | boolean}[] = [];
    function addDomListener (eventBus: Node | Window, type: string, cb: BackendEvent, options?: (AddEventListenerOptions | boolean)) {
        options = options || false;
        const callback = cb as EventListenerOrEventListenerObject;
        eventBus.addEventListener(type, callback, options);
        listeners.push({ eventBus, type, callback, options });
    }

    // Forwarding other events so they can be readed by the backend
    const keyboardState = new BrowserKeyState();
    const simPointer = new PrimaryPointerState();
    let inputsReset = false;
    let canvasFocused = false;

    function pointerIsLocked () {
        const legacyDocument = document as Document & {mozPointerLockElement?: Element | null};
        return document.pointerLockElement !== null || legacyDocument.mozPointerLockElement != null;
    }

    function markInputsActive () {
        if (!document.hidden && document.hasFocus()) {
            inputsReset = false;
        }
    }

    function forwardPhysicalKey (pressed: boolean, e: Parameters<BackendEvent>[0]) {
        markInputsActive();
        const routeToSimulation = !isSimulationKeyboardEventHandled(e)
            && !(isHeldActionKey(e.key) && model.uiWantsKeyboard());
        const key = pressed
            ? keyboardState.press(e.code, e.key, e.location, routeToSimulation)
            : keyboardState.release(e.code, e.key, e.location);
        return key === undefined ? Promise.resolve() : fireKeyboardEvent({ pressed, key });
    }

    function releaseSimPointer (button?: number) {
        const released = button === undefined ? simPointer.cancel() : simPointer.release(button);
        if (!released) {
            return Promise.resolve();
        }
        return fireBackendEvent('mouse-click', false);
    }

    function setCanvasFocused (focused: boolean) {
        if (canvasFocused === focused) {
            return Promise.resolve();
        }
        canvasFocused = focused;
        return fireKeyboardEvent({ pressed: focused, key: 'canvas_focused' });
    }

    async function resetInteractions () {
        if (inputsReset) {
            return;
        }
        inputsReset = true;
        canvasFocused = false;
        simPointer.reset();
        pulses.clear();
        keyboardState.clear();
        model.uiEvent('pointer-gone', {});
        await template.releaseHeldActions();
        await fireBackendEvent('blurred-window');
    }

    const isMac = /Mac|iPhone|iPad|iPod/.test(windowListener.navigator.platform);
    function modifierPayload (e: Parameters<BackendEvent>[0]) {
        return {
            altKey: e.altKey,
            ctrlKey: e.ctrlKey,
            shiftKey: e.shiftKey,
            macCommand: isMac && e.metaKey,
            command: isMac ? e.metaKey : e.ctrlKey
        };
    }

    function pointerPayload (e: Parameters<BackendEvent>[0]) {
        const rect = currentCanvas().getBoundingClientRect();
        return {
            x: e.clientX - rect.left,
            y: e.clientY - rect.top,
            ...modifierPayload(e)
        };
    }

    function pointerIsInsideCanvas (e: Parameters<BackendEvent>[0]) {
        const rect = currentCanvas().getBoundingClientRect();
        return e.clientX >= rect.left && e.clientX < rect.right
            && e.clientY >= rect.top && e.clientY < rect.bottom;
    }

    function updateCanvasFocusFromPointer (e: Parameters<BackendEvent>[0]) {
        const overSimulation = (pointerIsLocked() || pointerIsInsideCanvas(e)) && !model.uiCapturesPointer();
        return setCanvasFocused(overSimulation);
    }

    function sendUiKey (pressed: boolean, e: Parameters<BackendEvent>[0]) {
        model.uiEvent('key', {
            pressed,
            repeat: e.repeat,
            code: e.code,
            key: e.key,
            ...modifierPayload(e)
        });
        if (pressed && !e.repeat && !e.isComposing && e.key.length === 1 && !e.ctrlKey && !e.metaKey && model.uiWantsKeyboard()) {
            pendingKeyText = e.key;
            model.uiEvent('text', {text: e.key});
            windowListener.setTimeout(() => {
                if (pendingKeyText === e.key) {
                    pendingKeyText = null;
                }
            }, 0);
        }
        const canvas = currentCanvas();
        const shadowActiveElement = canvas.getRootNode() instanceof ShadowRoot
            ? (canvas.getRootNode() as ShadowRoot).activeElement
            : document.activeElement;
        if (model.uiWantsKeyboard() || shadowActiveElement === canvas || shadowActiveElement === imeAgent) {
            e.preventDefault();
        }
    }

    addDomListener(windowListener, 'keydown', e => {
        sendUiKey(true, e);
        return forwardPhysicalKey(true, e);
    });
    addDomListener(windowListener, 'keyup', e => {
        sendUiKey(false, e);
        return forwardPhysicalKey(false, e);
    });
    addDomListener(canvasListener, 'pointerdown', e => {
        markInputsActive();
        currentCanvas().focus({preventScroll: true});
        const pointer = pointerPayload(e);
        model.uiEvent('pointer-moved', pointer);
        model.uiEvent('pointer-button', {...pointer, button: e.button, pressed: true});
        void updateCanvasFocusFromPointer(e);
        if (model.uiCapturesPointer()) {
            try {
                currentCanvas().setPointerCapture((e as unknown as PointerEvent).pointerId);
            } catch (_) {
                // Synthetic events and pointer-lock transitions may not own a
                // capturable browser pointer.
            }
        }
    });
    addDomListener(canvasListener, 'mousedown', async e => {
        markInputsActive();
        if (model.uiCapturesPointer()) {
            return;
        }
        if (simPointer.press(e.button, e.buttons)) {
            await fireBackendEvent('mouse-click', true);
            model.runFrame(); // Needed so Firefox can go fullscreen during the scope of this event handler, otherwise the request is rejected.
        }
    });
    addDomListener(windowListener, 'pointerup', async e => {
        const pointer = pointerPayload(e);
        model.uiEvent('pointer-moved', pointer);
        model.uiEvent('pointer-button', {...pointer, button: e.button, pressed: false});
        try {
            currentCanvas().releasePointerCapture((e as unknown as PointerEvent).pointerId);
        } catch (_) {}
        await releaseSimPointer(e.button);
        await updateCanvasFocusFromPointer(e);
    });
    addDomListener(windowListener, 'pointercancel', e => {
        model.uiEvent('pointer-gone', {});
        void setCanvasFocused(false);
        return releaseSimPointer(e.button);
    });
    addDomListener(windowListener, 'pointermove', e => {
        model.uiEvent('pointer-moved', pointerPayload(e));
        void updateCanvasFocusFromPointer(e);
        if (simPointer.isDown()) {
            void fireBackendEvent('mouse-move', { x: e.movementX, y: e.movementY });
        }
    });
    addDomListener(canvasListener, 'wheel', async e => {
        e.preventDefault();
        markInputsActive();
        model.uiEvent('pointer-moved', pointerPayload(e));
        model.uiEvent('wheel', {
            deltaX: e.deltaX,
            deltaY: e.deltaY,
            deltaMode: e.deltaMode,
            ...modifierPayload(e)
        });
        await updateCanvasFocusFromPointer(e);
        if (model.uiCapturesPointer()) {
            return;
        }
        await setCanvasFocused(true);
        await fireBackendEvent('mouse-wheel', normalizeWheelDelta(e.deltaY, e.deltaMode, windowListener.innerHeight));
    }, {passive: false});
    addDomListener(canvasListener, 'contextmenu', e => {
        if (model.uiCapturesPointer()) {
            e.preventDefault();
        }
    });
    addDomListener(imeAgent, 'beforeinput', e => {
        if (composing || !imeActive || e.inputType !== 'insertText' || !e.data) {
            return;
        }
        e.preventDefault();
        if (pendingKeyText === e.data) {
            pendingKeyText = null;
            return;
        }
        model.uiEvent('text', {text: e.data});
    });
    addDomListener(imeAgent, 'compositionstart', () => {
        composing = true;
    });
    addDomListener(imeAgent, 'compositionend', e => {
        composing = false;
        imeAgent.value = '';
        if (e.data) {
            // winit 0.26 exposes committed text but no portable pre-edit
            // stream. Commit-only delivery on both adapters avoids a false
            // platform distinction while retaining dead-key/IME input.
            model.uiEvent('text', {text: e.data});
        }
    });
    addDomListener(imeAgent, 'paste', e => {
        e.preventDefault();
        model.uiEvent('paste', {text: e.clipboardData?.getData('text/plain') || ''});
    });
    addDomListener(windowListener, 'blur', async () => {
        model.uiEvent('focus', {focused: false});
        await resetInteractions();
    });
    addDomListener(windowListener, 'focus', () => {
        model.uiEvent('focus', {focused: true});
        markInputsActive();
    });
    addDomListener(document, 'visibilitychange', () => document.hidden ? resetInteractions() : markInputsActive());
    addDomListener(windowListener, 'pagehide', () => resetInteractions());
    const pointerLockChanged = () => pointerIsLocked() ? Promise.resolve() : releaseSimPointer();
    addDomListener(document, 'pointerlockchange', pointerLockChanged);
    addDomListener(document, 'mozpointerlockchange', pointerLockChanged);
    addDomListener(canvasListener, 'mouseenter', e => {
        markInputsActive();
        model.uiEvent('pointer-moved', pointerPayload(e));
        return updateCanvasFocusFromPointer(e);
    });
    addDomListener(canvasListener, 'mouseleave', async () => {
        await setCanvasFocused(false);
        if (!pointerIsLocked()) {
            model.uiEvent('pointer-gone', {});
            await releaseSimPointer();
        }
    });
    let viewportSize = {width: currentCanvas().width, height: currentCanvas().height};
    addDomListener(windowListener, 'resize', () => {
        const nextViewportSize = model.resizeCanvas();
        if (nextViewportSize.width === viewportSize.width && nextViewportSize.height === viewportSize.height) {
            return;
        }
        viewportSize = nextViewportSize;
        return fireBackendEvent('viewport-resize', nextViewportSize);
    });

    // `runFrame` dispatches the initial controller values through async
    // browser observers. Let those observers settle, render their values, and
    // only then reveal an input-ready page. Subsequent frames have already
    // paid the one-time shader/resource initialization cost.
    await Promise.resolve();
    view_model.newFrame();
    if (!initDto.glError) {
        view_model.hideLoading();
    }

    return Disposable.make(() => {
        pulses.clear();
        keyboardState.clear();
        void template.releaseHeldActions();
        windowListener.cancelAnimationFrame(newFrameId);
        model.unloadSimulation();
        listeners.forEach(({ eventBus, type, callback, options }) => eventBus.removeEventListener(type, callback, options));
        subscriptions.forEach(subscription => subscription.dispose());
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

const eventsIgnoringLogs = ['front2back:mouse-move', 'front2back:mouse-click', 'back2front:fps'];

function log_event(topic: string, msg: any) {
    if (eventsIgnoringLogs.includes(topic)) {
        return;
    }
    console.log(topic, msg)
}
