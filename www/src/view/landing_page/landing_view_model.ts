/* Copyright (c) 2019-2022 José manuel Barroso Galindo <theypsilon@gmail.com>
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
import {LandingTemplate} from "./landing_template";
import {SimImage, Images} from "../../services/images";

export function data () {
    return {
        images: [
            { src: Images.wwix.src, hq: Images.wwix.hq, width: 256, height: 224, id: Constants.FIRST_PREVIEW_IMAGE_ID } as SimImage,
            { src: Images.seiken.src, hq: Images.seiken.hq, width: 256, height: 224 } as SimImage,
            { src: Images.sonicscroll.src, hq: Images.sonicscroll.hq, width: 320, height: 224 } as SimImage,
            { src: Images.metroid.src, hq: Images.metroid.hq, width: 256, height: 224 } as SimImage,
            { src: Images.tf4.src, hq: Images.tf4.hq, width: 320, height: 224 } as SimImage,
            { src: Images.dkc2.src, hq: Images.dkc2.hq, width: 256, height: 224 } as SimImage
        ],
        imageSelection: 0,
        visible: false,
        isRunningOnMobileDevice: mobileAndTabletCheck()
    };
}

export type LandingViewData = ReturnType<typeof data>;

export class LandingViewModel {
    private readonly _template: LandingTemplate;
    private readonly _state: LandingViewData;
    private readonly _navigator: Navigator;
    private readonly _visibility: Visibility;

    constructor (state: LandingViewData, template: LandingTemplate, navigator: Navigator, visibility: Visibility) {
        this._state = state;
        this._template = template;
        this._navigator = navigator;
        this._visibility = visibility;
    }

    static make (state: LandingViewData, template: LandingTemplate, navigator?: Navigator, visibility?: Visibility): LandingViewModel {
        return new LandingViewModel(state, template, navigator || Navigator.make(), visibility || Visibility.make());
    }

    turnVisibilityOn (): void {
        this._state.visible = true;
        this._template.refresh(this._state);
        this._visibility.hideLoading();
    }

    turnVisibilityOff (): void {
        this._visibility.showLoading();
        this._state.visible = false;
        this._template.refresh(this._state);
    }

    showError (message: string): void {
        this._navigator.openTopMessage(message);
    }

    selectImage (idx: number): void {
        this._state.imageSelection = idx;
        this._template.refresh(this._state);
    }

    addImage (image: SimImage): void {
        this._state.images.push(image);
        this.selectImage(this._state.images.length - 1);
    }
}