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

import { renderTemplate } from './landing_template';
import { playQuerystring } from './play_simulation';

import { model, View } from './landing_view_model';

import { Store } from './landing_store';

const store = Store.make();
const state = model(store);

class LandingPage extends HTMLElement {
    constructor () {
        super();

        if (window.location.hash.length > 1) {
            playQuerystring(window.location.hash.substr(1));
            return;
        }

        this._state = state;
        this._root = this.attachShadow({ mode: 'open' });
        this._view = View.make(state, this, store);

        this._view.makeItVisible();
    }

    refresh () {
        renderTemplate(state, this._view, this._root);
    }

    getRoot () {
        return this._root;
    }
}

window.customElements.define('landing-page', LandingPage);
