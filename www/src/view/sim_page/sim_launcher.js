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

const displaySimPromise = import('../../wasm/display_sim');

let simulationResources;

export class Launcher {
    static make () {
        return new Launcher();
    }

    async launch (canvas, eventBus, params) {
        Logger.log('gl context form', params.ctxOptions);
        const gl = canvas.getContext('webgl2', params.ctxOptions);

        if (!gl) {
            console.error(new Error('Could not get webgl2 context.'));
            return { glError: true };
        }

        this.displaySim = await displaySimPromise;

        const videoInput = new this.displaySim.VideoInputWasm(
            params.imageWidth, params.imageHeight, // to read the image pixels
            canvas.width, canvas.height // gl.viewport
        );
        if (params.backgroundWidth !== params.imageWidth) {
            videoInput.set_background_size(params.backgroundWidth, params.backgroundHeight); // to calculate model distance to the camera
        }
        videoInput.set_max_texture_size(gl.getParameter(gl.MAX_TEXTURE_SIZE));
        for (let i = 0; i < params.animations.length; i++) {
            const rawImg = params.animations[i];
            videoInput.add_picture_frame(new Uint8Array(rawImg.raw.data.buffer), rawImg.delay);
        }

        if (params.activePreset) {
            videoInput.set_preset(params.activePreset);
        }

        if (params.skipDrawing) {
            videoInput.set_drawing_activation(false);
        }

        if (!simulationResources) {
            Logger.log('calling wasm load_simulation_resources');
            simulationResources = this.displaySim.load_simulation_resources();
            Logger.log('wasm load_simulation_resources done');
        }

        Logger.log('calling wasm run_program');
        const owner = this.displaySim.run_program(gl, eventBus, simulationResources, videoInput);
        Logger.log('wasm run_program done');

        return { success: true, owner };
    }

    stop (owner) {
        if (!owner) { return new Error('owner must not be null!'); }
        this.displaySim.stop_program(owner);
    }
}
