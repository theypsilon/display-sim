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

import {html, render, TemplateResult} from 'lit-html';
import { ifDefined } from 'lit-html/directives/if-defined';
import {
    ButtonInputEntry, CameraInputEntry,
    CheckboxInputEntry, ColorInputEntry,
    SimViewEntry,
    HalfPair,
    MenuEntry, NumberInputEntry,
    NumericPairEntry,
    PresetButtonsEntry, Ref, RgbInputEntry,
    ScalingInputEntry, SelectorsInput,
    SimViewData
} from "./sim_view_model";
import {throwOnNull} from "../../services/guards";

const css = require('!css-loader!./css/sim_page.css').default.toString();

interface Fire {
    (key: string, arg?: any): void;
}

export class SimTemplate
{
    private readonly _root: ShadowRoot;
    private readonly _fire: Fire;
    private _rendered: boolean;

    private constructor(root: ShadowRoot, fire: Fire) {
        this._root = root;
        this._fire = fire;
        this._rendered = false;
    }

    static make(root: ShadowRoot, fire: Fire) {
        return new SimTemplate(root, fire);
    }

    getCanvas(state: SimViewData): HTMLCanvasElement {
        if (this._rendered!) {
            this.refresh(state);
        }
        return throwOnNull(this._root.getElementById('gl-canvas-id') as HTMLCanvasElement | null);
    }

    refresh(state: SimViewData): void {
        this._rendered = true;
        render(this.generateSimTemplate(state), this._root);
    }

    private toggleControls() {
        this._fire('toggleControls');
    }

    private toggleMenu(msg: MenuEntry) {
        this._fire('toggleMenu', msg);
    }

    private dispatchKey(msg: {action: 'keyup' | 'keydown' | 'keyboth', key: string, current?: string}) {
        this._fire('dispatchKey', msg);
    }

    private changeSyncedInput(msg: { kind: string; value: number }) {
        this._fire('changeSyncedInput', msg);
    }

    private clickPreset(msg: string) {
        this._fire('clickPreset', msg);
    }

    private toggleCheckbox(msg: { kind: string, value: boolean }) {
        this._fire('toggleCheckbox', msg);
    }

    private generateSimTemplate (state: SimViewData) {
        return html`
        <style>
            ${css}
        </style>
        <div tabindex=0><canvas id="gl-canvas-id"></canvas></div>
        <div id="simulation-ui">
            <div id="fps-counter">${state.fps}</div>
            <div id="info-panel" class="${state.menu.visible ? '' : 'display-none'}">
                <div id="info-panel-content" class="${state.menu.open ? '' : 'display-none'}">
                    ${state.menu.entries.map(entry => this.generateTemplateFromGenericEntry(entry))}
                </div>
                <div id="info-panel-toggle" 
                    class="collapse-button collapse-controller" 
                    @click="${() => this.toggleControls()}">${state.menu.controlsText}</div>
            </div>
        </div>
    `;
}

    private generateTemplateFromGenericEntry (entry: SimViewEntry) {
    switch (entry.type) {
        case 'menu': return this.generateTemplateFromMenu(entry);
        case 'preset-buttons': return this.generateTemplateFromPresetButtons(entry);
        case 'scaling-input': return this.generateTemplateFromScalingInput(entry);
        case 'checkbox-input': return this.generateTemplateFromCheckboxInput(entry);
        case 'button-input': return this.generateTemplateFromButtonInput(entry);
        case 'selectors-input': return this.generateTemplateFromSelectorsInput(entry);
        case 'numeric-pair': return this.generateTemplateFromNumericPair(entry);
        case 'number-input': return this.generateTemplateFromNumberInput(entry);
        case 'color-input': return this.generateTemplateFromColorInput(entry);
        case 'camera-input': return this.generateTemplateFromCameraInput(entry);
        case 'rgb-input': return this.generateTemplateFromRgbInput(entry);
    }
}

    private generateTemplateFromMenu (menu: MenuEntry): TemplateResult {
    return html`
        <div class="collapse-button collapse-top-menu ${menu.open ? 'not-collapsed' : 'collapsed'}" @click="${() => this.toggleMenu(menu)}">${menu.text}</div>
        <div class="info-category ${menu.open ? '' : 'display-none'}">
            ${menu.entries.map(entry => this.generateTemplateFromGenericEntry(entry))}
        </div>
    `;
}

