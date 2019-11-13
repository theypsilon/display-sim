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

import { renderTemplate } from './sim_template';
import { model, View } from './sim_view_model';

const store = LocalStorage.make('sim_page/presets_selection');
const state = model(store);
const messenger = Messenger.getInstance();

class SimPage extends HTMLElement {
    constructor () {
        super();

        this._state = state;
        this._root = this.attachShadow({ mode: 'open' });
        this._view = View.make(state, this, store); // so it can be readed during the first template generation

        this.refresh();

        // This element works as canvas and also as an event bus
        this._view.canvas = this._root.getElementById('gl-canvas-id');

        setupEvents(this._view.canvas, this._view);

        document.body.style.setProperty('overflow', 'hidden');
        document.body.style.setProperty('background-color', 'black');
    }

    disconnectedCallback () {
        document.body.style.removeProperty('overflow');
        document.body.style.removeProperty('background-color');
    }

    refresh () {
        renderTemplate(this._state, this._view, this._root);
    }
}

window.customElements.define('sim-page', SimPage);

function setupEvents (canvas, view) {
    messenger.consumeInbox('sim-page').forEach(async msg => {
        switch (msg.topic) {
        case 'launch': return view.launchSimulation(msg);
        default: throw new Error('Wrong topic: ' + msg.topic);
        }
    });
    // Forwarding keyboard events so it can be readed by the backend
    window.addEventListener('keydown', e => canvas.dispatchEvent(new KeyboardEvent('keydown', { key: e.key })), false);
    window.addEventListener('keyup', e => canvas.dispatchEvent(new KeyboardEvent('keyup', { key: e.key })), false);

    canvas.onfocus = () => canvas.dispatchEvent(new KeyboardEvent('keydown', { key: 'canvas_focused' }));
    canvas.onblur = () => canvas.dispatchEvent(new KeyboardEvent('keyup', { key: 'canvas_focused' }));    
    // Listening backend events
    canvas.addEventListener('display-sim-event:backend-channel', e => {
        const msg = e.detail.message;
        switch (e.detail.type) {
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
        default:
            throw new Error('Not covered following backend event: ' + e.detail.type, msg);
        }
    }, false);
}