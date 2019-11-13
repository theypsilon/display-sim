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

import Constants from '../../services/constants';
import { mobileAndTabletCheck } from '../../services/utils';
import { Navigator } from '../../services/navigator';
import { Visibility } from '../../services/visibility';

import { playHtmlSelection } from './play_simulation';

export function model (store) {
    return {
        images: [
            { src: require('../../../assets/pics/opt-frames/wwix.gif'), hq: require('../../../assets/pics/frames/wwix.gif'), width: 256, height: 224, id: Constants.FIRST_PREVIEW_IMAGE_ID },
            { src: require('../../../assets/pics/opt-frames/seiken.png'), hq: require('../../../assets/pics/frames/seiken.png'), width: 256, height: 224 },
            { src: require('../../../assets/pics/opt-frames/sonicscroll.gif'), hq: require('../../../assets/pics/frames/sonicscroll.gif'), width: 320, height: 224 },
            { src: require('../../../assets/pics/opt-frames/metroid.gif'), hq: require('../../../assets/pics/frames/metroid.gif'), width: 256, height: 224 },
            { src: require('../../../assets/pics/opt-frames/tf4.gif'), hq: require('../../../assets/pics/frames/tf4.gif'), width: 320, height: 224 },
            { src: require('../../../assets/pics/opt-frames/dkc2.png'), hq: require('../../../assets/pics/frames/dkc2.png'), width: 256, height: 224 }
        ],
        imageSelection: 0,
        options: makeOptions(store),
        visible: false,
        isRunningOnMobileDevice: mobileAndTabletCheck()
    };
}

function makeOptions (store) {
    return {
        antialias: store.getAntiAliasing(),
        performanceOptions: [
            { value: Constants.POWER_PREFERENCE_DEFAULT, text: 'Default' },
            { value: 'high-performance', text: 'High Performance' },
            { value: 'low-power', text: 'Low Power' }
        ],
        performanceSelection: store.getPowerPreferenceSelectOption(),
        scalingOptions: [
            { value: Constants.SCALING_AUTO_ID, title: 'Auto detect', text: 'Auto Detect' },
            { value: 'scaling-none', title: 'None', text: 'None' },
            { value: Constants.SCALING_43_ID, title: '4:3 on full image', text: '4:3 on full image' },
            { value: Constants.SCALING_STRETCH_TO_BOTH_EDGES_ID, title: 'Stretch to both edges', text: 'Stretch to both edges' },
            { value: Constants.SCALING_STRETCH_TO_NEAREST_EDGE_ID, title: 'Stretch to nearest edge, keeps proportions', text: 'Stretch to nearest edge' },
            { value: Constants.SCALING_CUSTOM_ID, title: 'Introduce the values yourself', text: 'Custom' }
        ],
        scalingSelection: store.getScalingSelectOption(),
        scalingCustom: {
            resolution: { width: store.getCustomResWidth(), height: store.getCustomResHeight() },
            aspectRatio: { x: store.getCustomArX(), y: store.getCustomArY() },
            stretchNearest: store.getCustomStretchNearest()
        }
    };
}

export class View {
    constructor (state, page, store, navigator, visibility) {
        this._state = state;
        this._page = page;
        this._store = store;
        this._Constants = Constants;
        this._navigator = navigator;
        this._visibility = visibility;
        this._isDirty = true;
    }

    static make (state, page, store, navigator, visibility) {
        return new View(state, page, store, navigator || Navigator.make(), visibility || Visibility.make());
    }

    makeItVisible () {
        this._state.visible = true;
        this._page.refresh();
        this._visibility.hideLoading();
    }

    selectImage (idx) {
        this._state.imageSelection = idx;
        this._page.refresh();
    }

    selectPerformance (e) {
        this._state.options.performanceSelection = e.target.value;
        this._page.refresh();
    }

    selectScaling (e) {
        this._state.options.scalingSelection = e.target.value;
        this._page.refresh();
    }

    clickRestoreDefaultOptions () {
        this._store.removeAllOptions();
        this._state.options = makeOptions(this._store, this._Constants);
        this._page.refresh();
    }

    clickDropZone () {
        this._page.getRoot().getElementById('file').click();
    }

    dropOnDropZone (e) {
        e.stopPropagation();
        e.preventDefault();
        this._uploadFile(e.dataTransfer.files[0]);
    }

    dragOverDropZone (e) {
        e.stopPropagation();
        e.preventDefault();
        e.dataTransfer.dropEffect = 'copy';
    }

    changedFileInput (e) {
        this._uploadFile(e.target.files[0]);
    }

    clickPlaySimulation () {
        this._visibility.showLoading();
        this._state.visible = false;
        this._page.refresh();
        this._store.setAntiAliasing(this._state.options.antialias);
        this._store.setPowerPreferenceSelectOption(this._state.options.performanceSelection);
        this._store.setScalingSelectOption(this._state.options.scalingSelection);
        if (this._state.options.scalingSelection === 'scaling-custom') {
            this._store.setCustomResWidth(this._state.options.scalingCustom.resolution.width);
            this._store.setCustomResHeight(this._state.options.scalingCustom.resolution.height);
            this._store.setCustomArX(this._state.options.scalingCustom.aspectRatio.x);
            this._store.setCustomArY(this._state.options.scalingCustom.aspectRatio.y);
            this._store.setCustomStretchNearest(this._state.options.scalingCustom.stretchNearest);
        }
        playHtmlSelection(this._state);
    }

    _uploadFile (file) {
        const url = (window.URL || window.webkitURL).createObjectURL(file);
        this._handleFileToUpload(url).then(() => this.selectImage(this._state.images.length - 1));
    }

    async _handleFileToUpload (url) {
        try {
            await this._processFileToUpload(url);
        } catch (e) {
            console.error(e);
            this._navigator.openTopMessage('That file could not be loaded, try again with a picture.');
        }
    }

    async _processFileToUpload (url) {
        var xhr = new XMLHttpRequest();
        await new Promise((resolve, reject) => {
            xhr.onload = resolve;
            xhr.onerror = reject;
            xhr.open('GET', url, true);
            xhr.responseType = 'blob';
            xhr.send(null);
        });

        const previewUrl = URL.createObjectURL(xhr.response);
        const img = new Image();
        await new Promise((resolve, reject) => {
            img.onload = resolve;
            img.onerror = reject;
            img.setAttribute('crossOrigin', '');
            img.src = previewUrl;
        });

        img.isGif = xhr.response.type === 'image/gif';

        this._state.images.push({ width: img.width, height: img.height, src: previewUrl, hq: previewUrl, img });
    }
}