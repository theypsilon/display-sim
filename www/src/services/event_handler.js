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
    listenClass (type, klass, cb) {
        const registry = this._getTypeRegistry(type);
        registry.cbByClass[klass] = cb;
    }
    listenId (type, id, cb) {
        const registry = this._getTypeRegistry(type);
        registry.cbById[id] = cb;
    }
    remove (type, match) {
        if (!this.dict[type]) return;
        if (this.dict[type].cbByClass[match]) {
            delete this.dict[type].cbByClass[match];
        }
        if (this.dict[type].cbById[match]) {
            delete this.dict[type].cbById[match];
        }
    }
    _getTypeRegistry (type) {
        if (!this.dict[type]) {
            const typeRegistry = { cbByClass: {}, cbById: {} };
            window.addEventListener(type, event => {
                this._runIfNotNull(typeRegistry.cbById[event.target.id]);
                event.target.classList.forEach(klass => this._runIfNotNull(typeRegistry.cbByClass[klass]));
            });
            this.dict[type] = typeRegistry;
        }
        return this.dict[type];
    }
    _runIfNotNull (cb) {
        if (cb) {
            cb();
        }
    }
}

instance = new EventHandler();