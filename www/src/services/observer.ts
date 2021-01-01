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

export class Observer<T> {
    private _callbacks: ((arg: T) => {})[] = [];

    static make<T>(): Observer<T> {
        return new Observer<T>();
    }

    subscribe (cb: (arg: T) => {}): void {
        this._callbacks.push(cb);
    }

    unsubscribe (unsubscribedCb: (arg: T) => {}): void {
        this._callbacks = this._callbacks.filter(cb => cb !== unsubscribedCb);
    }

    async fire (event: T): Promise<void> {
        for (const cb of this._callbacks) {
            await cb(event);
        }
    }
}