    private generateTemplateFromScalingInput (scalingInput: ScalingInputEntry): TemplateResult {
    return html`
        <div class="scaling-input-opaque ${scalingInput.ref.value !== 'Custom' ? '' : 'display-none'}"
            title="For manually changing these values, select Custom as scaling method."
            ></div>
        <div>
            ${scalingInput.entries.map(entry => this.generateTemplateFromGenericEntry(entry))}
        </div>
    `;
}

    private generateTemplateFromNumericPair (numericPair: NumericPairEntry) {
    return html`
        <div class="menu-entry ${numericPair.class}">
            <div class="feature-pack"><div class="feature-name">${numericPair.text}</div></div>
            <div class="feature-value input-holder">
                ${this.generateTemplateFromHalfPair(numericPair.pair[0])}
                ${numericPair.separator}
                ${this.generateTemplateFromHalfPair(numericPair.pair[1])}
            </div>
        </div>
    `;
}

    private generateTemplateFromHalfPair (halfPair: HalfPair) {
    return html`
        <div class="half-numeric-container">
            <input class="number-input feature-modificable-input half-numeric-input" type="number" 
                placeholder="${halfPair.placeholder}" step="${halfPair.step}" min="${halfPair.min}" max="${halfPair.max}" .value="${halfPair.ref.value}"
                @focus="${() => this.dispatchKey({ action: 'keydown', key: 'input_focused' })}"
                @blur="${() => this.dispatchKey({ action: 'keyup', key: 'input_focused' })}"
                @keypress="${(e: KeyboardEvent) => e.charCode === 13 /* ENTER */ && (<HTMLInputElement>e.target).blur()}"
                @change="${(e: KeyboardEvent) => this.changeSyncedInput({value: +(<HTMLInputElement>e.target).value, kind: halfPair.ref.eventKind})}"
                >
            <button class="button-inc-dec"
                @mouseup="${() => this.dispatchKey({ action: 'keyup', key: halfPair.ref.eventKind + '-inc' })}"
                @mousedown="${() => this.dispatchKey({ action: 'keydown', key: halfPair.ref.eventKind + '-inc' })}"
                >+</button>
            <button class="button-inc-dec"
                @mouseup="${() => this.dispatchKey({ action: 'keyup', key: halfPair.ref.eventKind + '-dec' })}"
                @mousedown="${() => this.dispatchKey({ action: 'keydown', key: halfPair.ref.eventKind + '-dec' })}"
                >-</button>
        </div>
    `;
}

    private generateTemplateFromPresetButtons (presetButtons: PresetButtonsEntry) {
    return html`
        <div class="preset-list ${presetButtons.class}">
            ${presetButtons.ref.choices.map(choices => html`
                <a class="btn preset-btn ${presetButtons.ref.selected === choices.preset ? 'active-preset' : ''}" data-preset="${choices.preset}" href="#"
                    @click="${() => this.clickPreset(choices.preset)}"
                    >${choices.text}</a>
            `)}
        </div>
    `;
}

    private generateTemplateFromCheckboxInput (checkboxInput: CheckboxInputEntry) {
    return html`
        <div class="menu-entry menu-button ${checkboxInput.class}"
            @click="${() => this.toggleCheckbox({ value: !checkboxInput.ref.value, kind: checkboxInput.ref.eventKind })}">
            <div class="feature-pack">
                <div class="feature-name">${checkboxInput.text}</div>
            </div>
            <div class="feature-value input-holder"><input type="checkbox" ?checked=${checkboxInput.ref.value}></div>
        </div>
    `;
}

    private generateTemplateFromButtonInput (buttonInput: ButtonInputEntry) {
    return html`
        <div class="menu-entry menu-button ${buttonInput.class}" @click="${() => this.dispatchKey({ action: 'keyboth', key: buttonInput.ref.eventKind })}">
            <div class="feature-pack">
                <div class="feature-name">${buttonInput.text}</div>
            </div>
            <div class="feature-value input-holder"><div></div></div>
        </div>
    `;
}

