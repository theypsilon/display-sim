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

import Globals from '../globals';

import { makeVisibility } from '../visibility';
import { makeStorage } from '../storage';

const visibility = makeVisibility();
const storage = makeStorage();

export function loadInputValuesFromStorage () {
    Globals.optionScalingSelect.value = storage.getScalingSelectOption();
    Globals.optionPowerPreferenceSelect.value = storage.getPowerPreferenceSelectOption();
    if (Globals.optionScalingSelect.value === Globals.scalingCustomHtmlId) {
        visibility.showScaleCustomInputs();
    } else {
        visibility.hideScaleCustomInputs();
    }
    Globals.scalingCustomResWidthDeo.value = storage.getCustomResWidth();
    Globals.scalingCustomResHeightDeo.value = storage.getCustomResHeight();
    Globals.scalingCustomArXDeo.value = storage.getCustomArX();
    Globals.scalingCustomArYDeo.value = storage.getCustomArY();
    Globals.scalingCustomStretchNearestDeo.checked = storage.getCustomStretchNearest();
    Globals.antialiasDeo.checked = storage.getAntiAliasing();
}
