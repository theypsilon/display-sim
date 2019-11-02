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
    window.addEventListener(ctx.constants.APP_EVENT_SCREENSHOT, async event => {
        const arrayBuffer = event.detail[0];
        const multiplier = event.detail[1];

        const width = 1920 * 2 * multiplier;
        const height = 1080 * 2 * multiplier;
        var canvas = document.createElement('canvas');
        canvas.width = width;
        canvas.height = height;
        var ctx = canvas.getContext('2d');

        var imageData = ctx.createImageData(width, height);
        imageData.data.set(arrayBuffer);
        ctx.putImageData(imageData, 0, 0);
        ctx.globalCompositeOperation = 'copy';
        ctx.scale(1, -1); // Y flip
        ctx.translate(0, -imageData.height);
        ctx.drawImage(canvas, 0, 0);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.globalCompositeOperation = 'source-over';

        const a = document.createElement('a');
        document.body.appendChild(a);
        a.classList.add('no-display');
        const blob = await new Promise(resolve => canvas.toBlob(resolve));
        const url = URL.createObjectURL(blob);
        a.href = url;
        a.download = 'Display-Sim_' + new Date().toISOString() + '.png';
        a.click();
        setTimeout(() => {
            URL.revokeObjectURL(url);
            a.remove();
        }, 3000);
    }, false);
}