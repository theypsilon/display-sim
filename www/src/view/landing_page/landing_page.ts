/* Copyright (c) 2019-2024 José manuel Barroso Galindo <theypsilon@gmail.com>
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
import { Navigator } from '../../services/navigator';
import { Visibility } from '../../services/visibility';
import { Disposable } from '../../services/disposable';

import {data, LandingViewModel, LandingViewData} from './landing_view_model';
import {SimImage} from "../../services/images";

class LandingPage extends HTMLElement {
    private _future: Promise<Disposable | void>;

    constructor () {
        super();
        this._future = setupPage(this.attachShadow({ mode: 'open' }))
            .catch(e => console.error(e));
    }

    disconnectedCallback () {
        this._future.then(mess => mess && mess.dispose());
    }
}

window.customElements.define('landing-page', LandingPage);

const state = data();
const events = actions();

async function setupPage (root: ShadowRoot): Promise<Disposable | void> {
    if (window.location.hash.length > 1) {
        return playQuerystring(window.location.hash.substr(1))
            .catch(e => {
                console.error(e);
                Visibility.make().hideLoading();
                Navigator.make().openTopMessage('The simulation could not be started, try again.');
            });
    }
    const template = LandingTemplate.make(root, events);
    const view_model = LandingViewModel.make(state, template);
    return show(state, events, view_model);
}

async function show (state: LandingViewData, events: LandingTemplateEvents, view_model: LandingViewModel): Promise<Disposable> {
    view_model.turnVisibilityOn();

    const subscriptions = [
        events.addImage.subscribe(async file => await uploadFile(file)
            .then(img => view_model.addImage(img))
            .catch(e => {
                view_model.showError('That file could not be loaded, try again with a picture.');
                console.error(e);
            })
        ),
        events.selectImage.subscribe(n => view_model.selectImage(n)),
        events.clickPlaySimulation.subscribe(async () => {
            view_model.turnVisibilityOff();
            await playHtmlSelection(state)
                .catch(e => {
                    console.error(e);
                    view_model.turnVisibilityOn();
                    view_model.showError('The simulation could not be started, try again.');
                });
        })
    ];

    return Disposable.make(() => subscriptions.forEach(subscription => subscription.dispose()));
}

function uploadFile (file: File): Promise<SimImage> {
    const url = (window.URL || window.webkitURL).createObjectURL(file);
    return loadImageFromUrl(url)
        .finally(() => URL.revokeObjectURL(url));
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