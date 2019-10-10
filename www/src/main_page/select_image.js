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

import Globals from '../globals';

import { makeVisibility } from '../visibility';

const visibility = makeVisibility();

let previewDeo = document.getElementById(Globals.firstPreviewImageId);

window.ondrop = event => {
    event.preventDefault();
};

window.ondragover = event => {
    event.preventDefault();
    event.dataTransfer.dropEffect = 'none';
};

Globals.inputFileUploadDeo.onchange = () => {
    const file = Globals.inputFileUploadDeo.files[0];
    const url = (window.URL || window.webkitURL).createObjectURL(file);
    processFileToUpload(url);
};

Globals.dropZoneDeo.onclick = () => {
    Globals.inputFileUploadDeo.click();
};

Globals.dropZoneDeo.ondragover = event => {
    event.stopPropagation();
    event.preventDefault();
    event.dataTransfer.dropEffect = 'copy';
};

Globals.dropZoneDeo.ondrop = event => {
    event.stopPropagation();
    event.preventDefault();
    var file = event.dataTransfer.files[0];
    const url = (window.URL || window.webkitURL).createObjectURL(file);
    processFileToUpload(url);
};

document.querySelectorAll('.selectable-image').forEach(deo => {
    const img = deo.querySelector('img');
    img.isGif = img.src.includes('.gif');
    img.isAsset = true;
    makeImageSelectable(deo);
});

function makeImageSelectable (deo) {
    deo.onclick = () => {
        previewDeo.classList.remove('selected-image');
        previewDeo = deo;
        previewDeo.classList.add('selected-image');
    };
}

async function processFileToUpload (url) {
    var xhr = new XMLHttpRequest();
    xhr.open('GET', url, true);
    xhr.responseType = 'blob';
    xhr.send(null);

    await new Promise(resolve => {
        xhr.onload = () => resolve();
    });

    const previewUrl = URL.createObjectURL(xhr.response);
    const img = new Image();
    img.src = previewUrl;

    await new Promise((resolve, reject) => {
        img.onload = resolve;
        img.onerror = reject;
    });

    img.isGif = xhr.response.type === 'image/gif';

    const width = img.width;
    const height = img.height;
    Globals.scalingCustomResButtonDeo.value = 'Set to ' + width + ' ✕ ' + height;
    Globals.scalingCustomResButtonDeo.onclick = () => {
        Globals.scalingCustomResWidthDeo.value = width;
        Globals.scalingCustomResHeightDeo.value = height;
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
    Globals.selectImageList.insertBefore(li, Globals.dropZoneDeo);
    visibility.showScalingCustomResButton();
}
