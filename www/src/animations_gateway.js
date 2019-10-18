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

import * as fastgif from 'fastgif/fastgif';

import Logger from './logger';

export class AnimationsGateway {
    constructor (config) {
        this.gifCaching = {
            enabled: config.gifCaching === true,
            dict: {}
        };
        this.decoder = new fastgif.Decoder();
    }

    static make (config) {
        return new AnimationsGateway(config);
    }

    async getFromHardcodedTileset () {
        const img = new Image();
        await new Promise((resolve, reject) => {
            img.onload = resolve;
            img.onerror = reject;
            img.src = require('../assets/pics/wwix_spritesheet.png');
        });
        const canvas = document.createElement('canvas');
        const ctx = canvas.getContext('2d');
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

    async getFromImage (img) {
        const isOptimizedAsset = !!img.isOptimizedAsset;
        const isGif = img.isGif || img.src.endsWith('.gif');
        if (isOptimizedAsset) {
            const imgHqSrc = img.dataset.hq;
            Logger.reportIfFalsy(imgHqSrc);
            return this.getFromPath(imgHqSrc);
        } else if (isGif) {
            return this.extractDataFromGifUrl(img.src);
        } else {
            return this.extractDataFromImg(img);
        }
    }

    async getFromPath (path) {
        const isGif = path.endsWith('.gif');
        if (isGif) {
            return this.extractDataFromGifUrl(path);
        } else {
            const img = new Image();
            await new Promise((resolve, reject) => {
                img.onload = resolve;
                img.onerror = reject;
                img.src = path;
            });
            return this.extractDataFromImg(img);
        }
    }

    async extractDataFromGifUrl (gifUrl) {
        Logger.log('loading gif');

        if (!this.gifCaching.enabled) {
            return this.privateFetchGif(gifUrl);
        }

        if (!this.gifCaching.dict[gifUrl]) {
            Logger.log('decoding...');
            this.gifCaching.dict[gifUrl] = await this.privateFetchGif(gifUrl);
        }

        Logger.log('gif loaded');
        return this.gifCaching.dict[gifUrl];
    }

    async privateFetchGif (url) {
        const response = await window.fetch(url);
        const buffer = await response.arrayBuffer();
        const gif = await this.decoder.decode(buffer);
        return gif.map(frame => ({
            raw: frame.imageData,
            delay: frame.delay
        }));
    }

    extractDataFromImg (img) {
        const canvas = document.createElement('canvas');
        const ctx = canvas.getContext('2d');
        canvas.width = img.width;
        canvas.height = img.height;
        ctx.drawImage(img, 0, 0);
        return [{ raw: ctx.getImageData(0, 0, img.width, img.height), delay: 16 }];
    }
}
