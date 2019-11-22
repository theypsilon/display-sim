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
import { Launcher } from './sim_launcher';

const STORE_KEY_WEBGL_POWER_PREFERENCE = 'option-powerPreference';
const STORE_KEY_WEBGL_ANTIALIAS = 'option-antialias';

export class Model {
    constructor (canvas, eventBus, mailbox, launcher, store) {
        this.eventBus = eventBus;
        this.mailbox = mailbox;
        this.launcher = launcher;
        this.store = store;
        this.state = {
            canvas, msg: null, simOwner: null,
            storedValues: {
                selectedPreset: this.store.getItem(Constants.FILTERS_PRESET_STORE_KEY) || Constants.PRESET_KIND_APERTURE_GRILLE_1,
                powerPreference: this.store.getItem(STORE_KEY_WEBGL_POWER_PREFERENCE) || Constants.POWER_PREFERENCE_DEFAULT,
                antialias: this.store.getItem(STORE_KEY_WEBGL_ANTIALIAS) !== 'false'
            }
        };
    }

    static make (canvas, eventBus, mailbox, launcher, store) {
        return new Model(canvas, eventBus, mailbox || Mailbox.getInstance(), launcher || Launcher.make(), store || LocalStorage.make('sim-page'));
    }

    load () {
        const messages = this.mailbox.consumeMessages('sim-page');
        if (messages.length !== 1) {
            throw new Error('Can not handle these messages.', messages);
        }

        this.state.msg = messages.filter(msg => msg.topic === 'launch')[0];
        return this._launchSimulation();
    }

    async _launchSimulation () {
        this.resizeCanvas();
        const result = await this.launcher.launch(this.state.canvas, this.eventBus, Object.assign({
            ctxOptions: {
                alpha: false,
                antialias: this.state.storedValues.antialias,
                depth: true,
                failIfMajorPerformanceCaveat: false,
                powerPreference: this.state.storedValues.powerPreference,
                premultipliedAlpha: false,
                preserveDrawingBuffer: false,
                stencil: false
            },
        }, this.state.msg.launcherParams));
        this.state.simOwner = result.owner;
        return Object.assign({ storedValues: this.state.storedValues }, this.state.msg, result);
    }

    setPreset (preset) {
        if (preset !== Constants.PRESET_KIND_CUSTOM) {
            this.state.storedValues.selectedPreset = preset;
            this._saveStoredValues();
        }
    }

    _saveStoredValues() {
        this.store.setItem(Constants.FILTERS_PRESET_STORE_KEY, this.state.storedValues.selectedPreset);
        this.store.setItem(STORE_KEY_WEBGL_POWER_PREFERENCE, this.state.storedValues.powerPreference);
        this.store.setItem(STORE_KEY_WEBGL_ANTIALIAS, this.state.storedValues.antialias ? 'true' : 'false');
    }

    async changePerformance (performance, direction) {
        const options = [Constants.POWER_PREFERENCE_DEFAULT, 'high-performance', 'low-power'];
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

    async _reloadSimulation () {
        this.launcher.stop(this.state.simOwner);
        this.state.simOwner = null;
        const newCanvas = document.createElement('canvas');
        newCanvas.setAttribute('tabindex', 0);
        this.state.canvas.parentNode.replaceChild(newCanvas, this.state.canvas);
        this.state.canvas = newCanvas;
        await new Promise(resolve => setTimeout(resolve, 1000));
        return this._launchSimulation();
    }

    resizeCanvas () {
        const dpi = window.devicePixelRatio;
        const width = window.screen.width;
        const height = window.screen.height;
        const zoom = window.outerWidth / window.innerWidth;
    
        this.state.canvas.width = Math.round(width * dpi / zoom / 80) * 80;
        this.state.canvas.height = Math.round(height * dpi / zoom / 60) * 60;
    
        this.state.canvas.style.width = window.innerWidth;
        this.state.canvas.style.height = window.innerHeight + 0.5;
    
        Logger.log('resolution:', this.state.canvas.width, this.state.canvas.height, width, height);
    }

    async fireScreenshot (args) {
        Logger.log('starting screenshot');
    
        const arrayBuffer = args[0];
        const multiplier = args[1];
    
        const width = 1920 * 2 * multiplier;
        const height = 1080 * 2 * multiplier;
        var canvas = document.createElement('canvas');
        canvas.width = width;
        canvas.height = height;
        var ctx = canvas.getContext('2d');
    
        var imageData = ctx.createImageData(width, height);
        imageData.data.set(arrayBuffer);
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