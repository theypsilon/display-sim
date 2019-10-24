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

const OPTION_SCALING_SELECT = 'option-scaling';
const OPTION_POWER_PREFERENCE_SELECT = 'option-powerPreference';
const OPTION_SCALING_CUSTOM_RES_WIDTH = 'option-scaling-custom-resolution-width';
const OPTION_SCALING_CUSTOM_RES_HEIGHT = 'option-scaling-custom-resolution-height';
const OPTION_SCALING_CUSTOM_AR_X = 'option-scaling-custom-aspect-ratio-x';
const OPTION_SCALING_CUSTOM_AR_Y = 'option-scaling-custom-aspect-ratio-y';
const OPTION_SCALING_CUSTOM_STRETCH_NEAREST = 'option-scaling-custom-stretch-nearest';
const OPTION_ANTIALIAS = 'option-antialias';
const OPTION_FILTER_PRESETS = 'option-filter-presets';

let instance;
export class Storage {
    static make () { return instance; }
    getScalingSelectOption () { return getItem(OPTION_SCALING_SELECT) || Constants.SCALING_AUTO_ID; }
    setScalingSelectOption (option) { setItem(Constants.OPTION_SCALING_SELECT, option); }
    getPowerPreferenceSelectOption () { return getItem(OPTION_POWER_PREFERENCE_SELECT) || Constants.POWER_PREFERENCE_DEFAULT; }
    setPowerPreferenceSelectOption (option) { setItem(OPTION_POWER_PREFERENCE_SELECT, option); }
    getCustomResWidth () { return getItem(OPTION_SCALING_CUSTOM_RES_WIDTH) || 256; }
    setCustomResWidth (width) { setItem(OPTION_SCALING_CUSTOM_RES_WIDTH, width); }
    getCustomResHeight () { return getItem(OPTION_SCALING_CUSTOM_RES_HEIGHT) || 224; }
    setCustomResHeight (height) { setItem(OPTION_SCALING_CUSTOM_RES_HEIGHT, height); }
    getCustomArX () { return getItem(OPTION_SCALING_CUSTOM_AR_X) || 4; }
    setCustomArX (x) { setItem(OPTION_SCALING_CUSTOM_AR_X, x); }
    getCustomArY () { return getItem(OPTION_SCALING_CUSTOM_AR_Y) || 3; }
    setCustomArY (y) { setItem(OPTION_SCALING_CUSTOM_AR_Y, y); }
    getCustomStretchNearest () { return getItem(OPTION_SCALING_CUSTOM_STRETCH_NEAREST) === 'true'; }
    setCustomStretchNearest (stretch) { setItem(OPTION_SCALING_CUSTOM_STRETCH_NEAREST, stretch ? 'true' : 'false'); }
    getAntiAliasing () { return getItem(OPTION_ANTIALIAS) !== 'false'; }
    setAntiAliasing (antiAliasing) { setItem(OPTION_ANTIALIAS, antiAliasing ? 'true' : 'false'); }
    getFilterPresets () { return getItem(OPTION_FILTER_PRESETS) || Constants.PRESET_KIND_APERTURE_GRILLE_1; }
    setFilterPresets (filterPresets) { setItem(OPTION_FILTER_PRESETS, filterPresets); }
    removeAllOptions () {
        removeItem(Constants.OPTION_SCALING_SELECT);
        removeItem(Constants.OPTION_POWER_PREFERENCE_SELECT);
        removeItem(OPTION_SCALING_CUSTOM_RES_WIDTH);
        removeItem(OPTION_SCALING_CUSTOM_RES_HEIGHT);
        removeItem(OPTION_SCALING_CUSTOM_AR_X);
        removeItem(OPTION_SCALING_CUSTOM_AR_Y);
        removeItem(OPTION_SCALING_CUSTOM_STRETCH_NEAREST);
        removeItem(OPTION_ANTIALIAS);
        removeItem(OPTION_FILTER_PRESETS);
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
