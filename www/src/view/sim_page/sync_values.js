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

import Constants from '../../services/constants';

export default function (ctx) {
    window.addEventListener(ctx.constants.APP_EVENT_CAMERA_UPDATE, event => {
        ctx.constants.cameraPosXDeo.value = Math.round(event.detail[0] * 100) / 100;
        ctx.constants.cameraPosYDeo.value = Math.round(event.detail[1] * 100) / 100;
        ctx.constants.cameraPosZDeo.value = Math.round(event.detail[2] * 100) / 100;
        ctx.constants.cameraDirXDeo.value = Math.round(event.detail[3] * 100) / 100;
        ctx.constants.cameraDirYDeo.value = Math.round(event.detail[4] * 100) / 100;
        ctx.constants.cameraDirZDeo.value = Math.round(event.detail[5] * 100) / 100;
        ctx.constants.cameraAxisUpXDeo.value = Math.round(event.detail[6] * 100) / 100;
        ctx.constants.cameraAxisUpYDeo.value = Math.round(event.detail[7] * 100) / 100;
        ctx.constants.cameraAxisUpZDeo.value = Math.round(event.detail[8] * 100) / 100;
    }, false);

    [
        { deo: ctx.constants.cameraZoomDeo, eventId: ctx.constants.APP_EVENT_CHANGE_CAMERA_ZOOM },
        { deo: ctx.constants.cameraMovementModeDeo, eventId: ctx.constants.APP_EVENT_CHANGE_CAMERA_MOVEMENT_MODE },
        { deo: ctx.constants.pixelWidthDeo, eventId: ctx.constants.APP_EVENT_CHANGE_PIXEL_WIDTH },
        { deo: ctx.constants.pixelHorizontalGapDeo, eventId: ctx.constants.APP_EVENT_CHANGE_PIXEL_HORIZONTAL_GAP },
        { deo: ctx.constants.pixelVerticalGapDeo, eventId: ctx.constants.APP_EVENT_CHANGE_PIXEL_VERTICAL_GAP },
        { deo: ctx.constants.pixelSpreadDeo, eventId: ctx.constants.APP_EVENT_CHANGE_PIXEL_SPREAD },
        { deo: ctx.constants.pixelBrigthnessDeo, eventId: ctx.constants.APP_EVENT_CHANGE_PIXEL_BRIGHTNESS },
        { deo: ctx.constants.pixelContrastDeo, eventId: ctx.constants.APP_EVENT_CHANGE_PIXEL_CONTRAST },
        { deo: ctx.constants.blurLevelDeo, eventId: ctx.constants.APP_EVENT_CHANGE_BLUR_LEVEL },
        { deo: ctx.constants.verticalLppDeo, eventId: ctx.constants.APP_EVENT_CHANGE_VERTICAL_LPP },
        { deo: ctx.constants.horizontalLppDeo, eventId: ctx.constants.APP_EVENT_CHANGE_HORIZONTAL_LPP },
        { deo: ctx.constants.lightColorDeo, eventId: ctx.constants.APP_EVENT_CHANGE_LIGHT_COLOR },
        { deo: ctx.constants.brightnessColorDeo, eventId: ctx.constants.APP_EVENT_CHANGE_BRIGHTNESS_COLOR },
        { deo: ctx.constants.featureChangeMoveSpeedDeo, eventId: ctx.constants.APP_EVENT_CHANGE_MOVEMENT_SPEED },
        { deo: ctx.constants.featureChangePixelSpeedDeo, eventId: ctx.constants.APP_EVENT_CHANGE_PIXEL_SPEED },
        { deo: ctx.constants.featureChangeTurnSpeedDeo, eventId: ctx.constants.APP_EVENT_CHANGE_TURNING_SPEED },

        { deo: ctx.constants.featureChangeColorRepresentationDeo, eventId: ctx.constants.APP_EVENT_COLOR_REPRESENTATION },
        { deo: ctx.constants.featureChangePixelGeometryDeo, eventId: ctx.constants.APP_EVENT_PIXEL_GEOMETRY },
        { deo: ctx.constants.featureChangePixelShadowShapeDeo, eventId: ctx.constants.APP_EVENT_PIXEL_SHADOW_SHAPE },
        { deo: ctx.constants.featureChangePixelShadowHeightDeo, eventId: ctx.constants.APP_EVENT_PIXEL_SHADOW_HEIGHT },
        { deo: ctx.constants.featureBacklightPercentDeo, eventId: ctx.constants.APP_EVENT_BACKLIGHT_PERCENT },
        { deo: ctx.constants.featureInternalResolutionDeo, eventId: ctx.constants.APP_EVENT_INTERNAL_RESOLUTION },
        { deo: ctx.constants.featureInternalResolutionBasicDeo, eventId: ctx.constants.APP_EVENT_INTERNAL_RESOLUTION },
        { deo: ctx.constants.featureTextureInterpolationDeo, eventId: ctx.constants.APP_EVENT_TEXTURE_INTERPOLATION },
        { deo: ctx.constants.featureChangeScreenCurvatureDeo, eventId: ctx.constants.APP_EVENT_SCREEN_CURVATURE },
        { deo: ctx.constants.featureChangeScreenCurvatureBasicDeo, eventId: ctx.constants.APP_EVENT_SCREEN_CURVATURE }
    ].forEach(({ deo, eventId }) => {
        if (!deo) throw new Error('Wrong deo on defining: ' + eventId);
        window.addEventListener(eventId, event => {
            deo.value = event.detail;
            if (event.id === ctx.constants.APP_EVENT_CHANGE_CAMERA_MOVEMENT_MODE) {
                switch (event.detail) {
                case 'Lock on Display':
                    deo.title = 'The camera will move around the picture, always looking at it';
                    ctx.constants.freeModeControlsClas.forEach(deo => deo.classList.add(Constants.DISPLAY_NONE_CLASS));
                    break;
                case 'Free Flight':
                    deo.title = 'The camera can move without any restriction in the whole 3D space with plane-like controls';
                    ctx.constants.freeModeControlsClas.forEach(deo => deo.classList.remove(Constants.DISPLAY_NONE_CLASS));
                    break;
                default:
                    throw new Error('Unreachable!');
                }
            }
        }, false);
    });

    customEventOnButtonPressed(ctx.constants.featureCameraMovementsDeo);
    customEventOnButtonPressed(ctx.constants.featureCameraTurnsDeo);
    function customEventOnButtonPressed (deo) {
        deo.querySelectorAll('.activate-button').forEach(button => {
            const eventOptions = { key: button.innerHTML.toLowerCase() };
            button.onmousedown = () => document.dispatchEvent(new KeyboardEvent('keydown', eventOptions));
            button.onmouseup = () => document.dispatchEvent(new KeyboardEvent('keyup', eventOptions));
        });
    }

    customEventOnChange(ctx.constants.cameraPosXDeo, ctx.constants.EVENT_KIND_CAMERA_POS_X, a => +a);
    customEventOnChange(ctx.constants.cameraPosYDeo, ctx.constants.EVENT_KIND_CAMERA_POS_Y, a => +a);
    customEventOnChange(ctx.constants.cameraPosZDeo, ctx.constants.EVENT_KIND_CAMERA_POS_Z, a => +a);
    customEventOnChange(ctx.constants.cameraAxisUpXDeo, ctx.constants.EVENT_KIND_CAMERA_AXIS_UP_X, a => +a);
    customEventOnChange(ctx.constants.cameraAxisUpYDeo, ctx.constants.EVENT_KIND_CAMERA_AXIS_UP_Y, a => +a);
    customEventOnChange(ctx.constants.cameraAxisUpZDeo, ctx.constants.EVENT_KIND_CAMERA_AXIS_UP_Z, a => +a);
    customEventOnChange(ctx.constants.cameraDirXDeo, ctx.constants.EVENT_KIND_CAMERA_DIRECTION_X, a => +a);
    customEventOnChange(ctx.constants.cameraDirYDeo, ctx.constants.EVENT_KIND_CAMERA_DIRECTION_Y, a => +a);
    customEventOnChange(ctx.constants.cameraDirZDeo, ctx.constants.EVENT_KIND_CAMERA_DIRECTION_Z, a => +a);
    customEventOnChange(ctx.constants.cameraZoomDeo, ctx.constants.EVENT_KIND_CAMERA_ZOOM, a => +a);
    customEventOnChange(ctx.constants.cameraMovementModeDeo, ctx.constants.EVENT_KIND_CAMERA_MOVEMENT_MODE, a => +a);

    customEventOnChange(ctx.constants.pixelWidthDeo, ctx.constants.EVENT_KIND_PIXEL_WIDTH, a => +a);
    customEventOnChange(ctx.constants.pixelSpreadDeo, ctx.constants.EVENT_KIND_PIXEL_SPREAD, a => +a);
    customEventOnChange(ctx.constants.pixelHorizontalGapDeo, ctx.constants.EVENT_KIND_PIXEL_HORIZONTAL_GAP, a => +a);
    customEventOnChange(ctx.constants.pixelVerticalGapDeo, ctx.constants.EVENT_KIND_PIXEL_VERTICAL_GAP, a => +a);
    customEventOnChange(ctx.constants.blurLevelDeo, ctx.constants.EVENT_KIND_BLUR_LEVEL, a => +a);
    customEventOnChange(ctx.constants.verticalLppDeo, ctx.constants.EVENT_KIND_VERTICAL_LPP, a => +a);
    customEventOnChange(ctx.constants.horizontalLppDeo, ctx.constants.EVENT_KIND_HORIZONTAL_LPP, a => +a);
    customEventOnChange(ctx.constants.pixelBrigthnessDeo, ctx.constants.EVENT_KIND_PIXEL_BRIGHTNESS, a => +a);
    customEventOnChange(ctx.constants.pixelContrastDeo, ctx.constants.EVENT_KIND_PIXEL_CONTRAST, a => +a);
    customEventOnChange(ctx.constants.featureChangePixelShadowHeightDeo, ctx.constants.EVENT_KIND_PIXEL_SHADOW_HEIGHT, a => +a);
    customEventOnChange(ctx.constants.featureBacklightPercentDeo, ctx.constants.EVENT_KIND_BACKLIGHT_PERCENT, a => +a);

    const parseColor = (value) => parseInt('0x' + value.substring(1));
    customEventOnChange(ctx.constants.lightColorDeo, ctx.constants.EVENT_KIND_LIGHT_COLOR, parseColor);
    customEventOnChange(ctx.constants.brightnessColorDeo, ctx.constants.EVENT_KIND_BRIGHTNESS_COLOR, parseColor);
    function customEventOnChange (deo, kind, parse) {
        const changed = () => {
            window.dispatchEvent(new CustomEvent(ctx.constants.APP_EVENT_CUSTOM_INPUT, {
                detail: {
                    value: parse(deo.value),
                    kind: ctx.constants.EVENT_KIND_PREFIX + kind
                }
            }));
        };
        deo.onchange = changed;
    }
}