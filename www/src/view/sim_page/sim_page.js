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

import { SimVisibility } from './sim_visibility';

import initialize from './sim_initialize';

const template = document.createElement('template');
template.innerHTML = require('html-loader?interpolate!./sim_page.html');

class SimPage extends HTMLElement {
    constructor () {
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
            EVENT_KIND_PREFIX: 'event-kind:'
        };

        const elements = {
            glCanvasDeo: root.getElementById('gl-canvas-id'), 

            freeModeControlsClas: root.querySelectorAll('.free-mode-only-controls'),
        
            simulationUiDeo: root.getElementById('simulation-ui'),
            infoPanelDeo: root.getElementById('info-panel'),
            infoPanelToggleDeo: root.getElementById('info-panel-toggle'),
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
            featureInternalResolutionDeo: root.getElementById('feature-internal-resolution'),
            featureTextureInterpolationDeo: root.getElementById('feature-texture-interpolation'),
            featureBacklightPercentDeo: root.getElementById('feature-backlight-percent'),
        
            featureChangeMoveSpeedDeo: root.getElementById('feature-change-move-speed'),
            featureChangeTurnSpeedDeo: root.getElementById('feature-change-turn-speed'),
            featureChangePixelSpeedDeo: root.getElementById('feature-change-pixel-speed'),
            featureCameraMovementsDeo: root.getElementById('feature-camera-movements'),
            featureCameraTurnsDeo: root.getElementById('feature-camera-turns')
        };

        initialize({ root, constants, elements, eventBus: elements.glCanvasDeo, visibility: SimVisibility.make(elements) });

        document.body.style.setProperty('overflow', 'hidden');
        document.body.style.setProperty('background-color', 'black');
    }

    disconnectedCallback () {
        document.body.style.removeProperty('overflow');
        document.body.style.removeProperty('background-color');
    }
}

window.customElements.define('sim-page', SimPage);