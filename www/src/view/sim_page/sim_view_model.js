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

import { Navigator } from '../../services/navigator';
import { Visibility } from '../../services/visibility';
import { Launcher } from './sim_launcher';

const PRESET_KIND_APERTURE_GRILLE_1 = 'crt-aperture-grille-1';
const PRESET_KIND_CUSTOM = 'custom';
const FILTERS_PRESET_STORE_KEY = 'FiltersPreset';
const FILTER_PRESETS_SELECTED_EVENT_KIND = 'filter-presets-selected';

export function model (store) {
    const options = {
        presets: {
            selected: store.getItem(FILTERS_PRESET_STORE_KEY) || PRESET_KIND_APERTURE_GRILLE_1,
            eventKind: FILTER_PRESETS_SELECTED_EVENT_KIND,
            choices: [
                { preset: PRESET_KIND_APERTURE_GRILLE_1, text: 'CRT Aperture Grille 1' },
                { preset: 'crt-shadow-mask-1', text: 'CRT Shadow Mask 1' },
                { preset: 'crt-shadow-mask-2', text: 'CRT Shadow Mask 2' },
                { preset: 'sharp-1', text: 'CRT Sharp Pixels' },
                { preset: 'demo-1', text: 'Flight Demo' },
                { preset: PRESET_KIND_CUSTOM, text: 'Custom' }
            ]
        },
        internal_resolution: { value: null, eventKind: 'internal-resolution' },
        screen_curvature: { value: null, eventKind: 'screen-curvature' },
        blur_level: { value: null, eventKind: 'blur-level' },
        horizontal_gap: { value: null, eventKind: 'pixel-horizontal-gap' },
        vertical_gap: { value: null, eventKind: 'pixel-vertical-gap' },
        pixel_width: { value: null, eventKind: 'pixel-width' },
        vertical_lpp: { value: null, eventKind: 'vertical-lpp' },
        horizontal_lpp: { value: null, eventKind: 'horizontal-lpp' },
        light_color: { value: '#FFFFFF', eventKind: 'light-color' },
        pixel_brightness: { value: null, eventKind: 'pixel-brightness' },
        pixel_contrast: { value: null, eventKind: 'pixel-contrast' },
        color_representation: { value: null, eventKind: 'color-representation' },
        pixel_geometry: { value: null, eventKind: 'pixel-geometry' },
        pixel_shadow_shape: { value: null, eventKind: 'pixel-shadow-shape' },
        pixel_shadow_height: { value: null, eventKind: 'pixel-shadow-height' },
        texture_interpolation: { value: null, eventKind: 'texture-interpolation' },
        backlight_percent: { value: null, eventKind: 'backlight-percent' },
        pixel_spread: { value: null, eventKind: 'pixel-spread' },
        brightness_color: { value: '#FFFFFF', eventKind: 'brightness-color' },
        camera_movement_mode: { value: '', title: '', eventKind: 'camera-movement-mode' },
        camera_matrix: {
            free: false,
            pos: { x: { eventKind: 'camera-pos-x', value: 0 }, y: { eventKind: 'camera-pos-y', value: 0 }, z: { eventKind: 'camera-pos-z', value: 0 } },
            dir: { x: { eventKind: 'camera-dir-x', value: 0 }, y: { eventKind: 'camera-dir-y', value: 0 }, z: { eventKind: 'camera-dir-z', value: 0 } },
            axis_up: { x: { eventKind: 'camera-axis-up-x', value: 0 }, y: { eventKind: 'camera-axis-up-y', value: 0 }, z: { eventKind: 'camera-axis-up-z', value: 0 } }
        },
        camera_zoom: { value: null, eventKind: 'camera_zoom' },
        move_speed: { value: null, eventKind: 'move-speed' },
        pixel_speed: { value: null, eventKind: 'pixel-speed' },
        turn_speed: { value: null, eventKind: 'turn-speed' },
        reset_filters: { eventKind: 'reset-filters' },
        reset_camera: { eventKind: 'reset-camera' },
        reset_speeds: { eventKind: 'reset-speeds' },
        capture_framebuffer: { eventKind: 'capture-framebuffer' },
        quit_simulation: { eventKind: 'quit-simulation' }
    };
    
    return {
        fps: 60,
        options,
        menu: {
            open: true,
            visible: true,
            controlsText: 'Close Controls',
            entries: [
                {
                    type: 'menu',
                    text: 'Presets',
                    open: true,
                    entries: [
                        { type: 'preset-buttons', class: 'menu-2 menu-blc-grey', ref: options.presets }
                    ]
                },
                {
                    type: 'menu',
                    text: 'Basic Filter Settings',
                    open: true,
                    entries: [
                        { type: 'selectors-input', class: 'menu-2 menu-blc-white', text: 'Internal Resolution', hk: { inc: 'Y', dec: 'Shift + Y' }, ref: options.internal_resolution },
                        { type: 'selectors-input', class: 'menu-2 menu-blc-red', text: 'Screen curvature type', hk: { inc: 'B', dec: 'Shift + B' }, ref: options.screen_curvature },
                        { type: 'number-input', class: 'menu-2 menu-blc-blue', text: 'Blur level', hk: { inc: 'J', dec: 'Shift + J' }, step: 1, min: 0, max: 100, value: 0, placeholder: 0, ref: options.blur_level }
                    ]
                },
                {
                    type: 'menu',
                    text: 'Advanced Filter Settings',
                    open: false,
                    entries: [
                        { type: 'number-input', class: 'menu-2 menu-blc-red', text: 'Horizontal gap', hk: { inc: 'U', dec: 'Shift + U' }, step: 0.001, min: 0, max: 10, value: 0, placeholder: 0, ref: options.horizontal_gap },
                        { type: 'number-input', class: 'menu-2 menu-blc-red', text: 'Vertical gap', hk: { inc: 'I', dec: 'Shift + I' }, step: 0.001, min: 0, max: 10, value: 0, placeholder: 0, ref: options.vertical_gap },
                        { type: 'number-input', class: 'menu-2 menu-blc-yellow', text: 'Pixel width', hk: { inc: 'O', dec: 'Shift + O' }, step: 0.001, min: 0, max: 10, value: 0, placeholder: 0, ref: options.pixel_width },
                        { type: 'number-input', class: 'menu-2 menu-blc-lila', text: 'Vertical lines per pixel', hk: { inc: 'K', dec: 'Shift + K' }, step: 1, min: 0, max: 100, value: 0, placeholder: 0, ref: options.vertical_lpp },
                        { type: 'number-input', class: 'menu-2 menu-blc-lila', text: 'Horizontal lines per pixel', hk: { inc: 'L', dec: 'Shift + L' }, step: 1, min: 0, max: 100, value: 0, placeholder: 0, ref: options.horizontal_lpp },
                        { type: 'color-input', class: 'menu-2 menu-blc-blue', text: 'Source light color', value: '#ffffff', ref: options.light_color },
                        { type: 'number-input', class: 'menu-2 menu-blc-white', text: 'Brightness', hk: { inc: 'X', dec: 'Shift + X' }, step: 0.001, min: -1, max: 1, value: 0, placeholder: 0, ref: options.pixel_brightness },
                        { type: 'number-input', class: 'menu-2 menu-blc-white', text: 'Contrast', hk: { inc: 'Z', dec: 'Shift + Z' }, step: 0.001, min: 0, max: 20, value: 1, placeholder: 0, ref: options.pixel_contrast },
                        { type: 'selectors-input', class: 'menu-2 menu-blc-red', text: 'Color channels type', hk: { inc: 'C', dec: 'Shift + C' }, ref: options.color_representation },
                        { type: 'selectors-input', class: 'menu-2 menu-blc-yellow', text: 'Pixel geometry type', hk: { inc: 'V', dec: 'Shift + V' }, ref: options.pixel_geometry },
                        { type: 'selectors-input', class: 'menu-2 menu-blc-blue', text: 'Pixel texture', hk: { inc: 'N', dec: 'Shift + N' }, ref: options.pixel_shadow_shape },
                        { type: 'number-input', class: 'menu-2 menu-blc-lila', text: 'Pixel variable height', hk: { inc: 'M', dec: 'Shift + M' }, step: 0.001, min: 0, max: 1, value: 0, placeholder: 0, ref: options.texture_interpolation },
                        { type: 'selectors-input', class: 'menu-2 menu-blc-yellow', text: 'Texture interpolation', hk: { inc: 'H', dec: 'Shift + H' }, ref: options.texture_interpolation },
                        { type: 'number-input', class: 'menu-2 menu-blc-green', text: 'Backlight', hk: { inc: 'dot', dec: 'Shift + dot' }, step: 0.001, min: 0, max: 1, value: 0.5, placeholder: 0.5, ref: options.backlight_percent },
                        { type: 'number-input', class: 'display-none', text: 'Pixel spread', hk: { inc: 'P', dec: 'Shift + P' }, step: 0.001, min: 0, max: 10, value: 0, placeholder: 0, ref: options.pixel_spread },
                        { type: 'color-input', class: 'display-none', text: 'Brightness color', value: '#ffffff', ref: options.brightness_color },
                        { type: 'button-input', class: 'menu-2 menu-blc-grey', text: 'Reset Filter Values', ref: options.reset_filters }
                    ]
                },
                {
                    type: 'menu',
                    text: 'Camera',
                    open: false,
                    entries: [
                        { type: 'selectors-input', class: 'menu-2 menu-blc-lila', text: 'Movement Type', hk: { inc: 'G', dec: 'Shift + G' }, ref: options.camera_movement_mode },
                        { type: 'camera-input', class: 'menu-blc-red', ref: options.camera_matrix },
                        { type: 'number-input', class: 'menu-2 menu-blc-blue', text: 'Zoom', hk: { inc: 'Mouse Wheel Up', dec: 'Mouse Wheel Down' }, step: 1, min: 1, max: 45, value: 0, placeholder: 0, ref: options.camera_zoom },
                        { type: 'button-input', class: 'menu-2 menu-blc-grey', text: 'Reset Position', ref: options.reset_camera }
                    ]
                },
                {
                    type: 'menu',
                    text: 'Command Modifiers',
                    open: false,
                    entries: [
                        { type: 'selectors-input', class: 'menu-2 menu-blc-red', text: 'Camera speed', hk: { inc: 'F', dec: 'R' }, ref: options.move_speed },
                        { type: 'selectors-input', class: 'menu-2 menu-blc-blue', text: 'Filter speed', hk: { inc: 'Shift + F', dec: 'Shift + R' }, ref: options.pixel_speed },
                        { type: 'selectors-input', class: 'display-none', text: 'Turn speed', hk: { inc: 'Alt + F', dec: 'Alt + R' }, ref: options.turn_speed },
                        { type: 'button-input', class: 'menu-2 menu-blc-grey', text: 'Reset Modifiers', ref: options.reset_speeds }
                    ]
                },
                {
                    type: 'menu',
                    text: 'Extra',
                    open: false,
                    entries: [
                        { type: 'button-input', class: 'menu-2 menu-blc-yellow', text: 'Take Screenshot', ref: options.capture_framebuffer }
                    ]
                },
                { type: 'button-input', class: 'menu-1 menu-blc-grey favicon', text: 'Go to Landing Page', ref: options.quit_simulation }
            ]
        }
    };        
}

