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

import { renderTemplate, actions } from './landing_template';
import { playHtmlSelection, playQuerystring } from './play_simulation';

import {data, View, SimImage } from './landing_view_model';
import {DataEvent, FileEvent} from '../../services/event_types';

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

    const view = View.make(state, () => renderTemplate(state, events, root));
    view.turnVisibilityOn();

    events.changedFileInput.subscribe(async (e: FileEvent) => {
        const image = await uploadFile(e.target.files[0]).catch(e => {
            view.showError('That file could not be loaded, try again with a picture.');
            throw e;
        });
        view.addImage(image);
    });

    events.selectImage.subscribe((e: number) => view.selectImage(e));

    events.clickOnDropZone.subscribe(_ => root.getElementById('file')?.click())

    events.dropOnDropZone.subscribe(async (e: DataEvent) => {
        e.stopPropagation();
        e.preventDefault();
        const image = await uploadFile(e.dataTransfer.files[0]).catch(e => {
            view.showError('That file could not be loaded, try again with a picture.');
            throw e;
        });
        view.addImage(image);
    });

    events.dragOverDropZone.subscribe((e: DataEvent) => {
        e.stopPropagation();
        e.preventDefault();
        e.dataTransfer.dropEffect = 'copy';
    })

    events.clickPlaySimulation.subscribe(async () => {
        view.turnVisibilityOff();
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