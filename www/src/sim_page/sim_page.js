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

import Constants from '../constants';

import { prepareMainPage } from '../main_page/load';

import { Visibility } from '../visibility';

import './input_fields';
import './panel_visibility';
import './presets_selection';
import './screenshot';
import './sync_values';
import './tabs';

const getGlCanvasDeo = () => document.getElementById(Constants.glCanvasHtmlId);
const visibility = Visibility.make();

window.addEventListener('app-event.exit_pointer_lock', () => {
    document.exitPointerLock();
}, false);

window.addEventListener('app-event.exiting_session', () => {
    prepareMainPage();
    getGlCanvasDeo().remove();
    visibility.hideSimulationUi();
}, false);

window.addEventListener('app-event.fps', event => {
    Constants.fpsCounterDeo.innerHTML = Math.round(event.detail);
}, false);
