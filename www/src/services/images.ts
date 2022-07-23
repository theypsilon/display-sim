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

export interface SimImage {
    src: string,
    hq: string,
    width: number,
    height: number,
    id?: string,
    img?: HTMLImageElement & {isGif: boolean},
    isGif?: boolean
}

export const Images = {
    wwix: { src: require('../../assets/pics/opt-frames/wwix.gif').default, hq: require('../../assets/pics/frames/wwix.gif').default },
    seiken: { src: require('../../assets/pics/opt-frames/seiken.png').default, hq: require('../../assets/pics/frames/seiken.png').default },
    sonicscroll: { src: require('../../assets/pics/opt-frames/sonicscroll.gif').default, hq: require('../../assets/pics/frames/sonicscroll.gif').default },
    metroid: { src: require('../../assets/pics/opt-frames/metroid.gif').default, hq: require('../../assets/pics/frames/metroid.gif').default },
    tf4: { src: require('../../assets/pics/opt-frames/tf4.gif').default, hq: require('../../assets/pics/frames/tf4.gif').default },
    dkc2: { src: require('../../assets/pics/opt-frames/dkc2.png').default, hq: require('../../assets/pics/frames/dkc2.png').default },
    forkme: require('../../assets/pics/forkme.png').default,
    wwix_spritesheet: require('../../assets/pics/wwix_spritesheet.png').default
};
