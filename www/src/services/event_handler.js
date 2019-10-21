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

let instance;

export class EventHandler {
    constructor () {
        this.dict = {};
    }
    static make () { return instance; }
    listenMatch (type, match, cb) {
        if (!this.dict[type]) {
            const typedict = { matchdict: {}, iddict: {} };
            this.dict[type] = typedict;
            document.addEventListener(type, event => {
                Object.keys(this.dict[type].matchdict).forEach(match => {
                    if (event.target.matches(match)) {
                        typedict.matchdict[match]();
                    }
                });
            });
        }
        this.dict[type].matchdict[match] = cb;
    }
    listen (type, id, cb) {
        if (!this.dict[type]) {
            const typedict = { matchdict: {}, iddict: {} };
            this.dict[type] = typedict;
            document.addEventListener(type, event => {
                const callback = this.dict[type].iddict[event.target.id];
                if (callback) {
                    callback();
                }
            });
        }
        this.dict[type].iddict[id] = cb;
    }
    remove (type, match) {
        if (!this.dict[type]) return;
        if (this.dict[type].matchdict[match]) {
            delete this.dict[type].matchdict[match];
        }
        if (this.dict[type].iddict[match]) {
            delete this.dict[type].iddict[match];
        }
    }
}

instance = new EventHandler();