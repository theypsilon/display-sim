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

let instance;

export class Visibility {
    static make () { return instance; }
    showUi () { showElement(Constants.uiDeo); }
    hideUi () { hideElement(Constants.uiDeo); }
    showLoading () { showElement(Constants.loadingDeo); }
    hideLoading () { hideElement(Constants.loadingDeo); }
    showSimulationUi () {
        document.body.style.setProperty('overflow', 'hidden');
        document.body.style.setProperty('background-color', 'black');
        showElement(Constants.simulationUiDeo);
    }
    hideSimulationUi () {
        document.body.style.removeProperty('overflow');
        document.body.style.removeProperty('background-color');
        hideElement(Constants.simulationUiDeo);
    }
    showInfoPanel () { showElement(Constants.infoPanelDeo); }
    hideInfoPanel () { hideElement(Constants.infoPanelDeo); }
    isInfoPanelVisible () { isVisible(Constants.infoPanelDeo); }
    showScalingCustomResButton () { showElement(Constants.scalingCustomResButtonDeo); }
    showScaleCustomInputs () { showElement(Constants.scalingCustomInputsDeo); }
    hideScaleCustomInputs () { hideElement(Constants.scalingCustomInputsDeo); }
    hideAll () {
        this.hideUi();
        this.hideLoading();
        this.hideSimulationUi();
        this.hideInfoPanel();
        this.hideScaleCustomInputs();
    }
}

instance = new Visibility();

function showElement (element) {
    element.classList.remove(Constants.DISPLAY_NONE_CLASS);
}
function hideElement (element) {
    element.classList.add(Constants.DISPLAY_NONE_CLASS);
}
function isVisible (element) {
    return element.classList.contains(Constants.DISPLAY_NONE_CLASS) === false;
}
