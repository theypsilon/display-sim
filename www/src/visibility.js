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

import Globals from './globals';

export function makeVisibility () {
    return {
        showUi: () => showElement(Globals.uiDeo),
        hideUi: () => hideElement(Globals.uiDeo),
        showLoading: () => showElement(Globals.loadingDeo),
        hideLoading: () => hideElement(Globals.loadingDeo),
        showSimulationUi: () => {
            document.body.style.setProperty('overflow', 'hidden');
            document.body.style.setProperty('background-color', 'black');
            showElement(Globals.simulationUiDeo);
        },
        hideSimulationUi: () => {
            document.body.style.removeProperty('overflow');
            document.body.style.removeProperty('background-color');
            hideElement(Globals.simulationUiDeo);
        },
        showInfoPanel: () => showElement(Globals.infoPanelDeo),
        hideInfoPanel: () => hideElement(Globals.infoPanelDeo),
        isInfoPanelVisible: () => isVisible(Globals.infoPanelDeo),
        showFilterOptionMainList: () => showElement(Globals.filterOptionMainListDeo),
        hideFilterOptionMainList: () => hideElement(Globals.filterOptionMainListDeo),
        showScalingCustomResButton: () => showElement(Globals.scalingCustomResButtonDeo),
        showScaleCustomInputs: () => showElement(Globals.scalingCustomInputsDeo),
        hideScaleCustomInputs: () => hideElement(Globals.scalingCustomInputsDeo)
    };
    function showElement (element) {
        element.classList.remove(Globals.displayNoneClassName);
    }
    function hideElement (element) {
        element.classList.add(Globals.displayNoneClassName);
    }
    function isVisible (element) {
        return element.classList.contains(Globals.displayNoneClassName) === false;
    }
}
