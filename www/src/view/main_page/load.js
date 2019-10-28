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

import FontFaceObserver from 'fontfaceobserver';

import Constants from '../../services/constants';

import { Visibility } from '../../services/visibility';
import { Storage } from '../../services/storage';

import { mobileAndTabletCheck } from '../../services/utils';

import { playHtmlSelection } from './play_html_selection';
import { playQuerystring } from './play_querystring';

const isRunningOnMobileDevice = mobileAndTabletCheck();
const visibility = Visibility.make();
const storage = Storage.make();

Promise.all([
    new FontFaceObserver('Archivo Black', { weight: 400 }).load(null, 10000),
    new FontFaceObserver('Lato', { weight: 400 }).load(null, 10000),
    new FontFaceObserver('Lato', { weight: 700 }).load(null, 10000)
]).then(prepareMainPage).catch(e => {
    console.error(e);
    prepareMainPage();
});

let savedHash = '';
let hashNotChanged = false;
window.onhashchange = () => {
    if (hashNotChanged) {
        hashNotChanged = false;
        return;
    }
    if (window.location.hash.length === 0) {
        hashNotChanged = true;
        window.location.hash = savedHash;
        return;
    }
    visibility.hideAll();
    visibility.showLoading();
    prepareMainPage();
};

export async function prepareMainPage () {
    if (window.location.hash.length > 1) {
        savedHash = window.location.hash;
        return playQuerystring(window.location.hash.substr(1));
    }

    loadInputValuesFromStorage();

    visibility.showUi();
    visibility.hideLoading();

    if (isRunningOnMobileDevice) {
        Constants.startAnimationDeo.disabled = true;
        Constants.startAnimationDeo.title = 'You need a PC with NVIDIA or ATI graphics card with updated drivers and a WebGL2 compatible browser (Firefox, Opera or Chrome) in order to run this without problems.';
        return;
    }

    await new Promise(resolve => {
        Constants.startAnimationDeo.onclick = resolve;
    });

    visibility.hideUi();

    const result = await playHtmlSelection();
    if (result.reloadPage) {
        prepareMainPage();
    }
}

export function loadInputValuesFromStorage () {
    Constants.optionScalingSelect.value = storage.getScalingSelectOption();
    Constants.optionPowerPreferenceSelect.value = storage.getPowerPreferenceSelectOption();
    if (Constants.optionScalingSelect.value === Constants.SCALING_CUSTOM_ID) {
        visibility.showScaleCustomInputs();
    } else {
        visibility.hideScaleCustomInputs();
    }
    Constants.scalingCustomResWidthDeo.value = storage.getCustomResWidth();
    Constants.scalingCustomResHeightDeo.value = storage.getCustomResHeight();
    Constants.scalingCustomArXDeo.value = storage.getCustomArX();
    Constants.scalingCustomArYDeo.value = storage.getCustomArY();
    Constants.scalingCustomStretchNearestDeo.checked = storage.getCustomStretchNearest();
    Constants.antialiasDeo.checked = storage.getAntiAliasing();
}