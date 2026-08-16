/* Copyright (c) 2019-2024 José manuel Barroso Galindo <theypsilon@gmail.com>
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

export type FileEvent = Event & {target: EventTarget & { files: FileList }};
export type DataEvent = Event & {dataTransfer: DataTransfer};
export type KeyboardEvent = Event & {
    code: string, key: string, location: number, repeat: boolean, isComposing: boolean,
    altKey: boolean, ctrlKey: boolean, shiftKey: boolean, metaKey: boolean
};
export type MouseMovementEvent = Event & {movementX: number, movementY: number};
export type MouseDownEvent = Event & {
    button: number, buttons: number, clientX: number, clientY: number,
    altKey: boolean, ctrlKey: boolean, shiftKey: boolean, metaKey: boolean
};
export type MouseWheelEvent = Event & {
    deltaX: number, deltaY: number, deltaMode: number,
    altKey: boolean, ctrlKey: boolean, shiftKey: boolean, metaKey: boolean
};
export type TextEvent = Event & {data: string | null, inputType: string};
export type ClipboardEvent = Event & {clipboardData: DataTransfer | null};
export type BackendEvent = ((e: KeyboardEvent & MouseMovementEvent & MouseWheelEvent & MouseDownEvent & TextEvent & ClipboardEvent) => void | Promise<void>);
