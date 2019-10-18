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

import Logger from '../logger';

import { makeSimLauncher } from '../sim_launcher';
import { makeVisibility } from '../visibility';
import { AnimationsGateway } from '../animations_gateway';

const simLauncher = makeSimLauncher();
const visibility = makeVisibility();
const animationsGateway = AnimationsGateway.make({ gifCaching: false });

export async function playDemo (path) {
    Logger.log('Loading path: ' + path);

    const animations = await animationsGateway.getFromPath(path);

    visibility.hideLoading();

    const imageWidth = animations[0].raw.width;
    const imageHeight = animations[0].raw.height;

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
        animations
    });
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
