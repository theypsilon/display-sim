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

// @ts-ignore
import { Decoder } from 'fastgif/fastgif';

import { Logger } from './logger';
import {throwOnNull} from "./guards";

declare global {
    interface Window {
        saveCanvas: HTMLCanvasElement; saveImageData: ImageData
    }
}

declare interface Decoder {
    decode(buffer: ArrayBuffer): Promise<RawDecodedFrame[]>;
}

interface RawDecodedFrame {
    imageData: ImageData, delay: Number
}

interface TransformedDecodedFrame {
    raw: ImageData, delay: Number
}

interface AnimationsGatewayConfig {
    gifCaching: boolean
}

export class AnimationsGateway {
    private readonly _gifCaching: { dict: { [id: string]: TransformedDecodedFrame[]; }; enabled: boolean };
    private readonly _decoder: Decoder;

    constructor (config: AnimationsGatewayConfig) {
        this._gifCaching = {
            enabled: config.gifCaching,
            dict: {}
        };
        this._decoder = new Decoder();
    }

    static make (config: AnimationsGatewayConfig): AnimationsGateway {
        return new AnimationsGateway(config);
    }

    async getFromHardcodedTileset (): Promise<TransformedDecodedFrame[]> {
        const img = new Image();
        await new Promise((resolve, reject) => {
            img.onload = resolve;
            img.onerror = reject;
            img.src = require('../../assets/pics/wwix_spritesheet.png').default;
        });
        const canvas = document.createElement('canvas');
        const ctx = throwOnNull(canvas.getContext('2d'));
        canvas.width = img.width;
        canvas.height = img.height;
        ctx.drawImage(img, 0, 0);
        const columns = Math.floor(img.width / 256);
        const animations = [];
        for (let i = 0; i <= 45; i++) {
            const x = i % columns;
            const y = Math.floor(i / columns);
            animations.push({ raw: ctx.getImageData(x * 256, y * 224, 256, 224), delay: 16 });
        }
        return animations;
    }

    async getFromImage (img: HTMLImageElement & {isGif: boolean}): Promise<TransformedDecodedFrame[]> {
        const isGif = img.isGif || img.src.endsWith('.gif');
        if (isGif) {
            return this.extractDataFromGifUrl(img.src);
        } else {
            return this.extractDataFromImg(img);
        }
    }

    async getFromPath (path: string, forceGif: boolean): Promise<TransformedDecodedFrame[]> {
        const isGif = forceGif || path.endsWith('.gif');
        if (isGif) {
            return this.extractDataFromGifUrl(path);
        } else {
            const img = new Image();
            await new Promise((resolve, reject) => {
                img.onload = resolve;
                img.onerror = reject;
                img.setAttribute('crossOrigin', '');
                img.src = path;
            });
            return this.extractDataFromImg(img);
        }
    }

    async extractDataFromGifUrl (gifUrl: string): Promise<TransformedDecodedFrame[]> {
        Logger.log('loading gif');

        if (!this._gifCaching.enabled) {
            return this.privateFetchGif(gifUrl);
        }

        if (!this._gifCaching.dict[gifUrl]) {
            Logger.log('decoding...');
            this._gifCaching.dict[gifUrl] = await this.privateFetchGif(gifUrl);
        }

        Logger.log('gif loaded');
        return this._gifCaching.dict[gifUrl];
    }

    async privateFetchGif (url: string): Promise<TransformedDecodedFrame[]> {
        const response = await window.fetch(url, { mode: 'no-cors' });
        const buffer = await response.arrayBuffer();
        const gif = await this._decoder.decode(buffer);
        return gif.map((frame: RawDecodedFrame) => ({
            raw: frame.imageData,
            delay: frame.delay
        }));
    }

    extractDataFromImg (img: HTMLImageElement): TransformedDecodedFrame[] {
        img.setAttribute('crossOrigin', '');
        const canvas = document.createElement('canvas');
        const ctx = throwOnNull(canvas.getContext('2d'));
        canvas.width = img.width;
        canvas.height = img.height;
        ctx.drawImage(img, 0, 0);
        const imageData = ctx.getImageData(0, 0, img.width, img.height);
        window.saveCanvas = canvas;
        window.saveImageData = imageData;
        return [{ raw: imageData, delay: 16 }];
    }
}
