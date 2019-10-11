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

 import Logger from '../logger';
 
 import { makeSimLauncher } from '../sim_launcher';
 import { makeVisibility } from '../visibility';
 
 const simLauncher = makeSimLauncher();
 const visibility = makeVisibility();

export async function playDemo (path) {
    console.log(path);
    const isGif = path.includes('.gif');
    const img = new Image();
    img.src = path;
    await new Promise((resolve, reject) => {
        img.onload = resolve;
        img.onerror = reject;
    });
    const canvas = document.createElement('canvas');
    const ctx = canvas.getContext('2d');
    canvas.width = img.width;
    canvas.height = img.height;
    ctx.drawImage(img, 0, 0);
    let rawImgs; 
    if (isGif) {
        Logger.log('loading gif');
        const decoder = new fastgif.Decoder();
        const gif = await window.fetch(img.src)
            .then(response => response.arrayBuffer())
            .then(buffer => decoder.decode(buffer));
        Logger.log('gif loaded');
        rawImgs = gif.map(frame => ({
            raw: frame.imageData,
            delay: frame.delay
        }));
    } else {
        rawImgs = [{ raw: ctx.getImageData(0, 0, img.width, img.height), delay: 16 }];
    }

    visibility.hideLoading();

    const imageWidth = rawImgs[0].raw.width;
    const imageHeight = rawImgs[0].raw.height;

    await simLauncher.launch({
        ctxOptions: {
            alpha: false,
            antialias: false,
            depth: true,
            failIfMajorPerformanceCaveat: false,
            powerPreference: 'high-performance',
            premultipliedAlpha: false,
            preserveDrawingBuffer: false,
            stencil: false
        },
        scaleX: calculateAutoScaling(imageWidth, imageHeight).scaleX,
        imageWidth: imageWidth,
        imageHeight: imageHeight,
        backgroundWidth: imageWidth,
        backgroundHeight: imageHeight,
        stretch: false,
        rawImgs
    });

    //visibility.showSimulationUi();
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
