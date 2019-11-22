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

import { html, render } from 'lit-html';
import { ifDefined } from 'lit-html/directives/if-defined';

const css = require('!css-loader!./css/sim_page.css').toString();

export function renderTemplate (state, fire, root) {
    render(generateSimTemplate(state, fire), root);
}

function generateSimTemplate (state, fire) {
    return html`
        <style>
            ${css}
        </style>
        <div tabindex=0><canvas id="gl-canvas-id"></canvas></div>
        <div id="simulation-ui">
            <div id="fps-counter">${state.fps}</div>
            <div id="info-panel" class="${state.menu.visible ? '' : 'display-none'}">
                <div id="info-panel-content" class="${state.menu.open ? '' : 'display-none'}">
                    ${state.menu.entries.map(entry => generateTemplateFromGenericEntry(fire, entry))}
                </div>
                <div id="info-panel-toggle" 
                    class="collapse-button collapse-controller" 
                    @click="${() => fire('toggleControls')}">${state.menu.controlsText}</div>
            </div>
        </div>
    `;
}

function generateTemplateFromGenericEntry (fire, entry) {
    switch (entry.type) {
    case 'menu': return generateTemplateFromMenu(fire, entry);
    case 'preset-buttons': return generateTemplateFromPresetButtons(fire, entry);
    case 'scaling-input': return generateTemplateFromScalingInput(fire, entry);
    case 'checkbox-input': return generateTemplateFromCheckboxInput(fire, entry);
    case 'button-input': return generateTemplateFromButtonInput(fire, entry);
    case 'selectors-input': return generateTemplateFromSelectorsInput(fire, entry);
    case 'number-input': return generateTemplateFromNumberInput(fire, entry);
    case 'color-input': return generateTemplateFromColorInput(fire, entry);
    case 'camera-input': return generateTemplateFromCameraInput(fire, entry);
    default: throw new Error('Entry type ' + entry.type + ' not handled.');
    }
}

function generateTemplateFromMenu (fire, menu) {
    return html`
        <div class="collapse-button collapse-top-menu ${menu.open ? 'not-collapsed' : 'collapsed'}" @click="${() => fire('toggleMenu', menu)}">${menu.text}</div>
        <div class="info-category ${menu.open ? '' : 'display-none'}">
            ${menu.entries.map(entry => generateTemplateFromGenericEntry(fire, entry))}
        </div>
    `;
}

function generateTemplateFromScalingInput (fire, scalingInput) {
    return html`
    `;
}

function generateTemplateFromPresetButtons (fire, presetButtons) {
    return html`
        <div class="preset-list ${presetButtons.class}">
            ${presetButtons.ref.choices.map(choices => html`
                <a class="btn preset-btn ${presetButtons.ref.selected === choices.preset ? 'active-preset' : ''}" data-preset="${choices.preset}" href="#"
                    @click="${() => fire('clickPreset', choices.preset)}"
                    >${choices.text}</a>
            `)}
        </div>
    `;
}

function generateTemplateFromCheckboxInput (fire, checkboxInput) {
    return html`
        <div class="menu-entry menu-button ${checkboxInput.class}" @click="${() => fire('toggleCheckbox', checkboxInput.ref)}">
            <div class="feature-pack">
                <div class="feature-name">${checkboxInput.text}</div>
            </div>
            <div class="feature-value input-holder"><input type="checkbox" ?checked=${checkboxInput.ref.value}></div>
        </div>
    `;    
}

function generateTemplateFromButtonInput (fire, buttonInput) {
    return html`
        <div class="menu-entry menu-button ${buttonInput.class}" @click="${() => fire('dispatchKey', { action: 'keyboth', key: buttonInput.ref.eventKind })}">
            <div class="feature-pack">
                <div class="feature-name">${buttonInput.text}</div>
            </div>
            <div class="feature-value input-holder"><div></div></div>
        </div>
    `;
}

