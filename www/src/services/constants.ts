/* Copyright (c) 2019-2021 José manuel Barroso Galindo <theypsilon@gmail.com>
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

function throwOnNull<T>(value: T | null): T {
    if (value === null) {
        throw new Error('Can not be null!');
    }
    return value;
}

export const Constants = {
    // general
    TOP_MESSAGE_ID: 'top-message',
    loadingDeo: throwOnNull(document.getElementById('loading')),
    pageDeo: throwOnNull(document.getElementById('page')),

    // landing page
    FIRST_PREVIEW_IMAGE_ID: 'first-preview-image',

    // sim page
    FILTER_PRESETS_SELECTED_EVENT_KIND: 'filter-presets-selected',
    PRESET_KIND_CUSTOM: 'custom',
    PRESET_KIND_APERTURE_GRILLE_1: 'crt-aperture-grille-1'
};
