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

import Constants from '../constants';

import { Visibility } from '../visibility';
import { Storage } from '../storage';

const visibility = Visibility.make();
const storage = Storage.make();

export function loadInputValuesFromStorage () {
    Constants.optionScalingSelect.value = storage.getScalingSelectOption();
    Constants.optionPowerPreferenceSelect.value = storage.getPowerPreferenceSelectOption();
    if (Constants.optionScalingSelect.value === Constants.scalingCustomHtmlId) {
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