function generateTemplateFromSelectorsInput (fire, selectorInput) {
    return html`
        <div class="menu-entry ${selectorInput.class}">
            <div class="feature-pack">
                <div class="feature-name">${selectorInput.text}</div>
                ${selectorInput.hk ? html`<div class="feature-hotkeys">
                    <sup class="hotkey hk-inc" title="Press '${selectorInput.hk.inc}' to increse the value of this field">+: ${selectorInput.hk.inc}</sup>
                    <sup class="hotkey hk-dec" title="Press '${selectorInput.hk.inc}' to decrease the value of this field">-: ${selectorInput.hk.dec}</sup>
                </div>` : ''}
            </div>
            <div class="feature-value input-holder">
                <div class="selector-inc"
                    @mouseup="${e => { e.preventDefault(); fire('dispatchKey', { action: 'keyup', key: selectorInput.ref.eventKind + '-inc', current: selectorInput.ref.value }); }}"
                    @mousedown="${e => { e.preventDefault(); fire('dispatchKey', { action: 'keydown', key: selectorInput.ref.eventKind + '-inc', current: selectorInput.ref.value }); }}"
                    >
                    <input class="number-input feature-readonly-input" type="text"
                        title="${ifDefined(selectorInput.ref.title)}"
                        .value="${selectorInput.ref.value}"
                        >
                    <button class="button-inc-selector"
                        >+</button>
                </div>
                <button class="button-inc-dec"
                    @mouseup="${() => fire('dispatchKey', { action: 'keyup', key: selectorInput.ref.eventKind + '-dec', current: selectorInput.ref.value })}"
                    @mousedown="${() => fire('dispatchKey', { action: 'keydown', key: selectorInput.ref.eventKind + '-dec', current: selectorInput.ref.value })}"
                    >-</button>
            </div>
        </div>
    `;
}

function generateTemplateFromNumberInput (fire, numberInput) {
    return html`
        <div class="menu-entry ${numberInput.class}">
            <div class="feature-pack">
                <div class="feature-name">${numberInput.text}</div>
                <div class="feature-hotkeys">
                    <sup class="hotkey hk-inc" title="Press '${numberInput.hk.inc}' to increse the value of this field">+: ${numberInput.hk.inc}</sup>
                    <sup class="hotkey hk-dec" title="Press '${numberInput.hk.inc}' to decrease the value of this field">-: ${numberInput.hk.dec}</sup>
                </div>
            </div>
            <div class="feature-value input-holder">
                <input class="number-input feature-modificable-input" type="number" 
                    placeholder="${numberInput.placeholder}" step="${numberInput.step}" min="${numberInput.min}" max="${numberInput.max}" .value="${numberInput.ref.value}"
                    @focus="${() => fire('dispatchKey', { action: 'keydown', key: 'input_focused' })}"
                    @blur="${() => fire('dispatchKey', { action: 'keyup', key: 'input_focused' })}"
                    @keypress="${e => e.charCode === 13 /* ENTER */ && e.target.blur()}"
                    @change="${e => fire('changeSyncedInput', { value: e.target.value, kind: numberInput.ref.eventKind })}"
                    >
                <button class="button-inc-dec"
                    @mouseup="${() => fire('dispatchKey', { action: 'keyup', key: numberInput.ref.eventKind + '-inc' })}"
                    @mousedown="${() => fire('dispatchKey', { action: 'keydown', key: numberInput.ref.eventKind + '-inc' })}"
                    >+</button>
                <button class="button-inc-dec"
                    @mouseup="${() => fire('dispatchKey', { action: 'keyup', key: numberInput.ref.eventKind + '-dec' })}"
                    @mousedown="${() => fire('dispatchKey', { action: 'keydown', key: numberInput.ref.eventKind + '-dec' })}"
                    >-</button>
            </div>
        </div>
    `;
}

function generateTemplateFromColorInput (fire, colorInput) {
    return html`
        <div class="menu-entry ${colorInput.class}">
            <div class="feature-pack">
                <div class="feature-name">${colorInput.text}</div>
            </div>
            <div class="feature-value input-holder">
                <input class="feature-button" type="color" .value="${colorInput.ref.value}"
                    @change="${e => fire('changeSyncedInput', { value: parseInt('0x' + e.target.value.substring(1)), kind: colorInput.ref.eventKind })}"
                    >
            </div>
        </div>
    `;
}

