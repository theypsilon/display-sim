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

export function data (store) {
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
    constructor (state, refresh, store, navigator, visibility) {
        this._state = state;
        this._refresh = refresh;
        this._store = store;
        this._navigator = navigator;
        this._visibility = visibility;
    }

    static make (state, refresh, store, navigator, visibility) {
        return new View(state, refresh, store, navigator || Navigator.make(), visibility || Visibility.make());
    }

    turnVisibilityOn () {
        this._state.visible = true;
        this._refresh();
        this._visibility.hideLoading();
    }

    turnVisibilityOff () {
        this._visibility.showLoading();
        this._state.visible = false;
        this._refresh();
    }

    showError (message) {
        this._navigator.openTopMessage(message);
    }

    selectImage (idx) {
        this._state.imageSelection = idx;
        this._refresh();
    }

    addImage (image) {
        this._state.images.push(image);
        this.selectImage(this._state.images.length - 1);
    }

    selectScaling (value) {
        this._state.options.scalingSelection = value;
        this._refresh();
    }
}