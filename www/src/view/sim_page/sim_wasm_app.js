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

import Logger from '../../services/logger';

let instance;

export class WasmApp {
    constructor () {
        this.app = null;
    }
    static getInstance () {
        return instance;
    }

    async load (canvas, eventBus, params) {
        const exports = await import('../../wasm/display_sim');

        if (!this.app) {
            Logger.log('calling new WasmApp');
            this.app = new exports.WasmApp();
            Logger.log('new WasmApp done');
        }

        Logger.log('resolutions:', canvas.width, canvas.height, params.imageWidth, params.imageHeight);

        const config = new exports.VideoInputConfig(
            params.imageWidth, params.imageHeight, // to read the image pixels
            canvas.width, canvas.height // gl.viewport
        );

        if (params.backgroundWidth !== params.imageWidth) {
            config.set_background_size(params.backgroundWidth, params.backgroundHeight); // to calculate model distance to the camera
        }

        for (let i = 0; i < params.animations.length; i++) {
            const rawImg = params.animations[i];
            config.add_picture_frame(new Uint8Array(rawImg.raw.data.buffer), rawImg.delay);
        }

        if (params.activePreset) {
            config.set_preset(params.activePreset);
        }

        if (params.skipDrawing) {
            config.set_drawing_activation(false);
        }

        Logger.log('gl context form', params.ctxOptions);
        const gl = canvas.getContext('webgl2', params.ctxOptions);

        if (gl) {
            config.set_max_texture_size(gl.getParameter(gl.MAX_TEXTURE_SIZE));

            Logger.log('calling wasmApp.load');
            this.app.load(gl, eventBus, config);
            Logger.log('wasmApp.load done');
    
            return { success: true };   
        } else {
            console.error(new Error('Could not get webgl2 context.'));
            return { glError: true };
        }
    }

    runFrame () {
        return this.app.run_frame();
    }

    unload () {
        return this.app.unload();
    }
}

instance = new WasmApp();