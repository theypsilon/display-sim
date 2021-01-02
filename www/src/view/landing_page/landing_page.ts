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

import { LandingTemplate, actions, LandingTemplateEvents} from './landing_template';
import { playHtmlSelection, playQuerystring } from './play_simulation';

import {data, LandingViewModel, SimImage, LandingViewData} from './landing_view_model';

class LandingPage extends HTMLElement {
    constructor () {
        super();
        setupPage(this.attachShadow({ mode: 'open' }))
            .catch(e => console.error(e));
    }
}

window.customElements.define('landing-page', LandingPage);

const state = data();
const events = actions();

async function setupPage (root: ShadowRoot) {
    if (window.location.hash.length > 1) {
        return playQuerystring(window.location.hash.substr(1));
    }
    const template = new LandingTemplate(root, events);
    const view_model = LandingViewModel.make(state, template);
    await show(state, events, view_model);
}

async function show (state: LandingViewData, events: LandingTemplateEvents, view_model: LandingViewModel) {
    view_model.turnVisibilityOn();

    events.addImage.subscribe(async file => await uploadFile(file)
        .then(view_model.addImage)
        .catch(e => {
            view_model.showError('That file could not be loaded, try again with a picture.');
            console.error(e);
        })
    );

    events.selectImage.subscribe(view_model.selectImage);

    events.clickPlaySimulation.subscribe(async () => {
        view_model.turnVisibilityOff();
        await playHtmlSelection(state);
    })
}

function uploadFile (file: File): Promise<SimImage> {
    const url = (window.URL || window.webkitURL).createObjectURL(file);
    return loadImageFromUrl(url);
}

async function loadImageFromUrl (url: string): Promise<SimImage> {
    let xhr = new XMLHttpRequest();
    await new Promise((resolve, reject) => {
        xhr.onload = resolve;
        xhr.onerror = reject;
        xhr.open('GET', url, true);
        xhr.responseType = 'blob';
        xhr.send(null);
    });

    const previewUrl = URL.createObjectURL(xhr.response);
    const img = new Image() as HTMLImageElement & {isGif: boolean};
    await new Promise((resolve, reject) => {
        img.onload = resolve;
        img.onerror = reject;
        img.setAttribute('crossOrigin', '');
        img.src = previewUrl;
    });

    img.isGif = xhr.response.type === 'image/gif';

    return { width: img.width, height: img.height, src: previewUrl, hq: previewUrl, img };
}