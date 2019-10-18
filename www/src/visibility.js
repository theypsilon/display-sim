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

import Constants from './Constants';

export function makeVisibility () {
    return {
        showUi: () => showElement(Constants.uiDeo),
        hideUi: () => hideElement(Constants.uiDeo),
        showLoading: () => showElement(Constants.loadingDeo),
        hideLoading: () => hideElement(Constants.loadingDeo),
        showSimulationUi: () => {
            document.body.style.setProperty('overflow', 'hidden');
            document.body.style.setProperty('background-color', 'black');
            showElement(Constants.simulationUiDeo);
        },
        hideSimulationUi: () => {
            document.body.style.removeProperty('overflow');
            document.body.style.removeProperty('background-color');
            hideElement(Constants.simulationUiDeo);
        },
        showInfoPanel: () => showElement(Constants.infoPanelDeo),
        hideInfoPanel: () => hideElement(Constants.infoPanelDeo),
        isInfoPanelVisible: () => isVisible(Constants.infoPanelDeo),
        showFilterOptionMainList: () => showElement(Constants.filterOptionMainListDeo),
        hideFilterOptionMainList: () => hideElement(Constants.filterOptionMainListDeo),
        showScalingCustomResButton: () => showElement(Constants.scalingCustomResButtonDeo),
        showScaleCustomInputs: () => showElement(Constants.scalingCustomInputsDeo),
        hideScaleCustomInputs: () => hideElement(Constants.scalingCustomInputsDeo)
    };
    function showElement (element) {
        element.classList.remove(Constants.displayNoneClassName);
    }
    function hideElement (element) {
        element.classList.add(Constants.displayNoneClassName);
    }
    function isVisible (element) {
        return element.classList.contains(Constants.displayNoneClassName) === false;
    }
}