export class View {
    constructor (state, page, store, navigator, launcher, visibility) {
        this._state = state;
        this._page = page;
        this._store = store;
        this._navigator = navigator;
        this._launcher = launcher;
        this._visibility = visibility;
        this._isDirty = true;
    }

    static make (state, page, store, navigator, launcher, visibility) {
        return new View(state, page, store, navigator || Navigator.make(), launcher || Launcher.make(), visibility || Visibility.make());
    }

    async launchSimulation (msg) {
        const result = await this._launcher.launch(this.canvas, msg.launcherParams);

        if (result.glError) {
            this._visibility.showLoading();

            this._navigator.openTopMessage('WebGL2 is not working on your browser, try restarting it! And remember, this works only on a PC with updated browser and graphics drivers.');
            this._navigator.goToLandingPage();
            return;
        }
        
        this._visibility.hideLoading();
        
        if (msg.skipControllerUi) {
            this._state.menu.visible = false;
        }
        
        if (msg.fullscreen) {
            document.body.requestFullscreen();
        }
    }

    newFrame () {
        if (!this._isDirty) return;
        this._isDirty = false;
        this._page.refresh();
    }

    toggleControls () {
        this._state.menu.open = !this._state.menu.open;
        this._state.menu.controlsText = this._state.menu.open ? 'Close Controls' : 'Open Controls';
        this._isDirty = true;
    }

