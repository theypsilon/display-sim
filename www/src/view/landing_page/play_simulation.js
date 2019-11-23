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
import { Mailbox } from '../../services/mailbox';
import { AnimationsGateway } from '../../services/animations_gateway';

const navigator = Navigator.make();
const mailbox = Mailbox.getInstance();
const animationsGateway = AnimationsGateway.make({ gifCaching: true });

export async function playHtmlSelection (state) {
    const animations = await getAnimations(state);

    Logger.log('image readed');

    const imageWidth = animations[0].raw.width;
    const imageHeight = animations[0].raw.height;
    let backgroundWidth = imageWidth;
    let backgroundHeight = imageHeight;

    mailbox.placeMessage('sim-page', {
        topic: 'load-app',
        loadAppParams: {
            imageWidth,
            imageHeight,
            backgroundWidth,
            backgroundHeight,
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

    mailbox.placeMessage('sim-page', {
        topic: 'load-app',
        loadAppParams: {
            imageWidth: imageWidth,
            imageHeight: imageHeight,
            backgroundWidth: imageWidth,
            backgroundHeight: imageHeight,
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