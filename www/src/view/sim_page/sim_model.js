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

import Constants from '../../services/constants';
import Logger from '../../services/logger';
import { Mailbox } from '../../services/mailbox';
import { LocalStorage } from '../../services/local_storage';
import { WasmApp } from './sim_wasm_app';

const STORE_KEY_WEBGL_POWER_PREFERENCE = 'option-powerPreference';
const STORE_KEY_WEBGL_ANTIALIAS = 'option-antialias';
const POWER_PREFERENCE_DEFAULT = 'default';
const FILTERS_PRESET_STORE_KEY = 'FiltersPreset';

export class Model {
    constructor (canvas, eventBus, mailbox, wasmApp, store) {
        this.eventBus = eventBus;
        this.mailbox = mailbox;
        this.wasmApp = wasmApp;
        this.store = store;
        this.state = {
            canvas,
            msg: null,
            loaded: false,
            storedValues: {
                selectedPreset: this.store.getItem(FILTERS_PRESET_STORE_KEY) || Constants.PRESET_KIND_APERTURE_GRILLE_1,
                powerPreference: this.store.getItem(STORE_KEY_WEBGL_POWER_PREFERENCE) || POWER_PREFERENCE_DEFAULT,
                antialias: this.store.getItem(STORE_KEY_WEBGL_ANTIALIAS) !== 'false'
            }
        };
    }

    static make (canvas, eventBus, mailbox, wasmApp, store) {
        return new Model(canvas, eventBus, mailbox || Mailbox.getInstance(), wasmApp || WasmApp.getInstance(), store || LocalStorage.make('sim-page'));
    }

    load () {
        const messages = this.mailbox.consumeMessages('sim-page');
        if (messages.length !== 1) {
            throw new Error('Can not handle these messages.', messages);
        }

        this.state.msg = messages.filter(msg => msg.topic === 'load-app')[0];
        return this._launchSimulation();
    }

    async _launchSimulation () {
        await new Promise(resolve => window.requestAnimationFrame(resolve));
        this.resizeCanvas();
        this.state.loaded = true;
        const result = await this.wasmApp.load(this.state.canvas, this.eventBus, Object.assign({
            ctxOptions: {
                alpha: false,
                antialias: this.state.storedValues.antialias,
                depth: true,
                failIfMajorPerformanceCaveat: false,
                powerPreference: this.state.storedValues.powerPreference,
                premultipliedAlpha: false,
                preserveDrawingBuffer: false,
                stencil: false
            }
        }, this.state.msg.loadAppParams));
        return Object.assign({ storedValues: this.state.storedValues }, this.state.msg, result);
    }

    runFrame () {
        if (!this.state.loaded) {
            return false;
        }
        if (this.wasmApp.runFrame()) {
            return true;
        } else {
            this.unloadSimulation();
            return false;
        }
    }

    setPreset (preset) {
        if (preset !== Constants.PRESET_KIND_CUSTOM) {
            this.state.storedValues.selectedPreset = preset;
            this._saveStoredValues();
        }
    }

    _saveStoredValues () {
        this.store.setItem(FILTERS_PRESET_STORE_KEY, this.state.storedValues.selectedPreset);
        this.store.setItem(STORE_KEY_WEBGL_POWER_PREFERENCE, this.state.storedValues.powerPreference);
        this.store.setItem(STORE_KEY_WEBGL_ANTIALIAS, this.state.storedValues.antialias ? 'true' : 'false');
    }

    async changePerformance (performance, direction) {
        const options = [POWER_PREFERENCE_DEFAULT, 'high-performance', 'low-power'];
        let index = options.indexOf(performance);
        switch (direction) {
        case 'inc': index = index + 1; break;
        case 'dec': index = index - 1; break;
        default: throw new Error('Unreachable!');
        }
        const newPerformance = options[index % options.length];
        this.state.storedValues.performance = newPerformance;
        this._saveStoredValues();
        await this._reloadSimulation();
        return newPerformance;
    }

    async changeAntialiasing (currentAntialias) {
        const newAntialias = !currentAntialias;
        this.state.storedValues.antialias = newAntialias;
        this._saveStoredValues();
        await this._reloadSimulation();
    }

    unloadSimulation () {
        this.state.loaded = false;
        this.wasmApp.unload();
        const newCanvas = document.createElement('canvas');
        newCanvas.setAttribute('tabindex', 0);
        this.state.canvas.parentNode.replaceChild(newCanvas, this.state.canvas);
        this.state.canvas.remove();
        this.state.canvas = newCanvas;
    }

    async _reloadSimulation () {
        this.unloadSimulation();
        await new Promise(resolve => setTimeout(resolve, 1000));
        return this._launchSimulation();
    }

    resizeCanvas () {
        const dpi = window.devicePixelRatio;
        const canvas = this.state.canvas;
        const width = canvas.width = canvas.offsetWidth * dpi;
        const height = canvas.height = canvas.offsetHeight * dpi;    
        return { width, height };
    }

    async fireScreenshot ({ buffer, width, height }) {
        Logger.log('starting screenshot');
        Logger.log('width', width, 'height', height);

        var canvas = document.createElement('canvas');
        canvas.width = width;
        canvas.height = height;
        var ctx = canvas.getContext('2d');
    
        var imageData = ctx.createImageData(width, height);
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
        const url = URL.createObjectURL(blob);
        a.href = url;
        a.download = 'Display-Sim_' + new Date().toISOString() + '.png';
        a.click();
    
        await new Promise(resolve => setTimeout(resolve, 3000));
        URL.revokeObjectURL(url);
        a.remove();
    }
}