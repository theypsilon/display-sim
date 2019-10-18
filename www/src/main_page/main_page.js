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

import Constants from '../constants';

import { Visibility } from '../visibility';
import { mobileAndTabletCheck } from '../mobile_check';

import { playSimulation } from './play_simulation';
import { playDemo } from './play_demo';
import { loadInputValuesFromStorage } from './common';

import './select_image';
import './options';

const isRunningOnMobileDevice = mobileAndTabletCheck();
const visibility = Visibility.make();

Promise.all([
    new FontFaceObserver('Archivo Black', { weight: 400 }).load(null, 10000),
    new FontFaceObserver('Lato', { weight: 400 }).load(null, 10000),
    new FontFaceObserver('Lato', { weight: 700 }).load(null, 10000)
]).then(prepareMainPage).catch(e => {
    console.error(e);
    prepareMainPage();
});

export async function prepareMainPage () {
    if (window.location.hash.length > 1) {
        return playDemo(window.location.hash.substr(1));
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

    const result = await playSimulation();
    if (result.reloadPage) {
        prepareMainPage();
    }
}
