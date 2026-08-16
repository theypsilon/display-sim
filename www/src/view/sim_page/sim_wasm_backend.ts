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

import { Logger } from '../../services/logger';
import { Lazy } from '../../services/lazy';

export class SimWasmBackend {
    private _app: any;
    private _canvas: HTMLCanvasElement | null;

    private static _instance: Lazy<SimWasmBackend> = Lazy.from(() => new SimWasmBackend());
    static getInstance (): SimWasmBackend { return this._instance.get(); }
    private constructor () {
        this._app = null;
        this._canvas = null;
    }

    async load (canvas: HTMLCanvasElement, eventBus: any, params: any) {

        // @ts-ignore
        const { WasmApp, VideoInputConfig } = await import('../../wasm/display_sim');

        if (!this._app) {
            Logger.log('calling new WasmApp');
            this._app = new WasmApp();
            Logger.log('new WasmApp done');
        }

        Logger.log('resolutions:', canvas.width, canvas.height, params.imageWidth, params.imageHeight);

        const config = new VideoInputConfig(
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
        const gl = canvas.getContext('webgl2', params.ctxOptions) as WebGL2RenderingContext | null;

        if (gl) {
            this._canvas = canvas;
            config.set_max_texture_size(gl.getParameter(gl.MAX_TEXTURE_SIZE));

            Logger.log('calling wasmApp.load');
            this._app.load(gl, eventBus, config);
            Logger.log('wasmApp.load done');
    
            return { success: true };   
        } else {
            console.error(new Error('Could not get webgl2 context.'));
            return { glError: true };
        }
    }

    runFrame () {
        if (this._canvas) {
            this._app.set_ui_metrics(this._canvas.width, this._canvas.height, window.devicePixelRatio || 1);
        }
        return this._app.run_frame();
    }

    uiEvent (kind: string, value: any) {
        this._app.ui_event(kind, value);
    }

    uiCapturesPointer (): boolean {
        return Boolean(this._app && this._app.ui_captures_pointer());
    }

    uiWantsKeyboard (): boolean {
        return Boolean(this._app && this._app.ui_wants_keyboard());
    }

    uiMessage (message: string) {
        if (this._app) {
            this._app.ui_message(message);
        }
    }

    unload () {
        this._canvas = null;
        return this._app.unload();
    }
}
