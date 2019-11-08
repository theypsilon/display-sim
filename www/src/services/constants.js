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

export default {
    DISPLAY_NONE_CLASS: 'display-none',
    SCALING_AUTO_ID: 'scaling-auto',
    SCALING_43_ID: 'scaling-4:3',
    SCALING_CUSTOM_ID: 'scaling-custom',
    SCALING_STRETCH_TO_BOTH_EDGES_ID: 'scaling-stretch-both',
    SCALING_STRETCH_TO_NEAREST_EDGE_ID: 'scaling-stretch-nearest',
    POWER_PREFERENCE_DEFAULT: 'default',
    GL_CANVAS_ID: 'gl-canvas',
    TOP_MESSAGE_ID: 'top-message',
    APP_EVENT_TOP_MESSAGE: 'app-event.top_message',

    FIRST_PREVIEW_IMAGE_ID: 'first-preview-image',

    uiDeo: document.getElementById('ui'),
    loadingDeo: document.getElementById('loading'),
    inputFileUploadDeo: document.getElementById('file'),
    startAnimationDeo: document.getElementById('start-animation'),
    antialiasDeo: document.getElementById('antialias'),
    scalingCustomResWidthDeo: document.getElementById('scaling-custom-resolution-width'),
    scalingCustomResHeightDeo: document.getElementById('scaling-custom-resolution-height'),
    scalingCustomResButtonDeo: document.getElementById('scaling-custom-resolution-button'),
    scalingCustomArXDeo: document.getElementById('scaling-custom-aspect-ratio-x'),
    scalingCustomArYDeo: document.getElementById('scaling-custom-aspect-ratio-y'),
    scalingCustomStretchNearestDeo: document.getElementById('scaling-custom-stretch-nearest'),
    scalingCustomInputsDeo: document.getElementById('scaling-custom-inputs'),
    dropZoneDeo: document.getElementById('drop-zone'),
    selectImageList: document.getElementById('select-image-list'),
    restoreDefaultOptionsDeo: document.getElementById('restore-default-options'),

    optionPowerPreferenceSelect: document.getElementById('option-powerPreference'),
    optionScalingSelect: document.getElementById('option-scaling')
};
