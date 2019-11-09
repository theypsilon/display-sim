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

import Constants from './constants';
import { Visibility } from './visibility';

let instance;
let visibility = Visibility.make();

export class Navigator {
    static make () { return instance; }
    goToLandingPage () {
        this._goToPageTagged('landing-page');
    }
    goToSimPage () {
        this._goToPageTagged('sim-page');
    }
    _goToPageTagged (tag) {
        visibility.showLoading();
        setTimeout(() => {
            Constants.pageDeo.children[0].remove();
            const page = document.createElement(tag);
            Constants.pageDeo.appendChild(page);
        }, 0);
    }
    openTopMessage (text) {
        window.dispatchEvent(new CustomEvent(Constants.APP_EVENT_TOP_MESSAGE, {
            detail: text
        }));
    }
}

instance = new Navigator();
