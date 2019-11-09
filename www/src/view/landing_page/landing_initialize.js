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

import FontFaceObserver from 'fontfaceobserver';

import { Navigator } from '../../services/navigator';
import { mobileAndTabletCheck } from '../../services/utils';

import { playHtmlSelection, playQuerystring } from './play_simulation';

const navigator = Navigator.make();

export default function (ctx) {
    initializeHashtagHandling(ctx);
    initializeOptions(ctx);
    initializeSelectImage(ctx);

    Promise.all([
        new FontFaceObserver('Archivo Black', { weight: 400 }).load(null, 10000),
        new FontFaceObserver('Lato', { weight: 400 }).load(null, 10000),
        new FontFaceObserver('Lato', { weight: 700 }).load(null, 10000)
    ]).then(() => prepareLandingPage(ctx)).catch(e => {
        console.error(e);
        prepareLandingPage(ctx);
    });
}

let savedHash = '';
let hashNotChanged = false;
function initializeHashtagHandling (ctx) {
    window.onhashchange = () => {
        if (hashNotChanged) {
            hashNotChanged = false;
            return;
        }
        if (window.location.hash.length === 0) {
            hashNotChanged = true;
            window.location.hash = savedHash;
            return;
        }
        ctx.visibility.showLoading();
        navigator.goToLandingPage();
    };
}

function initializeOptions (ctx) {
    ctx.elements.optionScalingSelect.onchange = () => {
        if (ctx.elements.optionScalingSelect.value === ctx.constants.SCALING_CUSTOM_ID) {
            ctx.visibility.showScaleCustomInputs();
        } else {
            ctx.visibility.hideScaleCustomInputs();
        }
    };
    
    ctx.elements.restoreDefaultOptionsDeo.onclick = () => {
        ctx.store.removeAllOptions();
        loadInputValuesFromStorage();
    };
}

function initializeSelectImage (ctx) {
    window.ondrop = event => {
        event.preventDefault();
    };

    window.ondragover = event => {
        event.preventDefault();
        event.dataTransfer.dropEffect = 'none';
    };

    ctx.elements.inputFileUploadDeo.onchange = () => {
        const file = ctx.elements.inputFileUploadDeo.files[0];
        const url = (window.URL || window.webkitURL).createObjectURL(file);
        handleFileToUpload(ctx, url);
    };

    ctx.elements.dropZoneDeo.onclick = () => {
        ctx.elements.inputFileUploadDeo.click();
    };

    ctx.elements.dropZoneDeo.ondragover = event => {
        event.stopPropagation();
        event.preventDefault();
        event.dataTransfer.dropEffect = 'copy';
    };

    ctx.elements.dropZoneDeo.ondrop = event => {
        event.stopPropagation();
        event.preventDefault();
        var file = event.dataTransfer.files[0];
        const url = (window.URL || window.webkitURL).createObjectURL(file);
        handleFileToUpload(ctx, url);
    };

    ctx.root.querySelectorAll('.selectable-image').forEach(deo => {
        const img = deo.querySelector('img');
        img.isOptimizedAsset = true;
        makeImageSelectable(ctx, deo);
    });
}

const isRunningOnMobileDevice = mobileAndTabletCheck();
async function prepareLandingPage (ctx) {
    ctx.visibility.showLoading();
    
    if (window.location.hash.length > 1) {
        savedHash = window.location.hash;
        return playQuerystring(ctx, window.location.hash.substr(1));
    }

    loadInputValuesFromStorage(ctx);

    ctx.visibility.showUi();
    ctx.visibility.hideLoading();

    if (isRunningOnMobileDevice) {
        ctx.elements.startAnimationDeo.disabled = true;
        ctx.elements.startAnimationDeo.title = 'You need a PC with NVIDIA or ATI graphics card with updated drivers and a WebGL2 compatible browser (Firefox, Opera or Chrome) in order to run this without problems.';
        return;
    }

    await new Promise(resolve => {
        ctx.elements.startAnimationDeo.onclick = resolve;
    });

    ctx.visibility.hideUi();

    await playHtmlSelection(ctx);
}

function loadInputValuesFromStorage (ctx) {
    ctx.elements.optionScalingSelect.value = ctx.store.getScalingSelectOption();
    ctx.elements.optionPowerPreferenceSelect.value = ctx.store.getPowerPreferenceSelectOption();
    if (ctx.elements.optionScalingSelect.value === ctx.constants.SCALING_CUSTOM_ID) {
        ctx.visibility.showScaleCustomInputs();
    } else {
        ctx.visibility.hideScaleCustomInputs();
    }
    ctx.elements.scalingCustomResWidthDeo.value = ctx.store.getCustomResWidth();
    ctx.elements.scalingCustomResHeightDeo.value = ctx.store.getCustomResHeight();
    ctx.elements.scalingCustomArXDeo.value = ctx.store.getCustomArX();
    ctx.elements.scalingCustomArYDeo.value = ctx.store.getCustomArY();
    ctx.elements.scalingCustomStretchNearestDeo.checked = ctx.store.getCustomStretchNearest();
    ctx.elements.antialiasDeo.checked = ctx.store.getAntiAliasing();
}

function makeImageSelectable (ctx, deo) {
    deo.onclick = () => {
        ctx.elements.previewDeo.classList.remove('selected-image');
        ctx.elements.previewDeo = deo;
        ctx.elements.previewDeo.classList.add('selected-image');
    };
}

async function handleFileToUpload (ctx, url) {
    try {
        processFileToUpload(ctx, url);
    } catch (e) {
        console.error(e);
        navigator.openTopMessage('That file could not be loaded, try again with a picture.');
    }
}

async function processFileToUpload (ctx, url) {
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

    const width = img.width;
    const height = img.height;
    ctx.elements.scalingCustomResButtonDeo.value = 'Set to ' + width + ' ✕ ' + height;
    ctx.elements.scalingCustomResButtonDeo.onclick = () => {
        ctx.elements.scalingCustomResWidthDeo.value = width;
        ctx.elements.scalingCustomResHeightDeo.value = height;
    };
    const span = document.createElement('span');
    span.innerHTML = width + ' ✕ ' + height;
    const div = document.createElement('div');
    div.appendChild(img);
    div.appendChild(span);
    const li = document.createElement('li');
    li.classList.add('selectable-image');
    li.appendChild(div);
    makeImageSelectable(ctx, li);
    li.click();
    ctx.elements.selectImageList.insertBefore(li, ctx.elements.dropZoneDeo);
    ctx.visibility.showScalingCustomResButton();
}
