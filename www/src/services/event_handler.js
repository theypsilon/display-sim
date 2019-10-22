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
        this._dict = {};
    }
    static make () { return instance; }
    subscribeId (type, id, cb) {
        const registry = this._getTypeRegistry(type);
        registry.cbById[id] = cb;
    }
    subscribeClass (type, klass, cb) {
        const registry = this._getTypeRegistry(type);
        registry.cbByClass[klass] = cb;
        registry.hasClass = true;
    }
    remove (type, match) {
        if (!this._dict[type]) return;
        if (this._dict[type].cbByClass[match]) {
            delete this._dict[type].cbByClass[match];
        }
        if (this._dict[type].cbById[match]) {
            delete this._dict[type].cbById[match];
            this._dict[type].hasClass = Object.keys(this._dict[type].cbById).length > 0;
        }
    }
    _getTypeRegistry (type) {
        if (!this._dict[type]) {
            const typeRegistry = { cbByClass: {}, cbById: {}, hasClass: false };
            window.addEventListener(type, event => {
                this._runIfNotNull(typeRegistry.cbById[event.target.id], event);
                if (typeRegistry.hasClass === false) return;
                event.target.classList.forEach(klass => this._runIfNotNull(typeRegistry.cbByClass[klass], event));
            });
            this._dict[type] = typeRegistry;
        }
        return this._dict[type];
    }
    _runIfNotNull (cb, event) {
        if (cb) {
            cb(event);
        }
    }
}

instance = new EventHandler();