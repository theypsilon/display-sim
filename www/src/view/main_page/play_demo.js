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
import Constants from './../../services/constants';

import { SimLauncher } from '../../services/sim_launcher';
import { Visibility } from '../../services/visibility';
import { AnimationsGateway } from '../../services/animations_gateway';

import { calculateAutoScaling } from './play_common';

const visibility = Visibility.make();
const simLauncher = SimLauncher.make();
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
        activePreset: Constants.presetDemo1,
        animations
    });
}
