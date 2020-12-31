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

import { Constants } from '../../services/constants';
import { mobileAndTabletCheck } from '../../services/utils';
import { Navigator } from '../../services/navigator';
import { Visibility } from '../../services/visibility';

export function data () {
    return {
        images: [
            { src: require('../../../assets/pics/opt-frames/wwix.gif').default, hq: require('../../../assets/pics/frames/wwix.gif').default, width: 256, height: 224, id: Constants.FIRST_PREVIEW_IMAGE_ID },
            { src: require('../../../assets/pics/opt-frames/seiken.png').default, hq: require('../../../assets/pics/frames/seiken.png').default, width: 256, height: 224 },
            { src: require('../../../assets/pics/opt-frames/sonicscroll.gif').default, hq: require('../../../assets/pics/frames/sonicscroll.gif').default, width: 320, height: 224 },
            { src: require('../../../assets/pics/opt-frames/metroid.gif').default, hq: require('../../../assets/pics/frames/metroid.gif').default, width: 256, height: 224 },
            { src: require('../../../assets/pics/opt-frames/tf4.gif').default, hq: require('../../../assets/pics/frames/tf4.gif').default, width: 320, height: 224 },
            { src: require('../../../assets/pics/opt-frames/dkc2.png').default, hq: require('../../../assets/pics/frames/dkc2.png').default, width: 256, height: 224 }
        ],
        imageSelection: 0,
        visible: false,
        isRunningOnMobileDevice: mobileAndTabletCheck()
    };
}

export class View {
    constructor (state, refresh, navigator, visibility) {
        this._state = state;
        this._refresh = refresh;
        this._navigator = navigator;
        this._visibility = visibility;
    }

    static make (state, refresh, navigator, visibility) {
        return new View(state, refresh, navigator || Navigator.make(), visibility || Visibility.make());
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
}