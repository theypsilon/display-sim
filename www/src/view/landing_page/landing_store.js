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
import { LocalStorage } from '../../services/local_storage';

const OPTION_SCALING_SELECT = 'option-scaling';
const OPTION_POWER_PREFERENCE_SELECT = 'option-powerPreference';
const OPTION_SCALING_CUSTOM_RES_WIDTH = 'option-scaling-custom-resolution-width';
const OPTION_SCALING_CUSTOM_RES_HEIGHT = 'option-scaling-custom-resolution-height';
const OPTION_SCALING_CUSTOM_AR_X = 'option-scaling-custom-aspect-ratio-x';
const OPTION_SCALING_CUSTOM_AR_Y = 'option-scaling-custom-aspect-ratio-y';
const OPTION_SCALING_CUSTOM_STRETCH_NEAREST = 'option-scaling-custom-stretch-nearest';
const OPTION_ANTIALIAS = 'option-antialias';

export class LandingStore {
    constructor (localStorage) {
        this.localStorage = localStorage;
    }
    static make () {
        return new LandingStore(new LocalStorage('LandingStore'));
    }
    getScalingSelectOption () { return this.localStorage.getItem(OPTION_SCALING_SELECT) || Constants.SCALING_AUTO_ID; }
    setScalingSelectOption (option) { this.localStorage.setItem(OPTION_SCALING_SELECT, option); }
    getPowerPreferenceSelectOption () { return this.localStorage.getItem(OPTION_POWER_PREFERENCE_SELECT) || Constants.POWER_PREFERENCE_DEFAULT; }
    setPowerPreferenceSelectOption (option) { this.localStorage.setItem(OPTION_POWER_PREFERENCE_SELECT, option); }
    getCustomResWidth () { return this.localStorage.getItem(OPTION_SCALING_CUSTOM_RES_WIDTH) || 256; }
    setCustomResWidth (width) { this.localStorage.setItem(OPTION_SCALING_CUSTOM_RES_WIDTH, width); }
    getCustomResHeight () { return this.localStorage.getItem(OPTION_SCALING_CUSTOM_RES_HEIGHT) || 224; }
    setCustomResHeight (height) { this.localStorage.setItem(OPTION_SCALING_CUSTOM_RES_HEIGHT, height); }
    getCustomArX () { return this.localStorage.getItem(OPTION_SCALING_CUSTOM_AR_X) || 4; }
    setCustomArX (x) { this.localStorage.setItem(OPTION_SCALING_CUSTOM_AR_X, x); }
    getCustomArY () { return this.localStorage.getItem(OPTION_SCALING_CUSTOM_AR_Y) || 3; }
    setCustomArY (y) { this.localStorage.setItem(OPTION_SCALING_CUSTOM_AR_Y, y); }
    getCustomStretchNearest () { return this.localStorage.getItem(OPTION_SCALING_CUSTOM_STRETCH_NEAREST) === 'true'; }
    setCustomStretchNearest (stretch) { this.localStorage.setItem(OPTION_SCALING_CUSTOM_STRETCH_NEAREST, stretch ? 'true' : 'false'); }
    getAntiAliasing () { return this.localStorage.getItem(OPTION_ANTIALIAS) !== 'false'; }
    setAntiAliasing (antiAliasing) { this.localStorage.setItem(OPTION_ANTIALIAS, antiAliasing ? 'true' : 'false'); }
    removeAllOptions () {
        this.localStorage.removeItem(OPTION_SCALING_SELECT);
        this.localStorage.removeItem(OPTION_POWER_PREFERENCE_SELECT);
        this.localStorage.removeItem(OPTION_SCALING_CUSTOM_RES_WIDTH);
        this.localStorage.removeItem(OPTION_SCALING_CUSTOM_RES_HEIGHT);
        this.localStorage.removeItem(OPTION_SCALING_CUSTOM_AR_X);
        this.localStorage.removeItem(OPTION_SCALING_CUSTOM_AR_Y);
        this.localStorage.removeItem(OPTION_SCALING_CUSTOM_STRETCH_NEAREST);
        this.localStorage.removeItem(OPTION_ANTIALIAS);
    }
}