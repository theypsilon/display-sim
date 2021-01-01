/* Copyright (c) 2019-2021 José manuel Barroso Galindo <theypsilon@gmail.com>
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

export class Lazy<T> {
    private _callback: (() => T) | null;
    private _value: T | null = null;

    constructor(callback: () => T) {
        this._callback = callback;
    }

    static from<T>(callback: () => T): Lazy<T> {
        return new Lazy(callback);
    }

    get(): T {
        if (this._value === null) {
            if (this._callback == null) {
                throw new Error("this._callback shouldn't be null");
            }
            this._value = this._callback();
            this._callback = null;
        }
        return this._value;
    }
}
