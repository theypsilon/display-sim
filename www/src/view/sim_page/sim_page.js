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

import { prepareMainPage } from '../main_page/load';

import { Visibility } from '../../services/visibility';

import inputFields from './input_fields';
import panelVisibility from './panel_visibility';
import presetsSelection from './presets_selection';
import screenshot from './screenshot';
import syncValues from './sync_values';
import tabs from './tabs';

const visibility = Visibility.make();

const template = document.createElement('template');
template.innerHTML = `
<style>
    ${require('!css-loader!../../css/all.css').toString()}
</style>

<div id="simulation-ui" class="display-none">
    <div id="simulation-text">
    <ul>
        <li><span id="feature-quit">ESC</span> to quit simulation.</li>
        <li><span class="toggle-info-panel">SPACE</span> to toggle panel.</li>
    </ul>
    </div>
    <div id="info-panel">
        <input id="feature-close-panel" title="Hide this panel." class="btn btn-crt btn-inverted-grey text-white" type="button" value="&times;" onclick="this.blur();">
        <div id="info-panel-content">
            <div id="fps-counter-holder"><div id="fps-counter"></div><span>FPS</span></div>
            <h3>Sim Panel</h3>
            <div class="tab-container">
            <ul class="tabs clearfix" >
                <li id="panel-advanced">
                <a href=# >Advanced</a>
                </li>
                <li id="panel-basic" class='active'>
                <a href=# >Basic</a>
                </li>
            </ul>
            </div>
            <div id="info-panel-basic-settings" class="info-panel-settings">
            <div class="info-category">
                <div><h5>Presets</h5></div>
                <a class="btn preset-btn" data-preset="crt-aperture-grille-1" href="#">CRT Aperture Grille 1</a>
                <a class="btn preset-btn" data-preset="crt-shadow-mask-1" href="#">CRT Shadow Mask 1</a>
                <a class="btn preset-btn" data-preset="crt-shadow-mask-2" href="#">CRT Shadow Mask 2</a>
                <a class="btn preset-btn" data-preset="sharp-1" href="#">Sharp Pixels</a>
                <a class="btn preset-btn" data-preset="demo-1" href="#">Flight Demo</a>
                <a class="btn preset-btn" data-preset="custom" href="#">Custom</a>
            </div>
            <div class="info-category">
                <div><h5>Properties</h5></div>
                <li title="Screen curvature type Hotkeys: B"><div><div class="feature-name">Screen curvature type</div><sup class="feature-hotkeys">(B)</sup></div><div><input class="number-input feature-readonly-input" id="feature-change-screen-curvature-basic" type="text" value="0" disabled></div></li>
                <li title="Internal Resolution Hotkeys: Y"><div><div class="feature-name">Internal Resolution</div><sup class="feature-hotkeys">(Y)</sup></div><div><input class="number-input feature-readonly-input" id="feature-internal-resolution-basic" type="text" disabled></div></li>
            </div>
            </div>
            <div id="info-panel-advanced-settings" class="info-panel-settings display-none">
                <div class="info-category">
                    <div><h5>Camera Movements <div id="reset-camera" class="reset-button"><button class="activate-button" title="All camera parameters will be reset.">Reset Position</button></div></h5></div>
                    <div id="camera-right-panel">
                        <div id="camera-movement-indicator">
                        <div class="camera-field-header"><div class="camera-coordinate">X</div><div class="camera-coordinate">Y</div><div class="camera-coordinate">Z</div></div>
                        <div id="camera-pos" class="camera-field-group">
                        <div class="camera-field-name">pos:</div>
                        <input id="camera-pos-x" class="camera-field camera-field-left" type="number" step="0.01"/>
                        <input id="camera-pos-y" class="camera-field camera-field-center" type="number" step="0.01"/>
                        <input id="camera-pos-z" class="camera-field camera-field-right" type="number" step="0.01"/>
                        </div>
                        <div id="camera-dir" class="camera-field-group">
                        <div class="camera-field-name">dir:</div>
                        <input id="camera-dir-x" class="camera-field camera-field-left camera-field-mid" type="number" step="0.01"/>
                        <input id="camera-dir-y" class="camera-field camera-field-center camera-field-mid" type="number" step="0.01"/>
                        <input id="camera-dir-z" class="camera-field camera-field-right camera-field-mid" type="number" step="0.01"/>
                        </div>
                        <div id="camera-axis-up" class="camera-field-group">
                        <div class="camera-field-name">axis:</div>
                        <input id="camera-axis-up-x" class="camera-field camera-field-left" type="number" step="0.01"/>
                        <input id="camera-axis-up-y" class="camera-field camera-field-center" type="number" step="0.01"/>
                        <input id="camera-axis-up-z" class="camera-field camera-field-right" type="number" step="0.01"/>
                        </div>
                    </div>
                    <ul title="Zoom Hotkeys: Mouse wheel up & down"><li><div class="feature-name">· Zoom:</div><br><div id="camera-zoom-parent" class="feature-value"><input id="camera-zoom" class="number-input feature-modificable-input" type="number" placeholder="0" step="1" min="1" max="45" value="0"></div></li></ul>
                    </div>
                    <ul>
                    <li><div class="feature-name">· Translation:</div><br><div id="feature-camera-movements" class="camera-movements-arrows">
                        <div><button class="activate-button feature-movement-keys key-up-center">W</button><div class="free-mode-only-controls display-none"><button class="activate-button feature-movement-keys key-up-far-right">Q</button> ↑</div></div><br/>
                        <div><button class="activate-button feature-movement-keys">A</button><button class="activate-button feature-movement-keys key-down-center">S</button><button class="activate-button feature-movement-keys">D</button><div class="free-mode-only-controls display-none"><button class="activate-button feature-movement-keys key-down-far-right">E</button> ↓</div></div>
                    </div></li>
                    <li style="height: 65px"><div class="feature-name">· Rotation:</div><br><div id="feature-camera-turns" class="camera-movements-arrows">
                        <div><button class="activate-button feature-movement-keys key-up-center">↑</button><button class="activate-button feature-movement-keys key-up-far-right">+</button> ⟳</div><br/>
                        <div><button class="activate-button feature-movement-keys">←</button><button class="activate-button feature-movement-keys key-down-center">↓</button><button class="activate-button feature-movement-keys">→</button><button class="activate-button feature-movement-keys key-down-far-right">-</button> ⟲</div>
                    </div></li>
                    <li title="Movement mode Hotkeys: G" id="movement-mode"><div class="feature-name">· Movement Type:</div>
                        <div id="feature-camera-movement-mode" class="feature-value">
                            <input id="camera-movement-mode" class="number-input feature-readonly-input" type="text" value="Lock on Display" disabled>
                        </div>
                    </li>
                    </ul>
                </div>
                <div class="info-category">
                    <h5>Screen Filter Options <div id="reset-filters" class="reset-button"><button class="activate-button" title="All screen filter options will be reset.">Reset Filters</button></div></h5>
                    <li title="Horizontal gap Hotkeys: U & Shift+U"><div><div class="feature-name">Horizontal gap</div><sup class="feature-hotkeys">(U, Shift+U)</sup></div><div class="feature-value"><input id="pixel-horizontal-gap" class="number-input feature-modificable-input" type="number" placeholder="0" step="0.001" min="0" max="10" value="0"></div></li>
                    <li title="Vertical gap Hotkeys: I & Shift+I"><div><div class="feature-name">Vertical gap</div><sup class="feature-hotkeys">(I, Shift+I)</sup></div><div class="feature-value"><input id="pixel-vertical-gap" class="number-input feature-modificable-input" type="number" placeholder="0" step="0.001" min="0" max="10" value="0"></div></li>
                    <li title="Width Hotkeys: O & Shift+O"><div><div class="feature-name">Pixel width</div><sup class="feature-hotkeys">(O, Shift+O)</sup></div><div class="feature-value"><input id="pixel-width" class="number-input feature-modificable-input" type="number" placeholder="0" step="0.001" min="0" max="10" value="0"></div></li>
                    <li title="Blur Level Hotkeys: J & Shift+J"><div><div class="feature-name">Blur level</div><sup class="feature-hotkeys">(J, Shift+J)</sup></div><div class="feature-value"><input id="blur-level" class="number-input feature-modificable-input" type="number" placeholder="0" step="1" min="0" max="100" value="0"></div></li>
                    <li title="Vertical lines per pixel Hotkeys: K & Shift+K"><div><div class="feature-name">Vertical lines per pixel</div><sup class="feature-hotkeys">(K, Shift+K)</sup></div><div class="feature-value"><input id="vertical-lpp" class="number-input feature-modificable-input" type="number" placeholder="0" step="1" min="0" max="100" value="0"></div></li>
                    <li title="Horizontal lines per pixel Hotkeys: L & Shift+L"><div><div class="feature-name">Horizontal lines per pixel</div><sup class="feature-hotkeys">(L, Shift+L)</sup></div><div class="feature-value"><input id="horizontal-lpp" class="number-input feature-modificable-input" type="number" placeholder="0" step="1" min="0" max="100" value="0"></div></li>
                    <li><div class="feature-name">Source light color  </div><div><input class="feature-button" id="light-color" type="color" value="#ffffff"></div></li>
                    <li title="Brightness Hotkeys: X & Shift+X"><div><div class="feature-name">Brigthness</div><sup class="feature-hotkeys">(X, Shift+X)</sup></div><div class="feature-value"><input id="pixel-brightness" class="number-input feature-modificable-input" type="number" placeholder="0" step="0.001" min="-1" max="1" value="0"></div></li>
                    <li title="Contrast Hotkeys: Z & Shift+Z"><div><div class="feature-name">Contrast</div><sup class="feature-hotkeys">(Z, Shift+Z)</sup></div><div class="feature-value"><input id="pixel-contrast" class="number-input feature-modificable-input" type="number" placeholder="0" step="0.001" min="0" max="20" value="1"></div></li>
                    <li title="Color channels Hotkeys: C"><div><div class="feature-name">Color channels type</div><sup class="feature-hotkeys">(C)</sup></div><div><input class="number-input feature-readonly-input" id="feature-change-color-representation" type="text" value="0" disabled></div></li>
                    <li title="Pixel geometry Hotkeys: V"><div><div class="feature-name">Pixel geometry type</div><sup class="feature-hotkeys">(V)</sup></div><div><input class="number-input feature-readonly-input" id="feature-change-pixel-geometry" type="text" value="0" disabled></div></li>
                    <li title="Screen curvature type Hotkeys: B"><div><div class="feature-name">Screen curvature type</div><sup class="feature-hotkeys">(B)</sup></div><div><input class="number-input feature-readonly-input" id="feature-change-screen-curvature" type="text" value="0" disabled></div></li>
                    <li title="Pixel texture Hotkeys: N"><div><div class="feature-name">Pixel texture</div><sup class="feature-hotkeys">(N)</sup></div><div><input class="number-input feature-readonly-input" id="feature-change-pixel-shadow-shape" type="text" value="0" disabled></div></li>
                    <li title="Pixel variable height Hotkeys: M"><div><div class="feature-name">Pixel variable height</div><sup class="feature-hotkeys">(M)</sup></div><div><input class="number-input feature-modificable-input" id="feature-change-pixel-shadow-height" type="number" placeholder="0" step="0.001" min="0" max="1" value="0"></div></li>
                    <li title="Internal Resolution Hotkeys: Y"><div><div class="feature-name">Internal Resolution</div><sup class="feature-hotkeys">(Y)</sup></div><div><input class="number-input feature-readonly-input" id="feature-internal-resolution" type="text" disabled></div></li>
                    <li title="Texture Interpolation Hotkeys: H"><div><div class="feature-name">Texture Interpolation</div><sup class="feature-hotkeys">(H)</sup></div><div><input class="number-input feature-readonly-input" id="feature-texture-interpolation" type="text" disabled></div></li>
                    <li title="Backlight Hotkeys: ',' & '.'"><div><div class="feature-name">Backlight</div><sup class="feature-hotkeys">( ,  . )</sup></div><div><input class="number-input feature-modificable-input" id="feature-backlight-percent" type="number" placeholder="0.5" step="0.001" min="0" max="1" value="0.5"></div></li>
                    <li title="Pixel spread Hotkeys: P & Shift+P" class="display-none"><div><div class="feature-name">Pixel Spread</div><sup class="feature-hotkeys">(P, Shift+P)</sup><div class="feature-value"><input id="pixel-spread" class="number-input feature-modificable-input" type="number" placeholder="0" step="0.001" min="0" max="10" value="0"></div></div></li>
                    <li class="display-none"><div><div class="feature-name">Brightness color</div></div><div><input class="feature-button" id="brightness-color" type="color" value="#ffffff"></div></li>
                    </ul>
                </div>
                <div class="info-category">
                    <h5>Velocity <div id="reset-speeds" class="reset-button"><button class="activate-button" title="All speed parameters will be reset. Hotkey: T">Reset Speeds</button></div></h5>
                    <ul class="info-options">
                    <li title="Camera speed Hotkeys: F & R"><div><div class="feature-name">Camera speed</div><sup class="feature-hotkeys">(F, R)</sup></div><div class="feature-value"><input id="feature-change-move-speed" class="number-input feature-readonly-input" type="text" disabled></div></li>
                    <li title="Filter speed Hotkeys: Shift+F & Shift+R"><div><div class="feature-name">Filter speed</div><sup class="feature-hotkeys">(Shift+F, Shift+R)</sup></div><div class="feature-value"><input id="feature-change-pixel-speed" class="number-input feature-readonly-input" type="text" disabled></div></li>
                    <li title="Turn speed Hotkeys: Alt+F & Alt+R" class="display-none"><div><div class="feature-name">Turn speed</div><sup class="feature-hotkeys">(Alt+F, Alt+R)</sup></div><div class="feature-value"><input id="feature-change-turn-speed" class="number-input feature-readonly-input" type="text" disabled></div></li>
                    </ul>
                </div>
                <div class="info-button">
                    <input title="Hotkey: F4" class="btn btn-crt btn-dark-grey text-white" type="button" id="feature-capture-framebuffer" value="Capture framebuffer" onclick="this.blur();">
                </div>
            </div>
        </div>
    </div>
</div>
`;

