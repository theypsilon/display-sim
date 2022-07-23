/* Copyright (c) 2019-2022 José manuel Barroso Galindo <theypsilon@gmail.com>
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

export class LocalStorage {
    private readonly _root: any;
    private readonly _prefix: string;

    constructor (prefix: string, root: any) {
        this._root = root;
        this._prefix = prefix;
    }
    static make (prefix: string, root?: any): LocalStorage {
        return new LocalStorage(prefix, root || window);
    }
    getItem (key: string): string | null {
        return localStorage.getItem('DISPLAY_SIM.' + this._prefix + '.' + key);
    }
    setItem (key: string, value: any): void {
        localStorage.setItem('DISPLAY_SIM.' + this._prefix + '.' + key, value);
    }
    removeItem (key: string): void {
        localStorage.removeItem('DISPLAY_SIM.' + this._prefix + '.' + key);
    }
}
