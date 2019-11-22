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

import { Observer } from '../../services/observer';

import { renderTemplate } from './landing_template';
import { playHtmlSelection, playQuerystring } from './play_simulation';

import { data, View } from './landing_view_model';

const state = data();

class LandingPage extends HTMLElement {
    constructor () {
        super();
        setupPage(this.attachShadow({ mode: 'open' }), state, Observer.make());
    }
}

window.customElements.define('landing-page', LandingPage);

function setupPage (root, state, observer) {
    if (window.location.hash.length > 1) {
        playQuerystring(window.location.hash.substr(1));
        return;
    }

    const view = View.make(state, () => renderTemplate(state, (type, message) => observer.fire({ type, message }), root));
    view.turnVisibilityOn();

    observer.subscribe(e => {
        const msg = e.message;
        switch (e.type) {
        case 'changed-file-input': {
            uploadFile(msg.target.files[0])
                .then(image => view.addImage(image))
                .catch(e => {
                    console.error(e);
                    view.showError('That file could not be loaded, try again with a picture.');
                });
            break;
        }
        case 'select-image': return view.selectImage(msg);
        case 'click-drop-zone': return root.getElementById('file').click();
        case 'drop-on-drop-zone': {
            msg.stopPropagation();
            msg.preventDefault();
            uploadFile(msg.dataTransfer.files[0])
                .then(image => view.addImage(image))
                .catch(e => {
                    console.error(e);
                    view.showError('That file could not be loaded, try again with a picture.');
                });
            break;
        }
        case 'drag-over-drop-zone': {
            msg.stopPropagation();
            msg.preventDefault();
            msg.dataTransfer.dropEffect = 'copy';
            break;
        }
        case 'select-scaling': return view.selectScaling(msg.target.value);
        case 'click-play-simulation': {
            view.turnVisibilityOff();
            playHtmlSelection(state);
            break;
        }
        default: throw new Error('Not covered following event: ', e.type, e);
        }
    });
}

function uploadFile (file) {
    const url = (window.URL || window.webkitURL).createObjectURL(file);
    return loadImageFromUrl(url);
}

async function loadImageFromUrl (url) {
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

    return { width: img.width, height: img.height, src: previewUrl, hq: previewUrl, img };
}