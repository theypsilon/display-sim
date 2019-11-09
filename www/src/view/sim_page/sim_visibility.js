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

import { Visibility } from '../../services/visibility';

export class SimVisibility {
    constructor (elements, visibility) {
        this.elements = elements;
        this.visibility = visibility;
    }
    static make(elements) {
        return new SimVisibility(elements, Visibility.make());
    }
    showSimulationUi () {
        document.body.style.setProperty('overflow', 'hidden');
        document.body.style.setProperty('background-color', 'black');
        this.visibility.showDeo(this.elements.simulationUiDeo);
    }
    hideSimulationUi () {
        document.body.style.removeProperty('overflow');
        document.body.style.removeProperty('background-color');
        this.visibility.hideDeo(this.elements.simulationUiDeo);
    }
    showInfoPanel () { this.visibility.showDeo(this.elements.infoPanelDeo); }
    hideInfoPanel () { this.visibility.hideDeo(this.elements.infoPanelDeo); }
    showFreeModeCameraControls () {
        this.elements.featureCameraMovementsDeo.classList.remove('arrows-grid-move-free');
        this.elements.featureCameraMovementsDeo.classList.add('arrows-grid-move-lock');
        this.elements.freeModeControlsClas.forEach(deo => this.visibility.hideDeo(deo));
    }
    hideFreeModeCameraControls () {
        this.elements.featureCameraMovementsDeo.classList.remove('arrows-grid-move-lock');
        this.elements.featureCameraMovementsDeo.classList.add('arrows-grid-move-free');
        this.elements.freeModeControlsClas.forEach(deo => this.visibility.showDeo(deo));
    }
    showLoading () { this.visibility.showLoading() }
    hideLoading () { this.visibility.hideLoading() }
}