class SimPage extends HTMLElement {
    constructor() {
        super();
        const root = this.attachShadow({ mode: 'open' });
        root.appendChild(template.content.cloneNode(true));
        const constants = {
            PRESET_KIND_APERTURE_GRILLE_1: 'crt-aperture-grille-1',
            PRESET_KIND_SHADOW_MASK_1: 'crt-shadow-mask-1',
            PRESET_KIND_SHADOW_MASK_2: 'crt-shadow-mask-2',
            PRESET_KIND_SHARP_1: 'sharp-1',
            PRESET_KIND_FLIGHT_DEMO_1: 'demo-1',
            PRESET_KIND_CUSTOM: 'custom',
            PRESET_ACTIVE_CLASS: 'active-preset',
        
            TAB_PANEL_BASIC: 'panel-basic',
            TAB_PANEL_ADVANCED: 'panel-advanced',
        
            APP_EVENT_CAMERA_UPDATE: 'app-event.camera_update',
            APP_EVENT_CUSTOM_INPUT: 'app-event.custom_input_event',
            APP_EVENT_CHANGE_PIXEL_HORIZONTAL_GAP: 'app-event.change_pixel_horizontal_gap',
            APP_EVENT_CHANGE_PIXEL_VERTICAL_GAP: 'app-event.change_pixel_vertical_gap',
            APP_EVENT_CHANGE_PIXEL_WIDTH: 'app-event.change_pixel_width',
            APP_EVENT_CHANGE_PIXEL_SPREAD: 'app-event.change_pixel_spread',
            APP_EVENT_CHANGE_PIXEL_BRIGHTNESS: 'app-event.change_pixel_brightness',
            APP_EVENT_CHANGE_PIXEL_CONTRAST: 'app-event.change_pixel_contrast',
            APP_EVENT_CHANGE_LIGHT_COLOR: 'app-event.change_light_color',
            APP_EVENT_CHANGE_BRIGHTNESS_COLOR: 'app-event.change_brightness_color',
            APP_EVENT_CHANGE_CAMERA_ZOOM: 'app-event.change_camera_zoom',
            APP_EVENT_CHANGE_BLUR_LEVEL: 'app-event.change_blur_level',
            APP_EVENT_CHANGE_VERTICAL_LPP: 'app-event.change_vertical_lpp',
            APP_EVENT_CHANGE_HORIZONTAL_LPP: 'app-event.change_horizontal_lpp',
            APP_EVENT_COLOR_REPRESENTATION: 'app-event.color_representation',
            APP_EVENT_PIXEL_GEOMETRY: 'app-event.pixel_geometry',
            APP_EVENT_PIXEL_SHADOW_SHAPE: 'app-event.pixel_shadow_shape',
            APP_EVENT_PIXEL_SHADOW_HEIGHT: 'app-event.pixel_shadow_height',
            APP_EVENT_BACKLIGHT_PERCENT: 'app-event.backlight_percent',
            APP_EVENT_SCREEN_CURVATURE: 'app-event.screen_curvature',
            APP_EVENT_INTERNAL_RESOLUTION: 'app-event.internal_resolution',
            APP_EVENT_TEXTURE_INTERPOLATION: 'app-event.texture_interpolation',
            APP_EVENT_CHANGE_PIXEL_SPEED: 'app-event.change_pixel_speed',
            APP_EVENT_CHANGE_TURNING_SPEED: 'app-event.change_turning_speed',
            APP_EVENT_CHANGE_MOVEMENT_SPEED: 'app-event.change_movement_speed',
            APP_EVENT_EXITING_SESSION: 'app-event.exiting_session',
            APP_EVENT_TOGGLE_INFO_PANEL: 'app-event.toggle_info_panel',
            APP_EVENT_FPS: 'app-event.fps',
            APP_EVENT_REQUEST_POINTER_LOCK: 'app-event.request_pointer_lock',
            APP_EVENT_EXIT_POINTER_LOCK: 'app-event.exit_pointer_lock',
            APP_EVENT_SCREENSHOT: 'app-event.screenshot',
            APP_EVENT_PRESET_SELECTED_NAME: 'app-event.preset_selected_name',
            APP_EVENT_CHANGE_CAMERA_MOVEMENT_MODE: 'app-event.change_camera_movement_mode',
            APP_EVENT_TOP_MESSAGE: 'app-event.top_message',
        
            EVENT_KIND_FILTER_PRESETS_SELECTED: 'filter_presets_selected',
            EVENT_KIND_PIXEL_BRIGHTNESS: 'pixel_brightness',
            EVENT_KIND_PIXEL_CONTRAST: 'pixel_contrast',
            EVENT_KIND_LIGHT_COLOR: 'light_color',
            EVENT_KIND_BRIGHTNESS_COLOR: 'brightness_color',
            EVENT_KIND_BLUR_LEVEL: 'blur_level',
            EVENT_KIND_VERTICAL_LPP: 'vertical_lpp',
            EVENT_KIND_HORIZONTAL_LPP: 'horizontal_lpp',
            EVENT_KIND_PIXEL_SHADOW_HEIGHT: 'pixel_shadow_height',
            EVENT_KIND_PIXEL_VERTICAL_GAP: 'pixel_vertical_gap',
            EVENT_KIND_PIXEL_HORIZONTAL_GAP: 'pixel_horizontal_gap',
            EVENT_KIND_PIXEL_WIDTH: 'pixel_width',
            EVENT_KIND_PIXEL_SPREAD: 'pixel_spread',
            EVENT_KIND_BACKLIGHT_PERCENT: 'backlight_percent',
            EVENT_KIND_CAMERA_ZOOM: 'camera_zoom',
            EVENT_KIND_CAMERA_POS_X: 'camera_pos_x',
            EVENT_KIND_CAMERA_POS_Y: 'camera_pos_y',
            EVENT_KIND_CAMERA_POS_Z: 'camera_pos_z',
            EVENT_KIND_CAMERA_AXIS_UP_X: 'camera_axis_up_x',
            EVENT_KIND_CAMERA_AXIS_UP_Y: 'camera_axis_up_y',
            EVENT_KIND_CAMERA_AXIS_UP_Z: 'camera_axis_up_z',
            EVENT_KIND_CAMERA_DIRECTION_X: 'camera_direction_x',
            EVENT_KIND_CAMERA_DIRECTION_Y: 'camera_direction_y',
            EVENT_KIND_CAMERA_DIRECTION_Z: 'camera_direction_z',
            EVENT_KIND_PREFIX: 'event-kind:',

            toggleInfoPanelClass: document.querySelectorAll('.toggle-info-panel'),
            freeModeControlsClas: document.querySelectorAll('.free-mode-only-controls'),
        
            tabsSelector: root.querySelectorAll('.tabs > li'),
            simulationUiDeo: root.getElementById('simulation-ui'),
            infoPanelDeo: root.getElementById('info-panel'),
            infoPanelAdvancedSettingsDeo: root.getElementById('info-panel-advanced-settings'),
            infoPanelContentDeo: root.getElementById('info-panel-content'),
            fpsCounterDeo: root.getElementById('fps-counter'),
            lightColorDeo: root.getElementById('light-color'),
            brightnessColorDeo: root.getElementById('brightness-color'),
        
            cameraPosXDeo: root.getElementById('camera-pos-x'),
            cameraPosYDeo: root.getElementById('camera-pos-y'),
            cameraPosZDeo: root.getElementById('camera-pos-z'),
            cameraDirXDeo: root.getElementById('camera-dir-x'),
            cameraDirYDeo: root.getElementById('camera-dir-y'),
            cameraDirZDeo: root.getElementById('camera-dir-z'),
            cameraAxisUpXDeo: root.getElementById('camera-axis-up-x'),
            cameraAxisUpYDeo: root.getElementById('camera-axis-up-y'),
            cameraAxisUpZDeo: root.getElementById('camera-axis-up-z'),
            cameraZoomDeo: root.getElementById('camera-zoom'),
            cameraMovementModeDeo: root.getElementById('camera-movement-mode'),
        
            filterPresetsButtonDeoList: Array.from(root.querySelectorAll('.preset-btn')),
            pixelWidthDeo: root.getElementById('pixel-width'),
            pixelHorizontalGapDeo: root.getElementById('pixel-horizontal-gap'),
            pixelVerticalGapDeo: root.getElementById('pixel-vertical-gap'),
            pixelSpreadDeo: root.getElementById('pixel-spread'),
            pixelBrigthnessDeo: root.getElementById('pixel-brightness'),
            pixelContrastDeo: root.getElementById('pixel-contrast'),
            blurLevelDeo: root.getElementById('blur-level'),
            verticalLppDeo: root.getElementById('vertical-lpp'),
            horizontalLppDeo: root.getElementById('horizontal-lpp'),
            featureQuitDeo: root.getElementById('feature-quit'),
            featureCaptureFramebufferDeo: root.getElementById('feature-capture-framebuffer'),
            featureClosePanelDeo: root.getElementById('feature-close-panel'),
        
            featureChangeColorRepresentationDeo: root.getElementById('feature-change-color-representation'),
            featureChangePixelGeometryDeo: root.getElementById('feature-change-pixel-geometry'),
            featureChangePixelShadowShapeDeo: root.getElementById('feature-change-pixel-shadow-shape'),
            featureChangePixelShadowHeightDeo: root.getElementById('feature-change-pixel-shadow-height'),
            featureChangeScreenCurvatureDeo: root.getElementById('feature-change-screen-curvature'),
            featureChangeScreenCurvatureBasicDeo: root.getElementById('feature-change-screen-curvature-basic'),
            featureInternalResolutionDeo: root.getElementById('feature-internal-resolution'),
            featureInternalResolutionBasicDeo: root.getElementById('feature-internal-resolution-basic'),
            featureTextureInterpolationDeo: root.getElementById('feature-texture-interpolation'),
            featureBacklightPercentDeo: root.getElementById('feature-backlight-percent'),
        
            featureChangeMoveSpeedDeo: root.getElementById('feature-change-move-speed'),
            featureChangeTurnSpeedDeo: root.getElementById('feature-change-turn-speed'),
            featureChangePixelSpeedDeo: root.getElementById('feature-change-pixel-speed'),
            featureCameraMovementsDeo: root.getElementById('feature-camera-movements'),
            featureCameraTurnsDeo: root.getElementById('feature-camera-turns'),
            resetCameraDeo: root.getElementById('reset-camera'),
            resetFiltersDeo: root.getElementById('reset-filters'),
            resetSpeedsDeo: root.getElementById('reset-speeds'),

            infoPanelBasicDeo: root.getElementById('info-panel-basic-settings'),
            infoPanelAdvancedDeo: root.getElementById('info-panel-advanced-settings')
        };

        const ctx = { root, constants };
        inputFields(ctx);
        panelVisibility(ctx);
        presetsSelection(ctx);
        screenshot(ctx);
        syncValues(ctx);
        tabs(ctx);

        visibility.setSimPageConstants(ctx.constants);
    }
}

window.customElements.define('sim-page', SimPage);

const getGlCanvasDeo = () => document.getElementById(Constants.GL_CANVAS_ID);

window.addEventListener(Constants.APP_EVENT_EXIT_POINTER_LOCK, () => {
    document.exitPointerLock();
}, false);

window.addEventListener(Constants.APP_EVENT_EXITING_SESSION, () => {
    prepareMainPage();
    getGlCanvasDeo().remove();
    visibility.hideSimulationUi();
}, false);

window.addEventListener(Constants.APP_EVENT_FPS, event => {
    Constants.fpsCounterDeo.innerHTML = Math.round(event.detail);
}, false);
