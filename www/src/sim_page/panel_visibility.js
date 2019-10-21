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

const getGlCanvasDeo = () => document.getElementById(Constants.glCanvasHtmlId);

const visibility = Visibility.make();

window.addEventListener('app-event.toggle_info_panel', () => {
    if (!getGlCanvasDeo()) {
        return;
    }
    if (visibility.isInfoPanelVisible() === false) {
        visibility.showInfoPanel();
    } else {
        visibility.hideInfoPanel();
        window.dispatchEvent(new CustomEvent('app-event.top_message', {
            detail: 'Toggle the Sim Panel by pressing SPACE.'
        }));
    }
}, false);

Constants.toggleInfoPanelClass.forEach(deo => {
    deo.onclick = () => {
        if (!getGlCanvasDeo()) {
            return;
        }
        if (visibility.isInfoPanelVisible()) {
            visibility.hideInfoPanel();
            window.dispatchEvent(new CustomEvent('app-event.top_message', {
                detail: 'Show the Sim Panel again by pressing SPACE.'
            }));
        } else {
            visibility.showInfoPanel();
        }
    };
});
