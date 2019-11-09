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

import initialize from './landing_initialize';

import { LandingVisibility } from './landing_visibility';
import { LandingStore } from './landing_store';

const template = document.createElement('template');
template.innerHTML = require('html-loader?interpolate!./landing_page.html');

class LandingPage extends HTMLElement {
    constructor () {
        super();
        const root = this.attachShadow({ mode: 'open' });
        root.appendChild(template.content.cloneNode(true));

        const constants = {
            SCALING_AUTO_ID: 'scaling-auto',
            SCALING_43_ID: 'scaling-4:3',
            SCALING_CUSTOM_ID: 'scaling-custom',
            SCALING_STRETCH_TO_BOTH_EDGES_ID: 'scaling-stretch-both',
            SCALING_STRETCH_TO_NEAREST_EDGE_ID: 'scaling-stretch-nearest',
            POWER_PREFERENCE_DEFAULT: 'default',
        
            FIRST_PREVIEW_IMAGE_ID: 'first-preview-image'
        };
        const elements = {
            uiDeo: root.getElementById('ui'),
            inputFileUploadDeo: root.getElementById('file'),
            startAnimationDeo: root.getElementById('start-animation'),
            antialiasDeo: root.getElementById('antialias'),
            scalingCustomResWidthDeo: root.getElementById('scaling-custom-resolution-width'),
            scalingCustomResHeightDeo: root.getElementById('scaling-custom-resolution-height'),
            scalingCustomResButtonDeo: root.getElementById('scaling-custom-resolution-button'),
            scalingCustomArXDeo: root.getElementById('scaling-custom-aspect-ratio-x'),
            scalingCustomArYDeo: root.getElementById('scaling-custom-aspect-ratio-y'),
            scalingCustomStretchNearestDeo: root.getElementById('scaling-custom-stretch-nearest'),
            scalingCustomInputsDeo: root.getElementById('scaling-custom-inputs'),
            dropZoneDeo: root.getElementById('drop-zone'),
            selectImageList: root.getElementById('select-image-list'),
            restoreDefaultOptionsDeo: root.getElementById('restore-default-options'),
        
            optionPowerPreferenceSelect: root.getElementById('option-powerPreference'),
            optionScalingSelect: root.getElementById('option-scaling'),

            previewDeo: root.getElementById(constants.FIRST_PREVIEW_IMAGE_ID)
        };

        initialize({ root, constants, elements, store: LandingStore.make(constants), visibility: LandingVisibility.make(elements) });
    }
}

window.customElements.define('landing-page', LandingPage);