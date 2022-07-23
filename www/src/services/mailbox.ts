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

import { Lazy } from './lazy';

export class Mailbox {
    private readonly _dict: { [id: string]: any; } = {};

    private static _instance: Lazy<Mailbox> = Lazy.from(() => new Mailbox());
    static getInstance (): Mailbox { return this._instance.get(); }
    private constructor() {}

    placeMessage (address: string, content: any): void {
        if (!this._dict[address]) {
            this._dict[address] = [];
        }
        this._dict[address].push(content);
    }
    consumeMessages (address: string): any {
        const inbox = this._dict[address];
        this._dict[address] = [];
        return inbox;
    }
}