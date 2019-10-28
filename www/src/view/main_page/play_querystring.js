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

import { SimLauncher } from '../../services/sim_launcher';
import { Visibility } from '../../services/visibility';
import { AnimationsGateway } from '../../services/animations_gateway';

import { calculateAutoScaling } from './play_common';

const visibility = Visibility.make();
const simLauncher = SimLauncher.make();
const animationsGateway = AnimationsGateway.make({ gifCaching: false });

const presetKinds = Object.keys(Constants)
    .filter(key => key.startsWith('PRESET_KIND'))
    .map(key => Constants[key]);

export async function playQuerystring (querystring) {
    Logger.log('Loading querystring: ' + querystring);

    const searchParams = new URLSearchParams(querystring);

    let selectedPreset = searchParams.get('preset');
    if (!selectedPreset || !presetKinds.includes(selectedPreset)) {
        selectedPreset = Constants.PRESET_KIND_APERTURE_GRILLE_1;
    }

    const hasGif = searchParams.has('gif');
    const animations = searchParams.has('file') ? await animationsGateway.getFromPath(searchParams.get('file'), hasGif) : await animationsGateway.getFromHardcodedTileset();
    const hasBackgroundUi = searchParams.has('bg-ui');
    const hasControllerUi = searchParams.has('ui');
    const skipsBackend = searchParams.has('skip-backend');
    const fullscreen = searchParams.has('fullscreen');

    const imageWidth = animations[0].raw.width;
    const imageHeight = animations[0].raw.height;

    if (skipsBackend) {
        simLauncher.removeOldCanvasIfExists();
    } else {
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
            activePreset: selectedPreset,
            animations
        });
    }

    visibility.hideLoading();

    if (hasBackgroundUi) {
        visibility.showSimulationUi();
    }

    if (hasControllerUi) {
        visibility.showSimulationUi();
        visibility.showInfoPanel();
    }

    if (fullscreen) {
        document.body.requestFullscreen();
    }
}
