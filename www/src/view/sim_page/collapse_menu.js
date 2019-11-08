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
    ctx.root.querySelectorAll('.collapse-button').forEach(deo => {
        const open = deo.dataset.openText;
        const close = deo.dataset.closeText;
        const target = ctx.root.getElementById(deo.dataset.collapseTarget);
        deo.onclick = () => {
            if (target.classList.contains('display-none')) {
                target.classList.remove('display-none');
                deo.classList.remove('collapsed');
                deo.classList.add('not-collapsed');
                if (close) {
                    deo.innerText = close;
                }
            } else {
                target.classList.add('display-none');
                deo.classList.add('collapsed');
                deo.classList.remove('not-collapsed');
                if (open) {
                    deo.innerText = open;
                }
            }
        };
        deo.click();
        deo.click();
    });

    ctx.root.querySelectorAll('.number-input').forEach(deo => {
        let button;
        [{ button_text: '↓', mode: 'dec', placement: 'after' }, { button_text: '↑', mode: 'inc', placement: 'before' }].forEach(o => {
            button = document.createElement('button');
            button.innerText = o.button_text;
            button.classList.add('button-inc-dec');
            const eventOptions = { key: deo.id + '-' + o.mode };
            button.onmousedown = () => document.dispatchEvent(new KeyboardEvent('keydown', eventOptions));
            button.onmouseup = () => document.dispatchEvent(new KeyboardEvent('keyup', eventOptions));
            deo.parentNode.insertBefore(button, o.placement === 'before' ? deo : deo.nextSibling);
        });
        if (deo.classList.contains('feature-readonly-input')) {
            deo.onmousedown = e => { e.preventDefault(); document.dispatchEvent(new KeyboardEvent('keydown', { key: deo.id + '-inc' })); };
            deo.onmouseup = e => { e.preventDefault(); document.dispatchEvent(new KeyboardEvent('keyup', { key: deo.id + '-inc' })); };
            deo.onmouseenter = () => button.classList.add('hover');
            deo.onmouseleave = () => button.classList.remove('hover');
            button.onmouseenter = () => deo.classList.add('hover');
            button.onmouseleave = () => deo.classList.remove('hover');
        } else {
            deo.onkeypress = e => e.charCode === 13 /* ENTER */ && deo.blur();
        }
    });
    
    ctx.root.querySelectorAll('input[type="number"], input[type="text"]').forEach(deo => {
        const eventOptions = { key: 'input_focused' };
        deo.addEventListener('focus', () => document.dispatchEvent(new KeyboardEvent('keydown', eventOptions)));
        deo.addEventListener('blur', () => document.dispatchEvent(new KeyboardEvent('keyup', eventOptions)));
    });

    ctx.root.querySelectorAll('.hk-inc').forEach(deo => deo.setAttribute('title', 'Press \'' + deo.innerText + '\' to increse the value of this field'));
    ctx.root.querySelectorAll('.hk-dec').forEach(deo => deo.setAttribute('title', 'Press \'' + deo.innerText + '\' to decrease the value of this field'));
    
    ctx.root.querySelectorAll('.menu-button').forEach(deo => {
        deo.onclick = () => document.dispatchEvent(new KeyboardEvent('keydown', { key: deo.id }));
    });
}
