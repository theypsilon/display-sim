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

import { Constants } from './constants';
import { Logger } from './logger';
import { Visibility } from './visibility';
import { Lazy } from './lazy';

let visibility = Visibility.make();
const getTopMessageDeo = () => document.getElementById(Constants.TOP_MESSAGE_ID);

export class Navigator {
    private static _instance: Lazy<Navigator> = Lazy.from(() => new Navigator());
    static make (): Navigator { return this._instance.get(); }
    private constructor() {}

    goToLandingPage (): void {
        this._goToPageTagged('landing-page');
    }
    goToSimPage (): void {
        this._goToPageTagged('sim-page');
    }
    _goToPageTagged (tag: string): void {
        visibility.showLoading();
        setTimeout(() => {
            Constants.pageDeo.children[0].remove();
            const page = document.createElement(tag);
            Constants.pageDeo.appendChild(page);
        }, 0);
    }
    openTopMessage (msg: string): void {
        const existingTopMessage = getTopMessageDeo();
        if (existingTopMessage) {
            existingTopMessage.remove();
        }
        const div = document.createElement('div');
        div.id = Constants.TOP_MESSAGE_ID;
        const span = document.createElement('span');
        span.innerHTML = msg;
        Logger.log('top_message: ' + msg);
        div.appendChild(span);
        document.body.appendChild(div);
        let opacity = 0.75;
        div.style.opacity = opacity.toString();
        setTimeout(() => {
            function fade () {
                if (opacity >= 0.01) {
                    opacity -= 0.01;
                    div.style.opacity = opacity.toString();
                    setTimeout(fade, 16);
                } else {
                    div.remove();
                }
            }
            fade();
        }, msg.length * 100);
    }
}
