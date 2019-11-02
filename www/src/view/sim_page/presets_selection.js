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

export default function (ctx) {

    const storage = Storage.make();

    const preset = storage.getFilterPresets();
    ctx.constants.filterPresetsButtonDeoList
        .filter(deo => deo.dataset.preset === preset)[0]
        .classList.add(ctx.constants.PRESET_ACTIVE_CLASS);

    ctx.constants.filterPresetsButtonDeoList.forEach(deo => {
        deo.onclick = function () {
            const preset = deo.dataset.preset;
            ctx.constants.filterPresetsButtonDeoList.forEach(otherDeo => {
                otherDeo.classList.remove(ctx.constants.PRESET_ACTIVE_CLASS);
            });
            deo.classList.add(ctx.constants.PRESET_ACTIVE_CLASS);
            if (preset !== ctx.constants.PRESET_KIND_CUSTOM) {
                storage.setFilterPresets(preset);
            }
            window.dispatchEvent(new CustomEvent(ctx.constants.APP_EVENT_CUSTOM_INPUT, {
                detail: {
                    value: preset,
                    kind: 'event_kind:filter_presets_selected'
                }
            }));

            if (preset === ctx.constants.PRESET_KIND_CUSTOM) {
                Array.from(ctx.constants.tabsSelector)
                    .filter(deo => deo.id === ctx.constants.TAB_PANEL_ADVANCED)[0]
                    .click();
            }
        };
    });

    window.addEventListener(ctx.constants.APP_EVENT_PRESET_SELECTED_NAME, event => {
        ctx.constants.filterPresetsButtonDeoList
            .forEach(deo => {
                if (deo.dataset.preset === event.detail) {
                    deo.classList.add(ctx.constants.PRESET_ACTIVE_CLASS);
                } else if (deo.classList.contains(ctx.constants.PRESET_ACTIVE_CLASS)) {
                    deo.classList.remove(ctx.constants.PRESET_ACTIVE_CLASS);
                }
            });
        if (event.detail === ctx.constants.PRESET_KIND_CUSTOM) {
            window.dispatchEvent(new CustomEvent(ctx.constants.APP_EVENT_TOP_MESSAGE, {
                detail: 'Now you are in the Custom mode, you may change any filter value you want.'
            }));
        }
    });
}