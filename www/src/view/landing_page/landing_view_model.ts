/* Copyright (c) 2019-2021 José manuel Barroso Galindo <theypsilon@gmail.com>
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

export interface SimImage {
    src: string,
    hq: string,
    width: number,
    height: number,
    id?: string,
    img?: HTMLImageElement & {isGif: boolean},
    isGif?: boolean
}

export function data () {
    return {
        images: [
            { src: require('../../../assets/pics/opt-frames/wwix.gif').default, hq: require('../../../assets/pics/frames/wwix.gif').default, width: 256, height: 224, id: Constants.FIRST_PREVIEW_IMAGE_ID } as SimImage,
            { src: require('../../../assets/pics/opt-frames/seiken.png').default, hq: require('../../../assets/pics/frames/seiken.png').default, width: 256, height: 224 } as SimImage,
            { src: require('../../../assets/pics/opt-frames/sonicscroll.gif').default, hq: require('../../../assets/pics/frames/sonicscroll.gif').default, width: 320, height: 224 } as SimImage,
            { src: require('../../../assets/pics/opt-frames/metroid.gif').default, hq: require('../../../assets/pics/frames/metroid.gif').default, width: 256, height: 224 } as SimImage,
            { src: require('../../../assets/pics/opt-frames/tf4.gif').default, hq: require('../../../assets/pics/frames/tf4.gif').default, width: 320, height: 224 } as SimImage,
            { src: require('../../../assets/pics/opt-frames/dkc2.png').default, hq: require('../../../assets/pics/frames/dkc2.png').default, width: 256, height: 224 } as SimImage
        ],
        imageSelection: 0,
        visible: false,
        isRunningOnMobileDevice: mobileAndTabletCheck()
    };
}

export type ViewData = ReturnType<typeof data>;

export class View {
    private readonly _refresh: () => void;
    private readonly _state: ViewData;
    private readonly _navigator: Navigator;
    private readonly _visibility: Visibility;

    constructor (state: ViewData, refresh: (() => void), navigator: Navigator, visibility: Visibility) {
        this._state = state;
        this._refresh = refresh;
        this._navigator = navigator;
        this._visibility = visibility;
    }

    static make (state: ViewData, refresh: (() => void), navigator?: Navigator, visibility?: Visibility): View {
        return new View(state, refresh, navigator || Navigator.make(), visibility || Visibility.make());
    }

    turnVisibilityOn (): void {
        this._state.visible = true;
        this._refresh();
        this._visibility.hideLoading();
    }

    turnVisibilityOff (): void {
        this._visibility.showLoading();
        this._state.visible = false;
        this._refresh();
    }

    showError (message: string): void {
        this._navigator.openTopMessage(message);
    }

    selectImage (idx: number): void {
        this._state.imageSelection = idx;
        this._refresh();
    }

    addImage (image: SimImage): void {
        this._state.images.push(image);
        this.selectImage(this._state.images.length - 1);
    }
}