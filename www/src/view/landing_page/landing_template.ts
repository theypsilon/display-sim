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

import { html, render } from 'lit-html';
import { LandingViewData } from './landing_view_model';
import {DataEvent, FileEvent} from '../../services/event_types';
import {PubSubImpl} from "../../services/pubsub";
import {Images} from "../../services/images";

const css = require('!css-loader!./css/landing_page.css').default.toString();

export type LandingTemplateEvents = ReturnType<typeof actions>;

export function actions() {
    return {
        addImage: PubSubImpl.make<File>(),
        selectImage: PubSubImpl.make<number>(),
        clickPlaySimulation: PubSubImpl.make<void>()
    };
}

export class LandingTemplate {
    private readonly _root: ShadowRoot;
    private readonly _actions: LandingTemplateEvents;

    private constructor(root: ShadowRoot, actions: LandingTemplateEvents) {
        this._actions = actions;
        this._root = root;
    }

    static make(root: ShadowRoot, actions: LandingTemplateEvents) {
        return new LandingTemplate(root, actions);
    }

    refresh(state: LandingViewData): void {
        render(this.generateLandingTemplate(state), this._root);
    }

    private changedFileInput(e: FileEvent): void {
        this._actions.addImage.fire(e.target.files[0]);
    }

    private selectImage(_: Event, idx: number): void {
        this._actions.selectImage.fire(idx);
    }

    private clickOnDropZone(_: Event): void {
        this._root.getElementById('file')?.click()
    }

    private dropOnDropZone(e: DataEvent): void {
        e.stopPropagation();
        e.preventDefault();
        this._actions.addImage.fire(e.dataTransfer.files[0]);
    }

    private dragOverDropZone(e: DataEvent): void {
        e.stopPropagation();
        e.preventDefault();
        e.dataTransfer.dropEffect = 'copy';
    }

    private clickPlaySimulation(_: Event): void {
        this._actions.clickPlaySimulation.fire();
    }

