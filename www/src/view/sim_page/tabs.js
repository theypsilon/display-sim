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

let selectedInfoPanelDeo = Constants.infoPanelBasicDeo;

Constants.tabsSelector.forEach(clickedTab => {
    clickedTab.addEventListener('click', () => {
        Constants.tabsSelector.forEach(tab => {
            tab.classList.remove('active');
        });
        clickedTab.classList.add('active');
        selectedInfoPanelDeo.classList.add(Constants.DISPLAY_NONE_CLASS);
        switch (clickedTab.id) {
        case Constants.TAB_PANEL_BASIC:
            selectedInfoPanelDeo = Constants.infoPanelBasicDeo;
            break;
        case Constants.TAB_PANEL_ADVANCED:
            selectedInfoPanelDeo = Constants.infoPanelAdvancedDeo;
            break;
        default:
            console.error('Unknown clicked tab: ' + clickedTab.id);
            break;
        }
        selectedInfoPanelDeo.classList.remove(Constants.DISPLAY_NONE_CLASS);
    });
});