function generateTemplateFromCameraInput (fire, cameraInput) {
    return html`
        <div class="menu-dual-entry-container">
            <div class="menu-dual-entry-item menu-dual-entry-1 ${cameraInput.class}">
                <div class="feature-name">Translation</div>
                <div id="feature-camera-movements" class="arrows-grid ${cameraInput.ref.free ? 'arrows-grid-move-free' : 'arrows-grid-move-lock'}">
                    <div></div><div class="input-cell">${generateTemplateArrowKey(fire, 'W')}</div><div></div><div></div><div class="input-cell">${cameraInput.ref.free ? generateTemplateArrowKey(fire, 'Q') : ''}</div>
                    <div class="input-cell">${generateTemplateArrowKey(fire, 'A')}</div><div class="input-cell">${generateTemplateArrowKey(fire, 'S')}</div><div class="input-cell">${generateTemplateArrowKey(fire, 'D')}</div><div></div><div>${cameraInput.ref.free ? generateTemplateArrowKey(fire, 'E') : ''}</div>
                </div>
            </div>
            <div class="menu-dual-entry-item menu-dual-entry-2">
                <div class="feature-name">Rotation</div>
                <div id="feature-camera-turns" class="arrows-grid arrows-grid-turn">
                        <div></div><div>${generateTemplateArrowKey(fire, '↑')}</div><div></div><div></div><div>${generateTemplateArrowKey(fire, '+')}</div><div class="rotator">⟳</div>
                        <div>${generateTemplateArrowKey(fire, '←')}</div><div>${generateTemplateArrowKey(fire, '↓')}</div><div>${generateTemplateArrowKey(fire, '→')}</div><div></div><div>${generateTemplateArrowKey(fire, '-')}</div><div class="rotator">⟲</div>
                </div>
            </div>
        </div>
        <div class="camera-matrix input-holder">
            <div class="matrix-row ${cameraInput.class}"></div><div class="matrix-top-row"><label class="text-center">X</label></div><div class="matrix-top-row"><label class="text-center">Y</label></div><div class="matrix-top-row"><label class="text-center">Z</label></div>
            <div class="matrix-row ${cameraInput.class}"><div class="matrix-row-head">positon</div></div>
                ${[cameraInput.ref.pos.x, cameraInput.ref.pos.y, cameraInput.ref.pos.z].map(ref => generateTemplateForCameraMatrixInput(fire, ref))}
            <div class="matrix-row ${cameraInput.class}"><div class="matrix-row-head">direction</div></div>
                ${[cameraInput.ref.dir.x, cameraInput.ref.dir.y, cameraInput.ref.dir.z].map(ref => generateTemplateForCameraMatrixInput(fire, ref))}
            <div class="matrix-row ${cameraInput.class}"><div class="matrix-row-head">axis up</div></div>
                ${[cameraInput.ref.axis_up.x, cameraInput.ref.axis_up.y, cameraInput.ref.axis_up.z].map(ref => generateTemplateForCameraMatrixInput(fire, ref))}
        </div>
    `;
}

function generateTemplateArrowKey (fire, key) {
    return html`
        <input type="button" class="activate-button feature-modificable-input" value="${key}"
            @mousedown="${() => fire('dispatchKey', { action: 'keydown', key: key.toLowerCase() })}"
            @mouseup="${() => fire('dispatchKey', { action: 'keyup', key: key.toLowerCase() })}"
        >
    `;
}

function generateTemplateForCameraMatrixInput (fire, ref) {
    return html`
        <div class="input-cell">
            <input class="feature-modificable-input" type="number" step="0.01" .value="${ref.value}"
                @change="${e => fire('changeSyncedInput', { value: +e.target.value, kind: ref.eventKind })}"
                @focus="${() => fire('dispatchKey', { action: 'keydown', key: 'input_focused' })}"
                @blur="${() => fire('dispatchKey', { action: 'keyup', key: 'input_focused' })}"
                @keypress="${e => e.charCode === 13 /* ENTER */ && e.target.blur()}"
                >
        </div>
    `;
}