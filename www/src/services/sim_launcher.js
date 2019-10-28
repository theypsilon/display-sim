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

import Constants from './constants';
import Logger from './logger';

const displaySimPromise = import('../wasm/display_sim');

export class SimLauncher {
    SimLauncher () {
        this.simulationResources = null;
    }

    static make () {
        return new SimLauncher();
    }

    removeOldCanvasIfExists () {
        const oldCanvas = document.getElementById(Constants.GL_CANVAS_ID);
        if (oldCanvas) {
            oldCanvas.remove();
        }
    }

    async launch (params) {
        this.removeOldCanvasIfExists();
        const canvas = document.createElement('canvas');

        canvas.id = Constants.GL_CANVAS_ID;

        fixCanvasSize(canvas);
        window.addEventListener('resize', fixCanvasSize);

        canvas.onfocus = () => document.dispatchEvent(new KeyboardEvent('keydown', { key: 'canvas_focused' }));
        canvas.onblur = () => document.dispatchEvent(new KeyboardEvent('keyup', { key: 'canvas_focused' }));

        document.body.appendChild(canvas);

        Logger.log('gl context form', params.ctxOptions);
        const gl = canvas.getContext('webgl2', params.ctxOptions);

        var documentElement = document.documentElement;
        documentElement.requestFullscreen = documentElement.requestFullscreen ||
            documentElement.webkitRequestFullScreen ||
            documentElement['mozRequestFullScreen'] ||
            documentElement.msRequestFullscreen;

        canvas.onmousedown = (e) => {
            if (e.buttons !== 1) return;
            canvas.requestPointerLock();
            if (window.screen.width !== window.innerWidth && window.screen.height !== window.innerHeight) {
                documentElement.requestFullscreen();
            }
        };

        canvas.requestPointerLock = canvas.requestPointerLock || canvas.mozRequestPointerLock;
        document.exitPointerLock = document.exitPointerLock || document.mozExitPointerLock;

        if (!gl) {
            console.error(new Error('Could not get webgl2 context.'));
            canvas.remove();
            return { glError: true };
        }

        const displaySim = await displaySimPromise;

        const videoInput = new displaySim.VideoInputWasm(
            params.imageWidth, params.imageHeight, // to read the image pixels
            canvas.width, canvas.height // gl.viewport
        );
        if (params.backgroundWidth !== params.imageWidth) {
            videoInput.set_background_size(params.backgroundWidth, params.backgroundHeight); // to calculate model distance to the camera
        }
        videoInput.set_pixel_width(params.scaleX);
        if (params.stretch === true) {
            videoInput.stretch();
        }
        videoInput.set_max_texture_size(gl.getParameter(gl.MAX_TEXTURE_SIZE));
        for (let i = 0; i < params.animations.length; i++) {
            const rawImg = params.animations[i];
            videoInput.add_picture_frame(new Uint8Array(rawImg.raw.data.buffer), rawImg.delay);
        }

        if (!this.simulationResources) {
            Logger.log('calling wasm load_simulation_resources');
            this.simulationResources = displaySim.load_simulation_resources();
            Logger.log('wasm load_simulation_resources done');
        }

        if (params.activePreset) {
            videoInput.set_preset(params.activePreset);
        }

        Logger.log('calling wasm run_program');
        displaySim.run_program(gl, this.simulationResources, videoInput);
        Logger.log('wasm run_program done');

        return { success: true };
    }
}

function fixCanvasSize (canvas) {
    canvas = canvas instanceof HTMLCanvasElement ? canvas : document.getElementById(Constants.GL_CANVAS_ID);
    if (!canvas) return;

    const dpi = window.devicePixelRatio;
    const width = window.screen.width;
    const height = window.screen.height;
    const zoom = window.outerWidth / window.innerWidth;

    canvas.width = Math.round(width * dpi / zoom / 80) * 80;
    canvas.height = Math.round(height * dpi / zoom / 60) * 60;

    canvas.style.width = window.innerWidth;
    canvas.style.height = window.innerHeight + 0.5;

    Logger.log('resolution:', canvas.width, canvas.height, width, height);

    const infoPanelContentHeight = (window.innerHeight - 18) * 0.95;
    Constants.infoPanelContentDeo.style.setProperty('max-height', infoPanelContentHeight);
    Constants.infoPanelAdvancedSettingsDeo.style.setProperty('max-height', infoPanelContentHeight - 60);
}
