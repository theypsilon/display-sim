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
    .classList.add(Constants.PRESET_ACTIVE_CLASS);

Constants.filterPresetsButtonDeoList.forEach(deo => {
    deo.onclick = function () {
        const preset = deo.dataset.preset;
        Constants.filterPresetsButtonDeoList.forEach(otherDeo => {
            otherDeo.classList.remove(Constants.PRESET_ACTIVE_CLASS);
        });
        deo.classList.add(Constants.PRESET_ACTIVE_CLASS);
        if (preset !== Constants.PRESET_KIND_CUSTOM) {
            storage.setFilterPresets(preset);
        }
        window.dispatchEvent(new CustomEvent(Constants.APP_EVENT_CUSTOM_INPUT, {
            detail: {
                value: preset,
                kind: 'event_kind:filter_presets_selected'
            }
        }));

        if (preset === Constants.PRESET_KIND_CUSTOM) {
            Array.from(Constants.tabsSelector)
                .filter(deo => deo.id === Constants.TAB_PANEL_ADVANCED)[0]
                .click();
        }
    };
});

window.addEventListener(Constants.APP_EVENT_PRESET_SELECTED_NAME, event => {
    Constants.filterPresetsButtonDeoList
        .forEach(deo => {
            if (deo.dataset.preset === event.detail) {
                deo.classList.add(Constants.PRESET_ACTIVE_CLASS);
            } else if (deo.classList.contains(Constants.PRESET_ACTIVE_CLASS)) {
                deo.classList.remove(Constants.PRESET_ACTIVE_CLASS);
            }
        });
    if (event.detail === Constants.PRESET_KIND_CUSTOM) {
        window.dispatchEvent(new CustomEvent(Constants.APP_EVENT_TOP_MESSAGE, {
            detail: 'Now you are in the Custom mode, you may change any filter value you want.'
        }));
    }
});
