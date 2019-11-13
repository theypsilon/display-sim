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
import Constants from '../../services/constants';

import { Navigator } from '../../services/navigator';
import { Messenger } from '../../services/messenger';
import { AnimationsGateway } from '../../services/animations_gateway';

const navigator = Navigator.make();
const messenger = Messenger.getInstance();
const animationsGateway = AnimationsGateway.make({ gifCaching: true });

export async function playHtmlSelection (state) {
    const animations = await getAnimations(state);

    Logger.log('image readed');

    let scaleX = 1;
    let stretch = false;

    const imageWidth = animations[0].raw.width;
    const imageHeight = animations[0].raw.height;
    let backgroundWidth = imageWidth;
    let backgroundHeight = imageHeight;

    switch (state.options.scalingSelection) {
    case Constants.SCALING_AUTO_ID:
        const autoScaling = calculateAutoScaling(imageWidth, imageHeight);
        scaleX = autoScaling.scaleX;
        navigator.openTopMessage('Scaling auto detect: ' + autoScaling.message);
        break;
    case Constants.SCALING_43_ID:
        scaleX = (4 / 3) / (imageWidth / imageHeight);
        break;
    case Constants.SCALING_STRETCH_TO_BOTH_EDGES_ID:
        scaleX = (window.screen.width / window.screen.height) / (imageWidth / imageHeight);
        stretch = true;
        break;
    case Constants.SCALING_STRETCH_TO_NEAREST_EDGE_ID:
        stretch = true;
        break;
    case Constants.SCALING_CUSTOM_ID:
        stretch = state.options.scalingCustom.stretchNearest;
        backgroundWidth = +state.options.scalingCustom.resolution.width;
        backgroundHeight = +state.options.scalingCustom.resolution.height;
        scaleX = (+state.options.scalingCustom.aspectRatio.x / +state.options.scalingCustom.aspectRatio.y) / (backgroundWidth / backgroundHeight);
        break;
    }

    const ctxOptions = {
        alpha: false,
        antialias: state.options.antialias,
        depth: true,
        failIfMajorPerformanceCaveat: false,
        powerPreference: state.options.performanceSelection,
        premultipliedAlpha: false,
        preserveDrawingBuffer: false,
        stencil: false
    };

    messenger.sendMessage('sim-page', {
        topic: 'launch',
        launcherParams: {
            ctxOptions,
            scaleX,
            imageWidth,
            imageHeight,
            backgroundWidth,
            backgroundHeight,
            stretch,
            animations
        }
    });

    navigator.goToSimPage();
}

export async function playQuerystring (querystring) {
    Logger.log('Loading querystring: ' + querystring);

    const searchParams = new URLSearchParams(querystring);

    const selectedPreset = searchParams.get('preset');
    const hasGif = searchParams.has('gif');
    const animations = searchParams.has('file') ? await animationsGateway.getFromPath(searchParams.get('file'), hasGif) : await animationsGateway.getFromHardcodedTileset();
    const skipControllerUi = searchParams.has('skip-ui');
    const skipDrawing = searchParams.has('skip-drawing');
    const fullscreen = searchParams.has('fullscreen');

    const imageWidth = animations[0].raw.width;
    const imageHeight = animations[0].raw.height;

    messenger.sendMessage('sim-page', {
        topic: 'launch',
        launcherParams: {
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
            activePreset: selectedPreset,
            animations,
            skipDrawing
        },
        skipControllerUi,
        fullscreen
    });

    navigator.goToSimPage();
}

async function getAnimations (state) {
    const selectedImage = state.images[state.imageSelection];
    if (selectedImage.id === Constants.FIRST_PREVIEW_IMAGE_ID) {
        return animationsGateway.getFromHardcodedTileset();
    } else if (selectedImage.img) {
        return animationsGateway.getFromImage(selectedImage.img);
    } else {
        return animationsGateway.getFromPath(selectedImage.hq, selectedImage.isGif);
    }
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