    private generateTemplateFromSelectorsInput (selectorInput: SelectorsInput) {
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
                    @mouseup="${(e: Event) => { e.preventDefault(); this.dispatchKey({ action: 'keyup', key: selectorInput.ref.eventKind + '-inc', current: selectorInput.ref.value }); }}"
                    @mousedown="${(e: Event) => { e.preventDefault(); this.dispatchKey({ action: 'keydown', key: selectorInput.ref.eventKind + '-inc', current: selectorInput.ref.value }); }}"
                    >
                    <input class="number-input feature-readonly-input" type="text"
                        title="${ifDefined(selectorInput.ref.title)}"
                        .value="${selectorInput.ref.value}"
                        >
                    <button class="button-inc-selector"
                        >+</button>
                </div>
                <button class="button-inc-dec"
                    @mouseup="${() => this.dispatchKey({ action: 'keyup', key: selectorInput.ref.eventKind + '-dec', current: selectorInput.ref.value })}"
                    @mousedown="${() => this.dispatchKey({ action: 'keydown', key: selectorInput.ref.eventKind + '-dec', current: selectorInput.ref.value })}"
                    >-</button>
            </div>
        </div>
    `;
}

    private generateTemplateFromNumberInput (numberInput: NumberInputEntry) {
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
                    @focus="${() => this.dispatchKey({ action: 'keydown', key: 'input_focused' })}"
                    @blur="${() => this.dispatchKey({ action: 'keyup', key: 'input_focused' })}"
                    @keypress="${(e: KeyboardEvent) => e.charCode === 13 /* ENTER */ && (<HTMLInputElement>e.target).blur()}"
                    @change="${(e: Event) => this.changeSyncedInput({ value: +(<HTMLInputElement>e.target).value, kind: numberInput.ref.eventKind })}"
                    >
                <button class="button-inc-dec"
                    @mouseup="${() => this.dispatchKey({ action: 'keyup', key: numberInput.ref.eventKind + '-inc' })}"
                    @mousedown="${() => this.dispatchKey({ action: 'keydown', key: numberInput.ref.eventKind + '-inc' })}"
                    >+</button>
                <button class="button-inc-dec"
                    @mouseup="${() => this.dispatchKey({ action: 'keyup', key: numberInput.ref.eventKind + '-dec' })}"
                    @mousedown="${() => this.dispatchKey({ action: 'keydown', key: numberInput.ref.eventKind + '-dec' })}"
                    >-</button>
            </div>
        </div>
    `;
}

    private generateTemplateFromColorInput (colorInput: ColorInputEntry) {
    return html`
        <div class="menu-entry ${colorInput.class}">
            <div class="feature-pack">
                <div class="feature-name">${colorInput.text}</div>
            </div>
            <div class="feature-value input-holder">
                <input class="feature-button" type="color" .value="${colorInput.ref.value}"
                    @change="${(e: Event) => this.changeSyncedInput({value: parseInt('0x' + (<HTMLInputElement>e.target).value.substring(1)), kind: colorInput.ref.eventKind})}"
                    >
            </div>
        </div>
    `;
}

