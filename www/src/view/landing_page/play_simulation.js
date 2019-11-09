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

import { Navigator } from '../../services/navigator';
import { Messenger } from '../../services/messenger';
import { AnimationsGateway } from '../../services/animations_gateway';

const navigator = Navigator.make();
const messenger = Messenger.make();
const animationsGateway = AnimationsGateway.make({ gifCaching: true });

export async function playHtmlSelection (ctx) {
    ctx.visibility.showLoading();

    await new Promise(resolve => setTimeout(resolve, 50));

    const animations = await getAnimations(ctx);

    Logger.log('image readed');

    let scaleX = 1;
    let stretch = false;
    ctx.store.setScalingSelectOption(ctx.elements.optionScalingSelect.value);

    const imageWidth = animations[0].raw.width;
    const imageHeight = animations[0].raw.height;
    let backgroundWidth = imageWidth;
    let backgroundHeight = imageHeight;

    switch (ctx.elements.optionScalingSelect.value) {
    case ctx.constants.SCALING_AUTO_ID:
        const autoScaling = calculateAutoScaling(imageWidth, imageHeight);
        scaleX = autoScaling.scaleX;
        navigator.openTopMessage('Scaling auto detect: ' + autoScaling.message);
        break;
    case ctx.constants.SCALING_43_ID:
        scaleX = (4 / 3) / (imageWidth / imageHeight);
        break;
    case ctx.constants.SCALING_STRETCH_TO_BOTH_EDGES_ID:
        scaleX = (window.screen.width / window.screen.height) / (imageWidth / imageHeight);
        stretch = true;
        break;
    case ctx.constants.SCALING_STRETCH_TO_NEAREST_EDGE_ID:
        stretch = true;
        break;
    case ctx.constants.SCALING_CUSTOM_ID:
        stretch = ctx.elements.scalingCustomStretchNearestDeo.checked;
        ctx.store.setCustomResWidth(ctx.elements.scalingCustomResWidthDeo.value);
        ctx.store.setCustomResHeight(ctx.elements.scalingCustomResHeightDeo.value);
        ctx.store.setCustomArX(ctx.elements.scalingCustomArXDeo.value);
        ctx.store.setCustomArY(ctx.elements.scalingCustomArYDeo.value);
        ctx.store.setCustomStretchNearest(stretch);
        backgroundWidth = +ctx.elements.scalingCustomResWidthDeo.value;
        backgroundHeight = +ctx.elements.scalingCustomResHeightDeo.value;
        scaleX = (+ctx.elements.scalingCustomArXDeo.value / +ctx.elements.scalingCustomArYDeo.value) / (backgroundWidth / backgroundHeight);
        break;
    }

    const ctxOptions = {
        alpha: false,
        antialias: ctx.elements.antialiasDeo.checked,
        depth: true,
        failIfMajorPerformanceCaveat: false,
        powerPreference: ctx.elements.optionPowerPreferenceSelect.value,
        premultipliedAlpha: false,
        preserveDrawingBuffer: false,
        stencil: false
    };

    ctx.store.setAntiAliasing(ctxOptions.antialias);
    ctx.store.setPowerPreferenceSelectOption(ctxOptions.powerPreference);

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
            animations,
            activePreset: ctx.elements.PRESET_KIND_APERTURE_GRILLE_1
        }
    });

    navigator.goToSimPage();
}

export async function playQuerystring (ctx, querystring) {
    Logger.log('Loading querystring: ' + querystring);

    const presetKinds = Object.keys(ctx.constants)
        .filter(key => key.startsWith('PRESET_KIND'))
        .map(key => ctx.constants[key]);

    const searchParams = new URLSearchParams(querystring);

    let selectedPreset = searchParams.get('preset');
    if (!selectedPreset || !presetKinds.includes(selectedPreset)) {
        selectedPreset = ctx.constants.PRESET_KIND_APERTURE_GRILLE_1;
    }

    const hasGif = searchParams.has('gif');
    const animations = searchParams.has('file') ? await animationsGateway.getFromPath(searchParams.get('file'), hasGif) : await animationsGateway.getFromHardcodedTileset();
    const hasBackgroundUi = searchParams.has('bg-ui');
    const hasControllerUi = searchParams.has('ui');
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
        hasBackgroundUi,
        hasControllerUi,
        fullscreen
    });

    navigator.goToSimPage();
}

async function getAnimations (ctx) {
    if (ctx.elements.previewDeo.id === ctx.constants.FIRST_PREVIEW_IMAGE_ID) {
        return animationsGateway.getFromHardcodedTileset();
    } else {
        const img = ctx.elements.previewDeo.querySelector('img');
        return animationsGateway.getFromImage(img);
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