    private generateLandingTemplate (state: LandingViewData) {
        return html`
    <style>
        ${css}
    </style>

    <section id="ui" class="${state.visible ? '' : 'display-none'}">
        <a href="https://github.com/theypsilon/display-sim">
            <img id="fork-me" src="${Images.forkme}" alt="Fork me on GitHub">
        </a>
        <header class="container bg-white jumbotron page-header">
            <h1 class="margin-sm-bottom">Display Sim</h1>
            <div class="row">
                <div class="col-md-6 col-sm-12 margin-sm-bottom">
                    <p>This tool offers you a way to recreate
                        the retro visual feeling of old
                        displays within your modern LCD screens, with a strong emphasis on accuracy and flexibility.</p>
                </div>
                <div class="col-md-4 col-md-offset-2 col-sm-12 text-center">
                    <p>Don't get why this is a thing?</p>
                    <a class="btn btn-crt btn-green text-white" href="#explanation">Read some explanation below</a>
                </div>
            </div>
        </header>

        <section class="warning bg-warning text-white">
            <div class="container text-center">
                <p class="text-big spacing-md"><strong>WARNING! Mobile devices won't work!</strong></p>
                <p>You need a PC with NVIDIA or ATI graphics card with updated drivers and a<br>WebGL2 compatible
                    browser
                    (Firefox, Opera or Chrome) in order to run this without problems.</p>
            </div>
        </section>

        <section class="bg-green text-white form">
            <form class="container" id="form" name="form">
                <div class="col-sm-12 render-tests row">
                    <div class="margin-sm-bottom">
                        <h3>Select Image</h3>
                        <input type="file" id="file" class="display-none" accept="image/*" @change="${this.changedFileInput}">
                        <ul id="select-image-list" class="well select-image col-sm-12">
                            ${state.images.map((image, idx) => html`
                                <li id="${image.id}" @click="${(e: Event) => this.selectImage(e, idx)}" class="selectable-image ${idx === state.imageSelection ? 'selected-image' : ''}">
                                    <div><img src=${image.src} data-hq=${image.hq}><span>${image.width} ✕
                                            ${image.height}</span>
                                    </div>
                                </li>                                    
                            `)}
                            <li id="drop-zone" 
                                @click="${this.clickOnDropZone}" 
                                @drop="${this.dropOnDropZone}" 
                                @dragover="${this.dragOverDropZone}"
                                ><span>Add your image here</span>
                            </li>
                        </ul>
                    </div>
                </div>
                <input id="start-animation" 
                        class="start btn-crt btn-white" 
                        type="button" 
                        value="Play Simulation" 
                        @click="${this.clickPlaySimulation}" 
                        ?disabled="${state.isRunningOnMobileDevice}"
                        title="${state.isRunningOnMobileDevice ? 'You need a PC with NVIDIA or ATI graphics card with updated drivers and a WebGL2 compatible browser (Firefox, Opera or Chrome) in order to run this without problems.' : ''}"
                    >
            </form>
        </section>

        <section class="container explanation">
            <h3>Explanation:</h3>
            <a name="explanation"></a>
            <p>Old displays were designed to work with analog video signals carrying low resolution content. They used
                fundamentally different technologies (such as CRT) to represent images on the screen, than the LCD-based
                technologies that we use nowadays. Because of this, is no surprise that content created during that era
                is
                not fitting well our recently manufactured displays.</p>
            <p>That content was obviously never designed for the 21th century video systems. For example, consoles like
                the
                Mega Drive did usually work with resolutions such as 256x224 or 320x224 that would be deformed to fit a
                4:3
                screen. But because our current displays are made of a grid of fixed pixels, supporting only one native
                resolution, and one native aspect ratio, accomodating those kind of images can't be done without
                compromises.</p>
            <p>CRT's didn't have that constraint, they support multiple resolutions and aspect ratios natively. And
                graphics
                were designed with aspect ratio transformations in mind. Therefore, when in our current displays, we
                scale
                the picture by multiplying the original resolution by a given integer factor, those graphics are heavily
                distorted. Stretching the images to fit a 4:3 aspect ratio within the current resolution also gives
                distortion as result, because we would have to do some sort of pixel interpolation.</p>
            <p>Another issue is that the CRTs didn't render perfectly adyacent and squared pixels. This also alters
                heavily
                the perception of old graphics, which in today screens are looking too sharp. In old CRT's there was an
                inherent smooth effect, depending also on the analog video signal quality, that in some cases was used
                to
                make semi-transparency effects in games (see dithering).</p>
            <p>In order to preserve the CRT-era content, lots of work have been done in filters. They work usually by
                blending pixels and introducing gaps and effects. And they can be quite good, but there is still a
                significant amount of progress to be done in order to get closer to the original look.</p>
            <p>This project tries to experiment in a 3D space where every pixel is a 3D object, and filter parameters
                can be
                changed on the fly. This is an approach that I haven't seen so far, and offers great flexibility, so I
                had
                to give it a try. With the given controls, it's easy to recreate a simple scanlines filter while having
                many
                more advanced possibilities available.</p>
            <p>The tool renders automatically in your current screen resolution. I recommend having a 4k display,
                because
                the higher resolution, the more you can do to recreate a more accurate visualization. I also recommend
                having a decent GPU, because computing power can be also a limiting factor when playing with heavy
                filters.
            </p>
            <p>Keep in mind, that you should be uploading low res pictures, which is the original use case I intended to
                cover. Anything below 540p is fine. Here you got a <a
                    href="https://www.google.com/search?q=snes&biw=1853&bih=950&tbm=isch&source=lnt&tbs=isz:ex,iszw:256,iszh:224">link</a>
                with plenty of pictures of this kind. Keep in mind the scaling options you can select above, to fix the
                aspect ratio missmatches when working with other retro system with special screens (like the Game Boy).
            </p>
        </section>

        <footer class="clear text-white bg-footer">
            <div class="container">
                <h3>Credits</h3>
                <div class="row">
                    <div class="col-sm-4 margin-sm-bottom">
                        <label>Concept + Implementation</label>
                        <p>José Manuel Barroso Galindo<br>&lt;theypsilon@gmail.com&gt;</p>
                    </div>
                    <div class="col-sm-4 margin-sm-bottom">
                        <label>Landing page Designer</label>
                        <p>Ana Manosfrias<br>&lt;a.gilamor@gmail.com&gt;</p>
                    </div>
                    <div class="col-sm-4 margin-sm-bottom">
                        <label>Spaceship sprites</label>
                        <p>Ryoga<br>&lt;pig_saint@gmail.com&gt;</p>
                    </div>
                </div>
            </div>
        </footer>
    </section>
`;
    }
}
