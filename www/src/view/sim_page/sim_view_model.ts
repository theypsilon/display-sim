/* Copyright (c) 2019-2022 José manuel Barroso Galindo <theypsilon@gmail.com>
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
import { Navigator } from '../../services/navigator';
import { Visibility } from '../../services/visibility';
import { SimTemplate } from "./sim_template";

declare global {
    interface Document {
        mozRequestPointerLock: () => void;
        mozExitPointerLock: () => void;
    }

    interface HTMLElement {
        webkitRequestFullScreen: () => void;
        mozRequestFullScreen: () => void;
        msRequestFullscreen: () => void;
        mozRequestPointerLock: () => void;
    }
}

export type SimViewEntry =
    MenuEntry |
    PresetButtonsEntry |
    ScalingInputEntry |
    CheckboxInputEntry |
    ButtonInputEntry |
    SelectorsInput |
    NumericPairEntry |
    NumberInputEntry |
    ColorInputEntry |
    CameraInputEntry |
    RgbInputEntry;

interface CssEntry { class: string; text: string; }
interface RefEntry<T> { ref: Ref<T>; }
interface PlaceholderEntry { placeholder: number; step: number; min: number; max: number; }

export interface MenuEntry { type: 'menu'; open: boolean; text: string; entries: SimViewEntry[]; }
export interface PresetButtonsEntry { type: 'preset-buttons'; class: string; ref: PresetRef; }
export interface ScalingInputEntry extends RefEntry<string> { type: 'scaling-input'; entries: SimViewEntry[]; }
export interface CheckboxInputEntry extends CssEntry, RefEntry<number> { type: 'checkbox-input'; }
export interface ButtonInputEntry extends CssEntry, RefEntry<string> { type: 'button-input'; }
export interface SelectorsInput extends CssEntry, RefEntry<string> { type: 'selectors-input'; hk?: HotKey, }
export interface NumericPairEntry extends CssEntry { type: 'numeric-pair'; separator: string; pair: [HalfPair, HalfPair]; }
export interface NumberInputEntry extends CssEntry, RefEntry<number>, PlaceholderEntry { type: 'number-input'; hk: HotKey }
export interface ColorInputEntry extends CssEntry, RefEntry<string> { type: 'color-input'; }

export interface HotKey {
    inc: string;
    dec: string;
}

export interface HalfPair extends PlaceholderEntry, RefEntry<number> {}

export interface Ref<T> {
    value: T;
    eventKind: string;
    title?: string;
}

export interface PresetChoice {
    text: string;
    preset: string;
}

export interface PresetRef {
    selected: string;
    choices: PresetChoice[];
}

export interface CameraInputEntry { type: 'camera-input'; class: string; ref: CameraRef }
export interface CameraRef {
    lockMode: boolean;
    pos: Camera3DVector;
    dir: Camera3DVector;
    axis_up: Camera3DVector;
}
export interface Camera3DVector {
    x: Ref<number>;
    y: Ref<number>;
    z: Ref<number>;
}

export interface RgbInputEntry { type: 'rgb-input'; class: string; ref: RgbRef }
export interface RgbRef {
    red: RgbComponent;
    green: RgbComponent;
    blue: RgbComponent;
}

export interface RgbComponent {
    r: Ref<number>;
    g: Ref<number>;
    b: Ref<number>;
}

export function data () {
    const options = {
        presets: {
            selected: null as string | null,
            eventKind: Constants.FILTER_PRESETS_SELECTED_EVENT_KIND,
            choices: [
                { preset: Constants.PRESET_KIND_APERTURE_GRILLE_1, text: 'CRT Aperture Grille 1' },
                { preset: 'crt-shadow-mask-1', text: 'CRT Shadow Mask 1' },
                { preset: 'crt-shadow-mask-2', text: 'CRT Shadow Mask 2' },
                { preset: 'sharp-1', text: 'CRT Sharp Pixels' },
                { preset: 'demo-1', text: 'Flight Demo' },
                { preset: Constants.PRESET_KIND_CUSTOM, text: 'Custom' }
            ]
        },
        internal_resolution: { value: null as number | null, eventKind: 'internal-resolution' },
        screen_curvature: { value: null as number | null, eventKind: 'screen-curvature' },
        blur_level: { value: null as number | null, eventKind: 'blur-level' },
        horizontal_gap: { value: null as number | null, eventKind: 'pixel-horizontal-gap' },
        vertical_gap: { value: null as number | null, eventKind: 'pixel-vertical-gap' },
        pixel_width: { value: null as number | null, eventKind: 'pixel-width' },
        vertical_lpp: { value: null as number | null, eventKind: 'vertical-lpp' },
        horizontal_lpp: { value: null as number | null, eventKind: 'horizontal-lpp' },
        color_gamma: { value: 1.0, eventKind: 'color-gamma' },
        color_noise: { value: 0.0, eventKind: 'color-noise' },
        light_color: { value: '#FFFFFF', eventKind: 'light-color' },
        pixel_brightness: { value: null as number | null, eventKind: 'pixel-brightness' },
        pixel_contrast: { value: null as number | null, eventKind: 'pixel-contrast' },
        color_representation: { value: null as number | null, eventKind: 'color-representation' },
        pixel_geometry: { value: null as number | null, eventKind: 'pixel-geometry' },
        pixel_shadow_shape: { value: null as number | null, eventKind: 'pixel-shadow-shape' },
        pixel_shadow_height: { value: null as number | null, eventKind: 'pixel-shadow-height' },
        texture_interpolation: { value: null as number | null, eventKind: 'texture-interpolation' },
        backlight_percent: { value: null as number | null, eventKind: 'backlight-percent' },
        pixel_spread: { value: null as number | null, eventKind: 'pixel-spread' },
        brightness_color: { value: '#FFFFFF', eventKind: 'brightness-color' },
        camera_movement_mode: { value: '', title: '', eventKind: 'camera-movement-mode' },
        camera_matrix: {
            lockMode: false,
            pos: { x: { eventKind: 'camera-pos-x', value: 0 }, y: { eventKind: 'camera-pos-y', value: 0 }, z: { eventKind: 'camera-pos-z', value: 0 } },
            dir: { x: { eventKind: 'camera-dir-x', value: 0 }, y: { eventKind: 'camera-dir-y', value: 0 }, z: { eventKind: 'camera-dir-z', value: 0 } },
            axis_up: { x: { eventKind: 'camera-axis-up-x', value: 0 }, y: { eventKind: 'camera-axis-up-y', value: 0 }, z: { eventKind: 'camera-axis-up-z', value: 0 } }
        },
        rgb_values: {
            red: { r: { eventKind: 'rgb-red-r', value: 1 }, g: { eventKind: 'rgb-red-g', value: 0 }, b: { eventKind: 'rgb-red-b', value: 0 } },
            green: { r: { eventKind: 'rgb-green-r', value: 0 }, g: { eventKind: 'rgb-green-g', value: 1 }, b: { eventKind: 'rgb-green-b', value: 0 } },
            blue: { r: { eventKind: 'rgb-blue-r', value: 0 }, g: { eventKind: 'rgb-blue-g', value: 0 }, b: { eventKind: 'rgb-blue-b', value: 1 } }
        },
        camera_zoom: { value: null as number | null, eventKind: 'camera_zoom' },
        move_speed: { value: null as number | null, eventKind: 'move-speed' },
        pixel_speed: { value: null as number | null, eventKind: 'pixel-speed' },
        turn_speed: { value: null as number | null, eventKind: 'turn-speed' },
        reset_filters: { eventKind: 'reset-filters' },
        reset_camera: { eventKind: 'reset-camera' },
        reset_speeds: { eventKind: 'reset-speeds' },
        capture_framebuffer: { eventKind: 'capture-framebuffer' },
        webgl_performance: { value: null as string | null, eventKind: 'webgl:performance' },
        webgl_antialias: { value: null as boolean | null, eventKind: 'webgl:antialias' },
        scaling_method: { value: null as string | null, eventKind: 'scaling-method' },
        custom_resolution_width: { value: null as number | null, eventKind: 'custom-scaling-resolution-width' },
        custom_resolution_height: { value: null as number | null, eventKind: 'custom-scaling-resolution-height' },
        custom_aspect_ratio_x: { value: null as number | null, eventKind: 'custom-scaling-aspect-ratio-x' },
        custom_aspect_ratio_y: { value: null as number | null, eventKind: 'custom-scaling-aspect-ratio-y' },
        custom_scaling_stretch_nearest: { value: null as number | null, eventKind: 'custom-scaling-stretch-nearest' },
        quit_simulation: { eventKind: 'quit-simulation' }
    };
    
    return {
        initStoredValues: false,
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
                        { type: 'preset-buttons', class: 'menu-2 menu-blc-grey', ref: options.presets } as PresetButtonsEntry
                    ]
                } as MenuEntry,
                {
                    type: 'menu',
                    text: 'Image Scaling',
                    open: false,
                    entries: [
                        { type: 'selectors-input', class: 'menu-2 menu-blc-blue', text: 'Scaling Method', ref: options.scaling_method } as SelectorsInput,
                        { type: 'scaling-input',
                            ref: options.scaling_method,
                            entries: [
                                { type: 'numeric-pair',
                                    text: 'Image resolution',
                                    separator: '✕',
                                    class: 'menu-blc-lila',
                                    pair: [
                                        { min: 1, max: 10000, step: 1, placeholder: 256, ref: options.custom_resolution_width } as HalfPair,
                                        { min: 1, max: 10000, step: 1, placeholder: 240, ref: options.custom_resolution_height } as HalfPair
                                    ] } as NumericPairEntry,
                                { type: 'numeric-pair',
                                    text: 'Aspect Ratio',
                                    separator: ':',
                                    class: 'menu-blc-lila',
                                    pair: [
                                        { min: 1, max: 1920 * 4, step: 1, placeholder: 4, ref: options.custom_aspect_ratio_x },
                                        { min: 1, max: 1080 * 4, step: 1, placeholder: 3, ref: options.custom_aspect_ratio_y }
                                    ] } as NumericPairEntry,
                                { type: 'checkbox-input', class: 'menu-2 menu-blc-lila', text: 'Stretch to nearest border', ref: options.custom_scaling_stretch_nearest } as CheckboxInputEntry,
                                { type: 'number-input', class: 'menu-2 menu-blc-yellow', text: 'Pixel width', hk: { inc: 'O', dec: 'Shift + O' }, step: 0.001, min: 0, max: 10, value: 0, placeholder: 0, ref: options.pixel_width } as NumberInputEntry
                            ] } as ScalingInputEntry
                    ]
                } as MenuEntry,
                {
                    type: 'menu',
                    text: 'Performance',
                    open: true,
                    entries: [
                        { type: 'selectors-input', class: 'menu-2 menu-blc-white', text: 'Internal Resolution', hk: { inc: 'Y', dec: 'Shift + Y' }, ref: options.internal_resolution },
                        { type: 'number-input', class: 'menu-2 menu-blc-blue', text: 'Blur passes', hk: { inc: 'J', dec: 'Shift + J' }, step: 1, min: 0, max: 100, value: 0, placeholder: 0, ref: options.blur_level } as NumberInputEntry
                    ]
                } as MenuEntry,
                {
                    type: 'menu',
                    text: 'Colors',
                    open: false,
                    entries: [
                        { type: 'rgb-input', class: 'menu-blc-red', ref: options.rgb_values },
                        { type: 'number-input', class: 'menu-2 menu-blc-lila', text: 'Gamma correction', hk: { inc: '????', dec: 'Shift + ????' }, step: 0.1, min: 0, max: 1, value: 0, placeholder: 0, ref: options.color_gamma },
                        { type: 'number-input', class: 'menu-2 menu-blc-yellow', text: 'Color noise', hk: { inc: '????', dec: 'Shift + ????' }, step: 0.1, min: 0, max: 1, value: 0, placeholder: 0, ref: options.color_noise },
                        { type: 'color-input', class: 'menu-2 menu-blc-blue', text: 'Source light color', value: '#ffffff', ref: options.light_color },
                        { type: 'number-input', class: 'menu-2 menu-blc-white', text: 'Brightness', hk: { inc: 'X', dec: 'Shift + X' }, step: 0.001, min: -1, max: 1, value: 0, placeholder: 0, ref: options.pixel_brightness },
                        { type: 'number-input', class: 'menu-2 menu-blc-white', text: 'Contrast', hk: { inc: 'Z', dec: 'Shift + Z' }, step: 0.001, min: 0, max: 20, value: 1, placeholder: 0, ref: options.pixel_contrast },
                        { type: 'color-input', class: 'display-none', text: 'Brightness color', value: '#ffffff', ref: options.brightness_color }
                    ]
                } as MenuEntry,
                {
                    type: 'menu',
                    text: 'Geometry & Textures',
                    open: false,
                    entries: [
                        { type: 'selectors-input', class: 'menu-2 menu-blc-white', text: 'Screen curvature type', hk: { inc: 'B', dec: 'Shift + B' }, ref: options.screen_curvature },
                        { type: 'number-input', class: 'menu-2 menu-blc-red', text: 'Horizontal gap', hk: { inc: 'U', dec: 'Shift + U' }, step: 0.001, min: 0, max: 10, value: 0, placeholder: 0, ref: options.horizontal_gap },
                        { type: 'number-input', class: 'menu-2 menu-blc-red', text: 'Vertical gap', hk: { inc: 'I', dec: 'Shift + I' }, step: 0.001, min: 0, max: 10, value: 0, placeholder: 0, ref: options.vertical_gap },
                        { type: 'number-input', class: 'menu-2 menu-blc-lila', text: 'V. lines per pixel', hk: { inc: 'K', dec: 'Shift + K' }, step: 1, min: 0, max: 100, value: 0, placeholder: 0, ref: options.vertical_lpp },
                        { type: 'number-input', class: 'menu-2 menu-blc-lila', text: 'H. lines per pixel', hk: { inc: 'L', dec: 'Shift + L' }, step: 1, min: 0, max: 100, value: 0, placeholder: 0, ref: options.horizontal_lpp },
                        { type: 'selectors-input', class: 'menu-2 menu-blc-red', text: 'Color channels type', hk: { inc: 'C', dec: 'Shift + C' }, ref: options.color_representation },
                        { type: 'selectors-input', class: 'menu-2 menu-blc-yellow', text: 'Pixel geometry type', hk: { inc: 'V', dec: 'Shift + V' }, ref: options.pixel_geometry },
                        { type: 'selectors-input', class: 'menu-2 menu-blc-blue', text: 'Pixel texture', hk: { inc: 'N', dec: 'Shift + N' }, ref: options.pixel_shadow_shape },
                        { type: 'number-input', class: 'menu-2 menu-blc-lila', text: 'Pixel variable height', hk: { inc: 'M', dec: 'Shift + M' }, step: 0.001, min: 0, max: 1, value: 0, placeholder: 0, ref: options.pixel_shadow_height },
                        { type: 'selectors-input', class: 'menu-2 menu-blc-yellow', text: 'Texture interpolation', hk: { inc: 'H', dec: 'Shift + H' }, ref: options.texture_interpolation },
                        { type: 'number-input', class: 'menu-2 menu-blc-green', text: 'Backlight', hk: { inc: 'dot', dec: 'Shift + dot' }, step: 0.001, min: 0, max: 1, value: 0.5, placeholder: 0.5, ref: options.backlight_percent },
                        { type: 'number-input', class: 'display-none', text: 'Pixel spread', hk: { inc: 'P', dec: 'Shift + P' }, step: 0.001, min: 0, max: 10, value: 0, placeholder: 0, ref: options.pixel_spread },
                        { type: 'button-input', class: 'menu-2 menu-blc-grey', text: 'Reset Filter Values', ref: options.reset_filters }
                    ]
                } as MenuEntry,
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
                } as MenuEntry,
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
                } as MenuEntry,
                {
                    type: 'menu',
                    text: 'WebGL Settings',
                    open: false,
                    entries: [
                        { type: 'selectors-input', class: 'menu-2 menu-blc-red', text: 'Performance', ref: options.webgl_performance },
                        { type: 'checkbox-input', class: 'menu-2 menu-blc-red', text: 'Antialias', ref: options.webgl_antialias }
                    ]
                } as MenuEntry,
                {
                    type: 'menu',
                    text: 'Extra',
                    open: false,
                    entries: [
                        { type: 'button-input', class: 'menu-2 menu-blc-yellow', text: 'Take Screenshot', ref: options.capture_framebuffer }
                    ]
                } as MenuEntry,
                { type: 'button-input', class: 'menu-1 menu-blc-grey favicon', text: 'Go to Landing Page', ref: options.quit_simulation } as ButtonInputEntry
            ]
        }
    };        
}

export type SimViewData = ReturnType<typeof data>;

export interface SimViewInitDto {
    glError: boolean;
    skipControllerUi: boolean;
    fullscreen: boolean;
    storedValues: {selectedPreset: string, powerPreference: string, antialias: boolean};
}

export class SimViewModel {
    private readonly _state: SimViewData;
    private readonly _template: SimTemplate;
    private readonly _navigator: Navigator;
    private readonly _visibility: Visibility;
    private _isDirty: boolean;

    constructor (state: SimViewData, template: SimTemplate, navigator: Navigator, visibility: Visibility) {
        this._state = state;
        this._template = template;
        this._navigator = navigator;
        this._visibility = visibility;
        this._isDirty = true;
    }

    static make (state: SimViewData, template: SimTemplate, navigator?: Navigator, visibility?: Visibility) {
        return new SimViewModel(state, template, navigator || Navigator.make(), visibility || Visibility.make());
    }

    init (dto: SimViewInitDto) {
        if (dto.glError) {
            return this.showFatalError('WebGL2 is not working on your browser, try restarting it! And remember, this works only on a PC with updated browser and graphics drivers.');
        }
        this._visibility.hideLoading();
        if (dto.skipControllerUi) {
            this.setUiNotVisible();
        }
        if (dto.fullscreen) {
            this.setFullscreen();
        }
        if (!this._state.initStoredValues) {
            this._state.initStoredValues = true;
            this._state.options.presets.selected = dto.storedValues.selectedPreset;
            this._state.options.webgl_performance.value = dto.storedValues.powerPreference;
            this._state.options.webgl_antialias.value = dto.storedValues.antialias;
        }
        this._isDirty = true;
    }

    showLoading () {
        this._visibility.showLoading();
    }

    showFatalError (msg: string) {
        this._visibility.showLoading();
        this._navigator.openTopMessage(msg);
        this._navigator.goToLandingPage();
    }

    setUiNotVisible () {
        this._state.menu.visible = false;
        this._isDirty = true;
    }

    newFrame () {
        if (!this._isDirty) return;
        this._isDirty = false;
        this._template.refresh(this._state);
    }

    toggleControls () {
        this._state.menu.open = !this._state.menu.open;
        this._state.menu.controlsText = this._state.menu.open ? 'Close Controls' : 'Open Controls';
        this._isDirty = true;
    }

    toggleMenu (menu: MenuEntry) {
        menu.open = !menu.open;
        this._isDirty = true;
    }
    clickPreset (preset: string) {
        this._state.options.presets.selected = preset;
        this._isDirty = true;
    }

    openTopMessage (msg: string) {
        this._navigator.openTopMessage(msg);
    }
    setFullscreen () {
        if (window.screen.width !== window.innerWidth && window.screen.height !== window.innerHeight) {
            const element = document.documentElement;
            (element.requestFullscreen || element.webkitRequestFullScreen || element.mozRequestFullScreen || element.msRequestFullscreen).bind(element)();
        }
    }
    requestPointerLock () {
        const element = document.documentElement;
        (element.requestPointerLock || element.mozRequestPointerLock).bind(element)();
    }
    exitPointerLock () {
        (document.exitPointerLock || document.mozExitPointerLock).bind(document)();
        document.exitPointerLock();
    }

    presetSelectedName (msg: string) {
        if (msg === Constants.PRESET_KIND_CUSTOM) {
            this._navigator.openTopMessage('Now you are in the Custom mode, you may change any filter value you want.');
        }
        this._state.options.presets.selected = msg;
        this._isDirty = true;
    }
    updateCameraMatrix (msg: [number, number, number, number, number, number, number, number, number]) {
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
    changeFps (msg: number) {
        this._state.fps = Math.round(msg);
        this._isDirty = true;
    }
    exitingSession () {
        Logger.log('User closed the simulation.');
        window.location.hash = '';
        this._navigator.goToLandingPage();
    }
    changeCameraMovementMode (msg: string) {
        switch (msg) {
        case '2D':
            this._state.options.camera_movement_mode.title = 'The camera can move up down left right, facing the picture';
            this._state.options.camera_matrix.lockMode = false;
            break;
        case '3D':
            this._state.options.camera_movement_mode.title = 'The camera can move in all 3 axis and also can turn and rotate.';
            this._state.options.camera_matrix.lockMode = true;
            break;
        default: throw new Error('Unreachable!');
        }
        this._state.options.camera_movement_mode.value = msg;
        this._isDirty = true;
    }
    changeCameraZoom (msg: number) {
        this._state.options.camera_zoom.value = msg;
        this._isDirty = true;
    }
    changePixelWidth (msg: number) {
        this._state.options.pixel_width.value = msg;
        this._isDirty = true;
    }
    changePixelHorizontalGap (msg: number) {
        this._state.options.horizontal_gap.value = msg;
        this._isDirty = true;
    }
    changePixelVerticalGap (msg: number) {
        this._state.options.vertical_gap.value = msg;
        this._isDirty = true;
    }
    changePixelSpread (msg: number) {
        this._state.options.pixel_spread.value = msg;
        this._isDirty = true;
    }
    changePixelBrightness (msg: number) {
        this._state.options.pixel_brightness.value = msg;
        this._isDirty = true;
    }
    changePixelContrast (msg: number) {
        this._state.options.pixel_contrast.value = msg;
        this._isDirty = true;
    }
    changeBlurLevel (msg: number) {
        this._state.options.blur_level.value = msg;
        this._isDirty = true;
    }
    changeVerticalLpp (msg: number) {
        this._state.options.vertical_lpp.value = msg;
        this._isDirty = true;
    }
    changeHorizontalLpp (msg: number) {
        this._state.options.horizontal_lpp.value = msg;
        this._isDirty = true;
    }
    changeLightColor (msg: string) {
        this._state.options.light_color.value = msg;
        this._isDirty = true;
    }
    changeBrightnessColor (msg: string) {
        this._state.options.brightness_color.value = msg;
        this._isDirty = true;
    }
    changeMovementSpeed (msg: number) {
        this._state.options.move_speed.value = msg;
        this._isDirty = true;
    }
    changePixelSpeed (msg: number) {
        this._state.options.pixel_speed.value = msg;
        this._isDirty = true;
    }
    changeTurningSpeed (msg: number) {
        this._state.options.turn_speed.value = msg;
        this._isDirty = true;
    }
    changeColorRepresentation (msg: number) {
        this._state.options.color_representation.value = msg;
        this._isDirty = true;
    }
    changePixelGeometry (msg: number) {
        this._state.options.pixel_geometry.value = msg;
        this._isDirty = true;
    }
    changePixelShadowShape (msg: number) {
        this._state.options.pixel_shadow_shape.value = msg;
        this._isDirty = true;
    }
    changePixelShadowHeight (msg: number) {
        this._state.options.pixel_shadow_height.value = msg;
        this._isDirty = true;
    }
    changeBacklightPercent (msg: number) {
        this._state.options.backlight_percent.value = msg;
        this._isDirty = true;
    }
    changeInternalResolution (msg: number) {
        this._state.options.internal_resolution.value = msg;
        this._isDirty = true;
    }
    changeTextureInterpolation (msg: number) {
        this._state.options.texture_interpolation.value = msg;
        this._isDirty = true;
    }
    changeScreenCurvature (msg: number) {
        this._state.options.screen_curvature.value = msg;
        this._isDirty = true;
    }
    changeScalingMethod (msg: string) {
        this._state.options.scaling_method.value = msg;
        this._isDirty = true;
    }
    changeCustomScalingResWidth (width: number) {
        this._state.options.custom_resolution_width.value = width;
        this._isDirty = true;
    }
    changeCustomScalingResHeight (height: number) {
        this._state.options.custom_resolution_height.value = height;
        this._isDirty = true;
    }
    changeCustomScalingArX (x: number) {
        this._state.options.custom_aspect_ratio_x.value = x;
        this._isDirty = true;
    }
    changeCustomScalingArY (y: number) {
        this._state.options.custom_aspect_ratio_y.value = y;
        this._isDirty = true;
    }
    changeCustomScalingStretchNearest (stretch: number) {
        this._state.options.custom_scaling_stretch_nearest.value = stretch;
        this._isDirty = true;
    }
    changePerformance (performance: string) {
        this._state.options.webgl_performance.value = performance;
        this._isDirty = true;
        this._visibility.hideLoading();
    }
    changeAntialias (antialias: boolean) {
        this._state.options.webgl_antialias.value = antialias;
        this._isDirty = true;
        this._visibility.hideLoading();
    }
    changeColorGamma (gamma: number) {
        this._state.options.color_gamma.value = gamma;
        this._isDirty = true;
    }
    changeColorNoise (noise: number) {
        this._state.options.color_noise.value = noise;
        this._isDirty = true;
    }
    changeColorRgb (value: number, rgbRow: 'red' | 'green' | 'blue', rgbColumn: 'r' | 'g' | 'b') {
        this._state.options.rgb_values[rgbRow][rgbColumn].value = value;
        this._isDirty = true;
    }
}
