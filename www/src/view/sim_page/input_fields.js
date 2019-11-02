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

export default function (ctx) {
    [
        ctx.constants.featureChangeColorRepresentationDeo,
        ctx.constants.featureChangePixelGeometryDeo,
        ctx.constants.featureChangePixelShadowShapeDeo,
        ctx.constants.featureChangePixelShadowHeightDeo,
        ctx.constants.featureBacklightPercentDeo,
        ctx.constants.featureChangeScreenCurvatureDeo,
        ctx.constants.featureQuitDeo,
        ctx.constants.featureCaptureFramebufferDeo,
        ctx.constants.featureClosePanelDeo
    ].forEach(deo => {
        deo.onmousedown = () => document.dispatchEvent(new KeyboardEvent('keydown', { key: deo.id }));
        deo.onmouseup = () => document.dispatchEvent(new KeyboardEvent('keyup', { key: deo.id }));
    });
    
    [
        ctx.constants.resetCameraDeo,
        ctx.constants.resetSpeedsDeo,
        ctx.constants.resetFiltersDeo
    ].forEach(deo => {
        deo.onclick = () => document.dispatchEvent(new KeyboardEvent('keydown', { key: deo.id }));
    });
    
    ctx.root.querySelectorAll('.number-input').forEach(deo => {
        [{ button_text: '↑', mode: 'inc', placement: 'before' }, { button_text: '↓', mode: 'dec', placement: 'after' }].forEach(o => {
            const button = document.createElement('button');
            button.innerText = o.button_text;
            button.classList.add('button-inc-dec');
            const eventOptions = { key: deo.id + '-' + o.mode };
            button.onmousedown = () => document.dispatchEvent(new KeyboardEvent('keydown', eventOptions));
            button.onmouseup = () => document.dispatchEvent(new KeyboardEvent('keyup', eventOptions));
            deo.parentNode.insertBefore(button, o.placement === 'before' ? deo : deo.nextSibling);
        });
    });
    
    ctx.root.querySelectorAll('input').forEach(deo => {
        const eventOptions = { key: 'input_focused' };
        deo.addEventListener('focus', () => document.dispatchEvent(new KeyboardEvent('keydown', eventOptions)));
        deo.addEventListener('blur', () => document.dispatchEvent(new KeyboardEvent('keyup', eventOptions)));
    });    

}