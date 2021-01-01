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
import { Lazy } from './lazy';

const DISPLAY_NONE_CLASS = 'display-none';

export class Visibility {
    private static _instance: Lazy<Visibility> = Lazy.from(() => new Visibility());
    static make (): Visibility { return this._instance.get(); }
    private constructor() {}
    showLoading (): void { showElement(Constants.loadingDeo); }
    hideLoading (): void { hideElement(Constants.loadingDeo); }
    showDeo (deo: Element): void { showElement(deo); }
    hideDeo (deo: Element): void { hideElement(deo); }
    canSee (deo: Element): boolean { return isVisible(deo); }
}

function showElement (element: Element): void {
    element.classList.remove(DISPLAY_NONE_CLASS);
}
function hideElement (element: Element): void {
    element.classList.add(DISPLAY_NONE_CLASS);
}
function isVisible (element: Element): boolean {
    return !element.classList.contains(DISPLAY_NONE_CLASS);
}
