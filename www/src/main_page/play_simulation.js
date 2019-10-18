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

import Constants from '../constants';
import GlobalState from '../global_state';
import Logger from '../logger';

import { SimLauncher } from '../sim_launcher';
import { Visibility } from '../visibility';
import { Storage } from '../storage';
import { AnimationsGateway } from '../animations_gateway';

const simLauncher = SimLauncher.make();
const visibility = Visibility.make();
const storage = Storage.make();
const animationsGateway = AnimationsGateway.make({ gifCaching: true });

async function getAnimations () {
    if (GlobalState.previewDeo.id === Constants.firstPreviewImageId) {
        return animationsGateway.getFromHardcodedTileset();
    } else {
        const img = GlobalState.previewDeo.querySelector('img');
        return animationsGateway.getFromImage(img);
    }
}

export async function playSimulation () {
    visibility.showLoading();

    await new Promise(resolve => setTimeout(resolve, 50));

    const animations = await getAnimations();

    Logger.log('image readed');

    let scaleX = 1;
    let stretch = false;
    storage.setScalingSelectOption(Constants.optionScalingSelect.value);

    const imageWidth = animations[0].raw.width;
    const imageHeight = animations[0].raw.height;
    let backgroundWidth = imageWidth;
    let backgroundHeight = imageHeight;

    switch (Constants.optionScalingSelect.value) {
    case Constants.scalingAutoHtmlId:
        const autoScaling = calculateAutoScaling(imageWidth, imageHeight);
        scaleX = autoScaling.scaleX;
        window.dispatchEvent(new CustomEvent('app-event.top_message', {
            detail: 'Scaling auto detect: ' + autoScaling.message
        }));
        break;
    case Constants.scaling43HtmlId:
        scaleX = (4 / 3) / (imageWidth / imageHeight);
        break;
    case Constants.scalingStretchToBothEdgesHtmlId:
        scaleX = (window.screen.width / window.screen.height) / (imageWidth / imageHeight);
        stretch = true;
        break;
    case Constants.scalingStretchToNearestEdgeHtmlId:
        stretch = true;
        break;
    case Constants.scalingCustomHtmlId:
        stretch = Constants.scalingCustomStretchNearestDeo.checked;
        storage.setCustomResWidth(Constants.scalingCustomResWidthDeo.value);
        storage.setCustomResHeight(Constants.scalingCustomResHeightDeo.value);
        storage.setCustomArX(Constants.scalingCustomArXDeo.value);
        storage.setCustomArY(Constants.scalingCustomArYDeo.value);
        storage.setCustomStretchNearest(stretch);
        backgroundWidth = +Constants.scalingCustomResWidthDeo.value;
        backgroundHeight = +Constants.scalingCustomResHeightDeo.value;
        scaleX = (+Constants.scalingCustomArXDeo.value / +Constants.scalingCustomArYDeo.value) / (backgroundWidth / backgroundHeight);
        break;
    }

    Constants.lightColorDeo.value = '#FFFFFF';
    Constants.brightnessColorDeo.value = '#FFFFFF';

    const ctxOptions = {
        alpha: false,
        antialias: Constants.antialiasDeo.checked,
        depth: true,
        failIfMajorPerformanceCaveat: false,
        powerPreference: Constants.optionPowerPreferenceSelect.value,
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
        animations
    });

    if (result.glError) {
        window.dispatchEvent(new CustomEvent('app-event.top_message', {
            detail: 'WebGL2 is not working on your browser, try restarting it! And remember, this works only on a PC with updated browser and graphics drivers.'
        }));
        return { reloadPage: true };
    }

    Constants.filterPresetsDeo.value = storage.getFilterPresets();
    Constants.filterPresetsDeo.onchange();

    //    Constants.filterPresetsBasicDeo.value = storage.getFilterPresets();
    //    Constants.filterPresetsBasicDeo.onchange();

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
