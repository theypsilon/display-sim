/* Copyright (c) 2019-2022 José manuel Barroso Galindo <theypsilon@gmail.com>
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

import './css/index.css';
import './view/landing_page/landing_page';
import './view/sim_page/sim_page';

import FontFaceObserver from 'fontfaceobserver';
import { Navigator } from './services/navigator';

window.ondrop = event => {
    event.preventDefault();
};

window.ondragover = event => {
    event.preventDefault();
    event.dataTransfer.dropEffect = 'none';
};

window.onhashchange = () => {
    if (window.location.hash !== '') {
        window.location.reload();
    }
};

Promise.all([
    new FontFaceObserver('Archivo Black', { weight: 400 }).load(),
    new FontFaceObserver('Lato', { weight: 400 }).load(),
    new FontFaceObserver('Lato', { weight: 700 }).load()
]).catch((e) => {
    console.warn('Could not load fonts in time!')
    console.error(e);
}).finally(() => {
    const navigator = Navigator.make();
    navigator.goToLandingPage();
});
