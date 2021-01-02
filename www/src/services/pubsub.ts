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

import {Disposable} from "./disposable";
import {Observable, ObserverCb} from "./observable";
import {Action} from "./action";

export class PubSub<T> implements Observable<T>, Action<T> {
    private _callbacks: ObserverCb<T>[] = [];

    static make<T>(): PubSub<T> {
        return new PubSub<T>();
    }

    subscribe (cb: ObserverCb<T>): Disposable {
        this._callbacks.push(cb);
        return Disposable.make(() => this.unsubscribe(cb));
    }

    async fire (event: T): Promise<void> {
        for (const cb of this._callbacks) {
            await cb(event);
        }
    }

    private unsubscribe (unsubscribedCb: ObserverCb<T>): void {
        this._callbacks = this._callbacks.filter(cb => cb !== unsubscribedCb);
    }
}
