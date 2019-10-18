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

import Constants from '../constants';
import GlobalState from '../global_state';

import { makeVisibility } from '../visibility';

const visibility = makeVisibility();

window.ondrop = event => {
    event.preventDefault();
};

window.ondragover = event => {
    event.preventDefault();
    event.dataTransfer.dropEffect = 'none';
};

Constants.inputFileUploadDeo.onchange = () => {
    const file = Constants.inputFileUploadDeo.files[0];
    const url = (window.URL || window.webkitURL).createObjectURL(file);
    handleFileToUpload(url);
};

Constants.dropZoneDeo.onclick = () => {
    Constants.inputFileUploadDeo.click();
};

Constants.dropZoneDeo.ondragover = event => {
    event.stopPropagation();
    event.preventDefault();
    event.dataTransfer.dropEffect = 'copy';
};

Constants.dropZoneDeo.ondrop = event => {
    event.stopPropagation();
    event.preventDefault();
    var file = event.dataTransfer.files[0];
    const url = (window.URL || window.webkitURL).createObjectURL(file);
    handleFileToUpload(url);
};

document.querySelectorAll('.selectable-image').forEach(deo => {
    const img = deo.querySelector('img');
    img.isOptimizedAsset = true;
    makeImageSelectable(deo);
});

function makeImageSelectable (deo) {
    deo.onclick = () => {
        GlobalState.previewDeo.classList.remove('selected-image');
        GlobalState.previewDeo = deo;
        GlobalState.previewDeo.classList.add('selected-image');
    };
}

async function handleFileToUpload (url) {
    try {
        processFileToUpload(url);
    } catch (e) {
        console.error(e);
        window.dispatchEvent(new CustomEvent('app-event.top_message', {
            detail: 'That file could not be loaded, try again with a picture.'
        }));
    }
}

async function processFileToUpload (url) {
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
        img.src = previewUrl;
    });

    img.isGif = xhr.response.type === 'image/gif';

    const width = img.width;
    const height = img.height;
    Constants.scalingCustomResButtonDeo.value = 'Set to ' + width + ' ✕ ' + height;
    Constants.scalingCustomResButtonDeo.onclick = () => {
        Constants.scalingCustomResWidthDeo.value = width;
        Constants.scalingCustomResHeightDeo.value = height;
    };
    const span = document.createElement('span');
    span.innerHTML = width + ' ✕ ' + height;
    const div = document.createElement('div');
    div.appendChild(img);
    div.appendChild(span);
    const li = document.createElement('li');
    li.classList.add('selectable-image');
    li.appendChild(div);
    makeImageSelectable(li);
    li.click();
    Constants.selectImageList.insertBefore(li, Constants.dropZoneDeo);
    visibility.showScalingCustomResButton();
}