    toggleMenu (menu) {
        menu.open = !menu.open;
        this._isDirty = true;
    }

    dispatchKey (eventName, key) {
        const event = new KeyboardEvent(eventName, { key });
        this.canvas.dispatchEvent(event);
    }
    changeSyncedInput (value, kind) {
        const event = new CustomEvent('display-sim-event:frontend-channel', {
            detail: {
                message: value,
                type: 'front2back:' + kind
            }
        });
        this.canvas.dispatchEvent(event);
    }

    clickPreset (preset) {
        this._state.options.presets.selected = preset;
        if (preset !== PRESET_KIND_CUSTOM) {
            this._store.setItem(FILTERS_PRESET_STORE_KEY, preset);
        }
        this.changeSyncedInput(preset, FILTER_PRESETS_SELECTED_EVENT_KIND);
        this._isDirty = true;
    }

    openTopMessage (msg) {
        this._navigator.openTopMessage(msg);
    }

    requestPointerLock () {
        this.canvas.requestPointerLock();
        if (window.screen.width !== window.innerWidth && window.screen.height !== window.innerHeight) {
            document.documentElement.requestFullscreen();
        }
    }

    presetSelectedName (msg) {
        if (msg === PRESET_KIND_CUSTOM) {
            this._navigator.openTopMessage('Now you are in the Custom mode, you may change any filter value you want.');
        }
        this._state.options.presets.selected = msg;
        this._isDirty = true;
    }
    fireScreenshot (msg) {
        (async () => {
            const arrayBuffer = msg[0];
            const multiplier = msg[1];
        
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
        })();
    }
    updateCameraMatrix (msg) {
        this._state.options.camera_matrix.pos.x.value = Math.round(msg[0] * 100) / 100;
        this._state.options.camera_matrix.pos.y.value = Math.round(msg[1] * 100) / 100;
        this._state.options.camera_matrix.pos.z.value = Math.round(msg[2] * 100) / 100;
        this._state.options.camera_matrix.dir.x.value = Math.round(msg[3] * 100) / 100;
        this._state.options.camera_matrix.dir.y.value = Math.round(msg[4] * 100) / 100;
        this._state.options.camera_matrix.dir.z.value = Math.round(msg[5] * 100) / 100;
        this._state.options.camera_matrix.axis_up.x.value = Math.round(msg[6] * 100) / 100;
        this._state.options.camera_matrix.axis_up.y.value = Math.round(msg[7] * 100) / 100;
        this._state.options.camera_matrix.axis_up.z.value = Math.round(msg[8] * 100) / 100;
        this._isDirty = true;
    }
    toggleInfoPanel () {
        this._state.menu.open = !this._state.menu.open;
        this._isDirty = true;
    }
    changeFps (msg) {
        this._state.fps = Math.round(msg);
        this._isDirty = true;
    }
    exitPointerLock () {
        document.exitPointerLock();
    }
    exitingSession () {
        window.location.hash = '';
        this._navigator.goToLandingPage();
    }
    changeCameraMovementMode (msg) {
        switch (msg) {
        case 'Lock on Display':
            this._state.options.camera_movement_mode.title = 'The camera will move around the picture, always looking at it';
            this._state.options.camera_matrix.free = false;
            break;
        case 'Free Flight':
            this._state.options.camera_movement_mode.title = 'The camera can move without any restriction in the whole 3D space with plane-like controls';
            this._state.options.camera_matrix.free = true;
            break;
        default: throw new Error('Unreachable!');
        }
        this._state.options.camera_movement_mode.value = msg;
        this._isDirty = true;
    }
    changeCameraZoom (msg) {
        this._state.options.camera_zoom.value = msg;
        this._isDirty = true;
    }
    changePixelWidth (msg) {
        this._state.options.pixel_width.value = msg;
        this._isDirty = true;
    }
    changePixelHorizontalGap (msg) {
        this._state.options.horizontal_gap.value = msg;
        this._isDirty = true;
    }
    changePixelVerticalGap (msg) {
        this._state.options.vertical_gap.value = msg;
        this._isDirty = true;
    }
    changePixelSpread (msg) {
        this._state.options.pixel_spread.value = msg;
        this._isDirty = true;
    }
    changePixelBrightness (msg) {
        this._state.options.pixel_brightness.value = msg;
        this._isDirty = true;
    }
    changePixelContrast (msg) {
        this._state.options.pixel_contrast.value = msg;
        this._isDirty = true;
    }
    changeBlurLevel (msg) {
        this._state.options.blur_level.value = msg;
        this._isDirty = true;
    }
    changeVerticalLpp (msg) {
        this._state.options.vertical_lpp.value = msg;
        this._isDirty = true;
    }
    changeHorizontalLpp (msg) {
        this._state.options.horizontal_lpp.value = msg;
        this._isDirty = true;
    }
    changeLightColor (msg) {
        this._state.options.light_color.value = msg;
        this._isDirty = true;
    }
    changeBrightnessColor (msg) {
        this._state.options.brightness_color.value = msg;
        this._isDirty = true;
    }
    changeMovementSpeed (msg) {
        this._state.options.move_speed.value = msg;
        this._isDirty = true;
    }
    changePixelSpeed (msg) {
        this._state.options.pixel_speed.value = msg;
        this._isDirty = true;
    }
    changeTurningSpeed (msg) {
        this._state.options.turn_speed.value = msg;
        this._isDirty = true;
    }
    changeColorRepresentation (msg) {
        this._state.options.color_representation.value = msg;
        this._isDirty = true;
    }
    changePixelGeometry (msg) {
        this._state.options.pixel_geometry.value = msg;
        this._isDirty = true;
    }
    changePixelShadowShape (msg) {
        this._state.options.pixel_shadow_shape.value = msg;
        this._isDirty = true;
    }
    changePixelShadowHeight (msg) {
        this._state.options.pixel_shadow_height.value = msg;
        this._isDirty = true;
    }
    changeBacklightPercent (msg) {
        this._state.options.backlight_percent.value = msg;
        this._isDirty = true;
    }
    changeInternalResolution (msg) {
        this._state.options.internal_resolution.value = msg;
        this._isDirty = true;
    }
    changeTextureInterpolation (msg) {
        this._state.options.texture_interpolation.value = msg;
        this._isDirty = true;
    }
    changeScreenCurvature (msg) {
        this._state.options.screen_curvature.value = msg;
        this._isDirty = true;
    }
}
