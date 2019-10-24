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

import Constants from '../services/constants';
import Logger from '../services/logger';

const getTopMessageDeo = () => document.getElementById(Constants.TOP_MESSAGE_ID);

window.addEventListener(Constants.APP_EVENT_TOP_MESSAGE, event => {
    const existingTopMessage = getTopMessageDeo();
    if (existingTopMessage) {
        existingTopMessage.remove();
    }
    const div = document.createElement('div');
    div.id = Constants.TOP_MESSAGE_ID;
    const span = document.createElement('span');
    span.innerHTML = event.detail;
    Logger.log('top_message: ' + event.detail);
    div.appendChild(span);
    document.body.appendChild(div);
    let opacity = 0.75;
    div.style.opacity = opacity;
    setTimeout(() => {
        function fade () {
            if (opacity >= 0.01) {
                opacity -= 0.01;
                div.style.opacity = opacity;
                setTimeout(fade, 16);
            } else {
                div.remove();
            }
        }
        fade();
    }, event.detail.length * 100);
}, false);
