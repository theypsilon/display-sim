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

import {Constants} from '../../services/constants';
import {Logger} from '../../services/logger';
import {Mailbox} from '../../services/mailbox';
import {LocalStorage} from '../../services/local_storage';
import {SimWasmBackend} from './sim_wasm_backend';
import {throwOnNull} from "../../services/guards";

const STORE_KEY_WEBGL_POWER_PREFERENCE = 'option-powerPreference';
const STORE_KEY_WEBGL_ANTIALIAS = 'option-antialias';
const POWER_PREFERENCE_DEFAULT = 'default';
const FILTERS_PRESET_STORE_KEY = 'FiltersPreset';

export class SimModel {
    private readonly _eventBus: any;
    private readonly _mailbox: Mailbox;
    private readonly _wasmBackend: SimWasmBackend;
    private readonly _store: LocalStorage;
    private readonly _state: any;

    constructor (canvas: HTMLCanvasElement, eventBus: any, mailbox: Mailbox, wasmBackend: SimWasmBackend, store: LocalStorage) {
        this._eventBus = eventBus;
        this._mailbox = mailbox;
        this._wasmBackend = wasmBackend;
        this._store = store;
        this._state = {
            canvas,
            msg: null,
            loaded: false,
            storedValues: {
                selectedPreset: this._store.getItem(FILTERS_PRESET_STORE_KEY) || Constants.PRESET_KIND_APERTURE_GRILLE_1,
                powerPreference: this._store.getItem(STORE_KEY_WEBGL_POWER_PREFERENCE) || POWER_PREFERENCE_DEFAULT,
                antialias: this._store.getItem(STORE_KEY_WEBGL_ANTIALIAS) !== 'false'
            }
        };
    }

    static make (canvas: HTMLCanvasElement, eventBus: any, mailbox?: Mailbox, wasmBackend?: SimWasmBackend, store?: any) {
        return new SimModel(
            canvas,
            eventBus,
            mailbox || Mailbox.getInstance(),
            wasmBackend || SimWasmBackend.getInstance(),
            store || LocalStorage.make('sim-page')
        );
    }

    load () {
        const messages = this._mailbox.consumeMessages('sim-page');
        if (messages.length !== 1) {
            throw new Error('Can not handle these messages. ' + messages.toString());
        }

        this._state.msg = messages.filter((msg: any) => msg.topic === 'load-app')[0];
        return this._launchSimulation();
    }

    async _launchSimulation () {
        await new Promise(resolve => window.requestAnimationFrame(resolve));
        this.resizeCanvas();
        this._state.loaded = true;

        const result = await this._wasmBackend.load(this._state.canvas, this._eventBus, Object.assign({
            ctxOptions: {
                alpha: false,
                antialias: this._state.storedValues.antialias,
                depth: true,
                failIfMajorPerformanceCaveat: false,
                powerPreference: this._state.storedValues.powerPreference,
                premultipliedAlpha: false,
                preserveDrawingBuffer: false,
                stencil: false
            }
        }, this._state.msg.loadAppParams));
        return Object.assign({ storedValues: this._state.storedValues }, this._state.msg, result);
    }

    runFrame () {
        if (!this._state.loaded) {
            return false;
        }
        if (this._wasmBackend.runFrame()) {
            return true;
        } else {
            this.unloadSimulation();
            return false;
        }
    }

    setPreset (preset: string) {
        if (preset !== Constants.PRESET_KIND_CUSTOM) {
            this._state.storedValues.selectedPreset = preset;
            this._saveStoredValues();
        }
    }

    _saveStoredValues () {
        this._store.setItem(FILTERS_PRESET_STORE_KEY, this._state.storedValues.selectedPreset);
        this._store.setItem(STORE_KEY_WEBGL_POWER_PREFERENCE, this._state.storedValues.powerPreference);
        this._store.setItem(STORE_KEY_WEBGL_ANTIALIAS, this._state.storedValues.antialias ? 'true' : 'false');
    }

    async changePerformance (performance: string, direction: string): Promise<string> {
        const options = [POWER_PREFERENCE_DEFAULT, 'high-performance', 'low-power'];
        let index = options.indexOf(performance);
        switch (direction) {
            case 'inc': index = index + 1; break;
            case 'dec': index = index - 1; break;
            default: throw new Error('Unreachable!');
        }
        const newPerformance = options[index % options.length];
        this._state.storedValues.performance = newPerformance;
        this._saveStoredValues();
        await this._reloadSimulation();
        return newPerformance;
    }

    async changeAntialiasing (currentAntialias: boolean) {
        this._state.storedValues.antialias = !currentAntialias;
        this._saveStoredValues();
        await this._reloadSimulation();
    }

    unloadSimulation () {
        this._state.loaded = false;
        this._wasmBackend.unload();
        const newCanvas = document.createElement('canvas');
        newCanvas.setAttribute('tabindex', '0');
        this._state.canvas.parentNode.replaceChild(newCanvas, this._state.canvas);
        this._state.canvas.remove();
        this._state.canvas = newCanvas;
    }

    async _reloadSimulation () {
        this.unloadSimulation();
        await new Promise(resolve => setTimeout(resolve, 1000));
        return this._launchSimulation();
    }

    resizeCanvas () {
        const dpi = window.devicePixelRatio;
        const canvas = this._state.canvas;
        const width = canvas.width = canvas.offsetWidth * dpi;
        const height = canvas.height = canvas.offsetHeight * dpi;    
        return { width, height };
    }

    async fireScreenshot ({ buffer, width, height }: { buffer: ArrayLike<number>, width: number, height: number}) {
        Logger.log('starting screenshot');
        Logger.log('width', width, 'height', height);

        const canvas = document.createElement('canvas');
        canvas.width = width;
        canvas.height = height;
        const ctx = throwOnNull(canvas.getContext('2d'));
    
        const imageData = ctx.createImageData(width, height);
        imageData.data.set(buffer);
        ctx.putImageData(imageData, 0, 0);
        ctx.globalCompositeOperation = 'copy';
        ctx.scale(1, -1); // Y flip
        ctx.translate(0, -imageData.height);
        ctx.drawImage(canvas, 0, 0);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.globalCompositeOperation = 'source-over';
    
        const a = document.createElement('a');
        document.body.appendChild(a);
        a.classList.add('no-display');
        const blob = await new Promise(resolve => canvas.toBlob(resolve));
        const url = URL.createObjectURL(blob as Blob);
        a.href = url;
        a.download = 'Display-Sim_' + new Date().toISOString() + '.png';
        a.click();
    
        await new Promise(resolve => setTimeout(resolve, 3000));
        URL.revokeObjectURL(url);
        a.remove();
    }
}