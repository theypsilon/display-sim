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

export class LandingVisibility {
    constructor (elements, visibility) {
        this.elements = elements;
        this.visibility = visibility;
    }
    static make(elements) {
        return new LandingVisibility(elements, Visibility.make());
    }
    showUi () { this.visibility.showDeo(this.elements.uiDeo); }
    hideUi () { this.visibility.hideDeo(this.elements.uiDeo); }
    showScalingCustomResButton () { this.visibility.showDeo(this.elements.scalingCustomResButtonDeo); }
    showScaleCustomInputs () { this.visibility.showDeo(this.elements.scalingCustomInputsDeo); }
    hideScaleCustomInputs () { this.visibility.hideDeo(this.elements.scalingCustomInputsDeo); }
    showLoading () { this.visibility.showLoading() }
    hideLoading () { this.visibility.hideLoading() }
}