    private generateTemplateFromCameraInput (cameraInput: CameraInputEntry) {
    return html`
        <div class="menu-dual-entry-container">
            <div class="menu-dual-entry-item menu-dual-entry-1 ${cameraInput.class}">
                <div class="feature-name">Translation</div>
                <div id="feature-camera-movements" class="arrows-grid ${cameraInput.ref.lockMode ? 'arrows-grid-move-free' : 'arrows-grid-move-lock'}">
                    <div></div><div class="input-cell">${this.generateTemplateArrowKey('W')}</div><div></div><div></div><div class="input-cell">${cameraInput.ref.lockMode ? this.generateTemplateArrowKey('Q') : ''}</div>
                    <div class="input-cell">${this.generateTemplateArrowKey('A')}</div><div class="input-cell">${this.generateTemplateArrowKey('S')}</div><div class="input-cell">${this.generateTemplateArrowKey('D')}</div><div></div><div>${cameraInput.ref.lockMode ? this.generateTemplateArrowKey('E') : ''}</div>
                </div>
            </div>
            <div class="menu-dual-entry-item menu-dual-entry-2">
                <div class="feature-name">Rotation</div>
                <div id="feature-camera-turns" class="arrows-grid arrows-grid-turn">
                        <div></div><div>${this.generateTemplateArrowKey('↑')}</div><div></div><div></div><div>${this.generateTemplateArrowKey('+')}</div><div class="rotator">⟳</div>
                        <div>${this.generateTemplateArrowKey('←')}</div><div>${this.generateTemplateArrowKey('↓')}</div><div>${this.generateTemplateArrowKey('→')}</div><div></div><div>${this.generateTemplateArrowKey('-')}</div><div class="rotator">⟲</div>
                </div>
            </div>
        </div>
        <div class="camera-matrix input-holder">
            <div class="matrix-row ${cameraInput.class}"></div><div class="matrix-top-row"><label class="text-center">X</label></div><div class="matrix-top-row"><label class="text-center">Y</label></div><div class="matrix-top-row"><label class="text-center">Z</label></div>
            <div class="matrix-row ${cameraInput.class}"><div class="matrix-row-head">positon</div></div>
                ${[cameraInput.ref.pos.x, cameraInput.ref.pos.y, cameraInput.ref.pos.z].map(ref => this.generateTemplateForCameraMatrixInput(ref))}
            <div class="matrix-row ${cameraInput.class}"><div class="matrix-row-head">direction</div></div>
                ${[cameraInput.ref.dir.x, cameraInput.ref.dir.y, cameraInput.ref.dir.z].map(ref => this.generateTemplateForCameraMatrixInput(ref))}
            <div class="matrix-row ${cameraInput.class}"><div class="matrix-row-head">axis up</div></div>
                ${[cameraInput.ref.axis_up.x, cameraInput.ref.axis_up.y, cameraInput.ref.axis_up.z].map(ref => this.generateTemplateForCameraMatrixInput(ref))}
        </div>
    `;
}

    private generateTemplateFromRgbInput (rgb: RgbInputEntry) {
    return html`
        <div class="camera-matrix input-holder">
            <div class="matrix-row ${rgb.class}"></div><div class="matrix-top-row"><label class="text-center">R</label></div><div class="matrix-top-row"><label class="text-center">G</label></div><div class="matrix-top-row"><label class="text-center">B</label></div>
            <div class="matrix-row ${rgb.class}"><div class="matrix-row-head">red</div></div>
                ${[rgb.ref.red.r, rgb.ref.red.g, rgb.ref.red.b].map(ref => this.generateTemplateForCameraMatrixInput(ref))}
            <div class="matrix-row ${rgb.class}"><div class="matrix-row-head">green</div></div>
                ${[rgb.ref.green.r, rgb.ref.green.g, rgb.ref.green.b].map(ref => this.generateTemplateForCameraMatrixInput(ref))}
            <div class="matrix-row ${rgb.class}"><div class="matrix-row-head">blue</div></div>
                ${[rgb.ref.blue.r, rgb.ref.blue.g, rgb.ref.blue.b].map(ref => this.generateTemplateForCameraMatrixInput(ref))}
        </div>
    `;
}

    private generateTemplateArrowKey (key: string) {
    return html`
        <input type="button" class="activate-button feature-modificable-input" value="${key}"
            @mousedown="${() => this.dispatchKey({ action: 'keydown', key: key.toLowerCase() })}"
            @mouseup="${() => this.dispatchKey({ action: 'keyup', key: key.toLowerCase() })}"
        >
    `;
}

    private generateTemplateForCameraMatrixInput (ref: Ref<number>) {
    return html`
        <div class="input-cell">
            <input class="feature-modificable-input" type="number" step="0.01" .value="${ref.value}"
                @change="${(e: KeyboardEvent) => this.changeSyncedInput({value: +(<HTMLInputElement>e.target).value, kind: ref.eventKind})}"
                @focus="${() => this.dispatchKey({ action: 'keydown', key: 'input_focused' })}"
                @blur="${() => this.dispatchKey({ action: 'keyup', key: 'input_focused' })}"
                @keypress="${(e: KeyboardEvent) => e.charCode === 13 /* ENTER */ && (<HTMLInputElement>e.target).blur()}"
                >
        </div>
    `;
}
}