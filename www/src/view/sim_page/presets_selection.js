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

const storage = Storage.make();

const preset = storage.getFilterPresets();
Constants.filterPresetsButtonDeoList
    .filter(deo => deo.dataset.preset === preset)[0]
    .classList.add('active-preset');

Constants.filterPresetsButtonDeoList.forEach(deo => {
    deo.onclick = function () {
        const preset = deo.dataset.preset;
        Constants.filterPresetsButtonDeoList.forEach(otherDeo => {
            otherDeo.classList.remove('active-preset');
        });
        deo.classList.add('active-preset');
        if (preset !== 'custom') {
            storage.setFilterPresets(preset);
        }
        window.dispatchEvent(new CustomEvent('app-event.custom_input_event', {
            detail: {
                value: preset,
                kind: 'event_kind:filter_presets_selected'
            }
        }));

        if (preset === Constants.presetCustom) {
            Array.from(document.querySelectorAll('.tabs > li'))
                .filter(deo => deo.id === 'panel-advanced')[0]
                .click();
        }
    };
});