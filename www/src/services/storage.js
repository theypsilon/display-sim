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

import Constants from './constants';

const optionScalingSelect = 'option-scaling';
const optionPowerPreferenceSelect = 'option-powerPreference';
const optionScalingCustomResWidth = 'option-scaling-custom-resolution-width';
const optionScalingCustomResHeight = 'option-scaling-custom-resolution-height';
const optionScalingCustomArX = 'option-scaling-custom-aspect-ratio-x';
const optionScalingCustomArY = 'option-scaling-custom-aspect-ratio-y';
const optionScalingCustomStretchNearest = 'option-scaling-custom-stretch-nearest';
const optionAntialias = 'option-antialias';
const optionFilterPresets = 'option-filter-presets';

let instance;
export class Storage {
    static make () { return instance; }
    getScalingSelectOption () { return getItem(optionScalingSelect) || Constants.scalingAutoHtmlId; }
    setScalingSelectOption (option) { setItem(Constants.optionScalingSelect, option); }
    getPowerPreferenceSelectOption () { return getItem(optionPowerPreferenceSelect) || Constants.powerPreferenceDefaultHtml; }
    setPowerPreferenceSelectOption (option) { setItem(optionPowerPreferenceSelect, option); }
    getCustomResWidth () { return getItem(optionScalingCustomResWidth) || 256; }
    setCustomResWidth (width) { setItem(optionScalingCustomResWidth, width); }
    getCustomResHeight () { return getItem(optionScalingCustomResHeight) || 224; }
    setCustomResHeight (height) { setItem(optionScalingCustomResHeight, height); }
    getCustomArX () { return getItem(optionScalingCustomArX) || 4; }
    setCustomArX (x) { setItem(optionScalingCustomArX, x); }
    getCustomArY () { return getItem(optionScalingCustomArY) || 3; }
    setCustomArY (y) { setItem(optionScalingCustomArY, y); }
    getCustomStretchNearest () { return getItem(optionScalingCustomStretchNearest) === 'true'; }
    setCustomStretchNearest (stretch) { setItem(optionScalingCustomStretchNearest, stretch ? 'true' : 'false'); }
    getAntiAliasing () { return getItem(optionAntialias) !== 'false'; }
    setAntiAliasing (antiAliasing) { setItem(optionAntialias, antiAliasing ? 'true' : 'false'); }
    getFilterPresets () { return getItem(optionFilterPresets) || Constants.presetApertureGrille1; }
    setFilterPresets (filterPresets) { setItem(optionFilterPresets, filterPresets); }
    removeAllOptions () {
        removeItem(Constants.optionScalingSelect);
        removeItem(Constants.optionPowerPreferenceSelect);
        removeItem(optionScalingCustomResWidth);
        removeItem(optionScalingCustomResHeight);
        removeItem(optionScalingCustomArX);
        removeItem(optionScalingCustomArY);
        removeItem(optionScalingCustomStretchNearest);
        removeItem(optionAntialias);
        removeItem(optionFilterPresets);
    }
}

instance = new Storage();

function getItem (key) {
    return localStorage.getItem('DISPLAY_SIM.' + key);
}
function setItem (key, value) {
    localStorage.setItem('DISPLAY_SIM.' + key, value);
}
function removeItem (key) {
    localStorage.removeItem('DISPLAY_SIM.' + key);
}
