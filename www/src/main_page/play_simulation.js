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

import Globals from '../globals';
import Logger from '../logger';

import { makeSimLauncher } from '../sim_launcher';
import { makeVisibility } from '../visibility';
import { makeStorage } from '../storage';

const simLauncher = makeSimLauncher();
const visibility = makeVisibility();
const storage = makeStorage();

const gifCache = {};
window.gifCache = gifCache;

let previewDeo = document.getElementById(Globals.firstPreviewImageId);

export async function playSimulation () {
    visibility.showLoading();

    await new Promise(resolve => setTimeout(resolve, 50));

    const rawImgs = await (async function () {
        if (previewDeo.id === Globals.firstPreviewImageId) {
            const img = new Image();
            img.src = require('../../assets/pics/wwix_spritesheet.png');
            await new Promise((resolve, reject) => {
                img.onload = resolve;
                img.onerror = reject;
            });
            const canvas = document.createElement('canvas');
            const ctx = canvas.getContext('2d');
            canvas.width = img.width;
            canvas.height = img.height;
            ctx.drawImage(img, 0, 0);
            const columns = Math.floor(img.width / 256);
            const rawImgs = [];
            for (let i = 0; i <= 45; i++) {
                const x = i % columns;
                const y = Math.floor(i / columns);
                rawImgs.push({ raw: ctx.getImageData(x * 256, y * 224, 256, 224), delay: 16 });
            }
            return rawImgs;
        } else {
            let img = previewDeo.querySelector('img');
            const isAsset = !!img.isAsset;
            const isGif = !!img.isGif;
            if (isAsset) {
                const imgHqSrc = img.dataset.hq;
                img = new Image();
                img.src = imgHqSrc;
                await new Promise((resolve, reject) => {
                    img.onload = resolve;
                    img.onerror = reject;
                });
            }
            const canvas = document.createElement('canvas');
            const ctx = canvas.getContext('2d');
            canvas.width = img.width;
            canvas.height = img.height;
            ctx.drawImage(img, 0, 0);
            if (!isGif) {
                return [{ raw: ctx.getImageData(0, 0, img.width, img.height), delay: 16 }];
            }
            Logger.log('loading gif');
            const gifKey = isAsset ? img.src : canvas.toDataURL();
            let gif = gifCache[gifKey];
            if (!gif) {
                Logger.log('decoding...');
                const decoder = new fastgif.Decoder();
                gif = await window.fetch(img.src)
                    .then(response => response.arrayBuffer())
                    .then(buffer => decoder.decode(buffer));
                gifCache[gifKey] = gif;
            }
            Logger.log('gif loaded');
            return gif.map(frame => ({
                raw: frame.imageData,
                delay: frame.delay
            }));
        }
    }());

    Logger.log('image readed');

    let scaleX = 1;
    let stretch = false;
    storage.setScalingSelectOption(Globals.optionScalingSelect.value);

    const imageWidth = rawImgs[0].raw.width;
    const imageHeight = rawImgs[0].raw.height;
    let backgroundWidth = imageWidth;
    let backgroundHeight = imageHeight;

    switch (Globals.optionScalingSelect.value) {
    case Globals.scalingAutoHtmlId:
        const autoScaling = calculateAutoScaling(imageWidth, imageHeight);
        scaleX = autoScaling.scaleX;
        window.dispatchEvent(new CustomEvent('app-event.top_message', {
            detail: 'Scaling auto detect: ' + autoScaling.message
        }));
        break;
    case Globals.scaling43HtmlId:
        scaleX = (4 / 3) / (imageWidth / imageHeight);
        break;
    case Globals.scalingStretchToBothEdgesHtmlId:
        scaleX = (window.screen.width / window.screen.height) / (imageWidth / imageHeight);
        stretch = true;
        break;
    case Globals.scalingStretchToNearestEdgeHtmlId:
        stretch = true;
        break;
    case Globals.scalingCustomHtmlId:
        stretch = Globals.scalingCustomStretchNearestDeo.checked;
        storage.setCustomResWidth(Globals.scalingCustomResWidthDeo.value);
        storage.setCustomResHeight(Globals.scalingCustomResHeightDeo.value);
        storage.setCustomArX(Globals.scalingCustomArXDeo.value);
        storage.setCustomArY(Globals.scalingCustomArYDeo.value);
        storage.setCustomStretchNearest(stretch);
        backgroundWidth = +Globals.scalingCustomResWidthDeo.value;
        backgroundHeight = +Globals.scalingCustomResHeightDeo.value;
        scaleX = (+Globals.scalingCustomArXDeo.value / +Globals.scalingCustomArYDeo.value) / (backgroundWidth / backgroundHeight);
        break;
    }

    Globals.lightColorDeo.value = '#FFFFFF';
    Globals.brightnessColorDeo.value = '#FFFFFF';

    const ctxOptions = {
        alpha: false,
        antialias: Globals.antialiasDeo.checked,
        depth: true,
        failIfMajorPerformanceCaveat: false,
        powerPreference: Globals.optionPowerPreferenceSelect.value,
        premultipliedAlpha: false,
        preserveDrawingBuffer: false,
        stencil: false
    };

    storage.setAntiAliasing(ctxOptions.antialias);
    storage.setPowerPreferenceSelectOption(ctxOptions.powerPreference);

    const result = await simLauncher.launch({
        ctxOptions,
        scaleX,
        imageWidth,
        imageHeight,
        backgroundWidth,
        backgroundHeight,
        stretch,
        rawImgs
    });

    if (result.glError) {
        window.dispatchEvent(new CustomEvent('app-event.top_message', {
            detail: 'WebGL2 is not working on your browser, try restarting it! And remember, this works only on a PC with updated browser and graphics drivers.'
        }));
        return { reloadPage: true };
    }

    Globals.filterPresetsDeo.value = storage.getFilterPresets();
    Globals.filterPresetsDeo.onchange();

    //    Globals.filterPresetsBasicDeo.value = storage.getFilterPresets();
    //    Globals.filterPresetsBasicDeo.onchange();

    visibility.hideLoading();
    visibility.showSimulationUi();

    return { reloadPage: false };
}

function calculateAutoScaling (imageWidth, imageHeight) {
    if (imageHeight > 540) {
        return {
            scaleX: 1,
            message: 'none.'
        };
    } else if (imageHeight === 144) {
        return {
            scaleX: (11 / 10) / (imageWidth / imageHeight),
            message: '11:10 (Game Boy) on full image.'
        };
    } else if (imageHeight === 160) {
        return {
            scaleX: (3 / 2) / (imageWidth / imageHeight),
            message: '3:2 (Game Boy Advance) on full image.'
        };
    } else {
        return {
            scaleX: (4 / 3) / (imageWidth / imageHeight),
            message: '4:3 on full image.'
        };
    }
}
