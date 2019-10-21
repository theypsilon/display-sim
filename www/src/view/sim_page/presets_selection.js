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
import { Storage } from '../../services/storage';
import { Visibility } from '../../services/visibility';

const storage = Storage.make();
const visibility = Visibility.make();

Constants.filterPresetsButtonDeoList.forEach(deo => {
    deo.onclick = function () {
        Constants.filterPresetsButtonDeoList.forEach(otherDeo => {
            otherDeo.classList.remove('active-preset');
        });
        deo.classList.add('active-preset');
        window.dispatchEvent(new CustomEvent('app-event.custom_input_event', {
            detail: {
                value: deo.dataset.preset,
                kind: 'event_kind:filter_presets_selected'
            }
        }));
    };
});

const presetsDeoAvailable = [Constants.filterPresetsDeo];
window.addEventListener('app-event.preset_selected_name', event => {
    const presetValue = event.detail.toLowerCase().replace(/\s/g, '-');
    if (!Constants.properPresets.includes(presetValue)) {
        throw new Error('Wrong preset value: ' + presetValue);
    }
    presetsDeoAvailable.forEach(presetsDeo => {
        presetsDeo.value = presetValue;
    });
    Constants.filterPresetsButtonDeoList.forEach(deo => {
        if (deo.dataset.preset === presetValue) {
            deo.classList.add('active-preset');
        } else {
            deo.classList.remove('active-preset');
        }
    });
}, false);

presetsDeoAvailable.forEach(presetsDeo => {
    presetsDeo.onchange = () => {
        if (presetsDeo.value === Constants.presetCustom) {
            visibility.showFilterOptionMainList();
        } else if (Constants.properPresets.includes(presetsDeo.value)) {
            visibility.showFilterOptionMainList();
        } else {
            presetsDeo.value = Constants.presetApertureGrille1;
        }
        storage.setFilterPresets(presetsDeo.value);
        window.dispatchEvent(new CustomEvent('app-event.custom_input_event', {
            detail: {
                value: presetsDeo.value,
                kind: 'event_kind:filter_presets_selected'
            }
        }));
    };
});
