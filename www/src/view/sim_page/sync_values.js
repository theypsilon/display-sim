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

window.addEventListener(Constants.APP_EVENT_CAMERA_UPDATE, event => {
    Constants.cameraPosXDeo.value = Math.round(event.detail[0] * 100) / 100;
    Constants.cameraPosYDeo.value = Math.round(event.detail[1] * 100) / 100;
    Constants.cameraPosZDeo.value = Math.round(event.detail[2] * 100) / 100;
    Constants.cameraDirXDeo.value = Math.round(event.detail[3] * 100) / 100;
    Constants.cameraDirYDeo.value = Math.round(event.detail[4] * 100) / 100;
    Constants.cameraDirZDeo.value = Math.round(event.detail[5] * 100) / 100;
    Constants.cameraAxisUpXDeo.value = Math.round(event.detail[6] * 100) / 100;
    Constants.cameraAxisUpYDeo.value = Math.round(event.detail[7] * 100) / 100;
    Constants.cameraAxisUpZDeo.value = Math.round(event.detail[8] * 100) / 100;
}, false);

[
    { deo: Constants.cameraZoomDeo, eventId: Constants.APP_EVENT_CHANGE_CAMERA_ZOOM },
    { deo: Constants.cameraMovementModeDeo, eventId: Constants.APP_EVENT_CHANGE_CAMERA_MOVEMENT_MODE },
    { deo: Constants.pixelWidthDeo, eventId: Constants.APP_EVENT_CHANGE_PIXEL_WIDTH },
    { deo: Constants.pixelHorizontalGapDeo, eventId: Constants.APP_EVENT_CHANGE_PIXEL_HORIZONTAL_GAP },
    { deo: Constants.pixelVerticalGapDeo, eventId: Constants.APP_EVENT_CHANGE_PIXEL_VERTICAL_GAP },
    { deo: Constants.pixelSpreadDeo, eventId: Constants.APP_EVENT_CHANGE_PIXEL_SPREAD },
    { deo: Constants.pixelBrigthnessDeo, eventId: Constants.APP_EVENT_CHANGE_PIXEL_BRIGHTNESS },
    { deo: Constants.pixelContrastDeo, eventId: Constants.APP_EVENT_CHANGE_PIXEL_CONTRAST },
    { deo: Constants.blurLevelDeo, eventId: Constants.APP_EVENT_CHANGE_BLUR_LEVEL },
    { deo: Constants.verticalLppDeo, eventId: Constants.APP_EVENT_CHANGE_VERTICAL_LPP },
    { deo: Constants.horizontalLppDeo, eventId: Constants.APP_EVENT_CHANGE_HORIZONTAL_LPP },
    { deo: Constants.lightColorDeo, eventId: Constants.APP_EVENT_CHANGE_LIGHT_COLOR },
    { deo: Constants.brightnessColorDeo, eventId: Constants.APP_EVENT_CHANGE_BRIGHTNESS_COLOR },
    { deo: Constants.featureChangeMoveSpeedDeo, eventId: Constants.APP_EVENT_CHANGE_MOVEMENT_SPEED },
    { deo: Constants.featureChangePixelSpeedDeo, eventId: Constants.APP_EVENT_CHANGE_PIXEL_SPEED },
    { deo: Constants.featureChangeTurnSpeedDeo, eventId: Constants.APP_EVENT_CHANGE_TURNING_SPEED },

    { deo: Constants.featureChangeColorRepresentationDeo, eventId: Constants.APP_EVENT_COLOR_REPRESENTATION },
    { deo: Constants.featureChangePixelGeometryDeo, eventId: Constants.APP_EVENT_PIXEL_GEOMETRY },
    { deo: Constants.featureChangePixelShadowShapeDeo, eventId: Constants.APP_EVENT_PIXEL_SHADOW_SHAPE },
    { deo: Constants.featureChangePixelShadowHeightDeo, eventId: Constants.APP_EVENT_PIXEL_SHADOW_HEIGHT },
    { deo: Constants.featureBacklightPercentDeo, eventId: Constants.APP_EVENT_BACKLIGHT_PERCENT },
    { deo: Constants.featureInternalResolutionDeo, eventId: Constants.APP_EVENT_INTERNAL_RESOLUTION },
    { deo: Constants.featureInternalResolutionBasicDeo, eventId: Constants.APP_EVENT_INTERNAL_RESOLUTION },
    { deo: Constants.featureTextureInterpolationDeo, eventId: Constants.APP_EVENT_TEXTURE_INTERPOLATION },
    { deo: Constants.featureChangeScreenCurvatureDeo, eventId: Constants.APP_EVENT_SCREEN_CURVATURE },
    { deo: Constants.featureChangeScreenCurvatureBasicDeo, eventId: Constants.APP_EVENT_SCREEN_CURVATURE }
].forEach(({ deo, eventId }) => {
    if (!deo) throw new Error('Wrong deo on defining: ' + eventId);
    window.addEventListener(eventId, event => {
        deo.value = event.detail;
        if (event.id === Constants.APP_EVENT_CHANGE_CAMERA_MOVEMENT_MODE) {
            switch (event.detail) {
            case 'Lock on Display':
                deo.title = 'The camera will move around the picture, always looking at it';
                Constants.freeModeControlsClas.forEach(deo => deo.classList.add(Constants.DISPLAY_NONE_CLASS));
                break;
            case 'Free Flight':
                deo.title = 'The camera can move without any restriction in the whole 3D space with plane-like controls';
                Constants.freeModeControlsClas.forEach(deo => deo.classList.remove(Constants.DISPLAY_NONE_CLASS));
                break;
            default:
                throw new Error('Unreachable!');
            }
        }
    }, false);
});

customEventOnButtonPressed(Constants.featureCameraMovementsDeo);
customEventOnButtonPressed(Constants.featureCameraTurnsDeo);
function customEventOnButtonPressed (deo) {
    deo.querySelectorAll('.activate-button').forEach(button => {
        const eventOptions = { key: button.innerHTML.toLowerCase() };
        button.onmousedown = () => document.dispatchEvent(new KeyboardEvent('keydown', eventOptions));
        button.onmouseup = () => document.dispatchEvent(new KeyboardEvent('keyup', eventOptions));
    });
}

customEventOnChange(Constants.cameraPosXDeo, Constants.EVENT_KIND_CAMERA_POS_X, a => +a);
customEventOnChange(Constants.cameraPosYDeo, Constants.EVENT_KIND_CAMERA_POS_Y, a => +a);
customEventOnChange(Constants.cameraPosZDeo, Constants.EVENT_KIND_CAMERA_POS_Z, a => +a);
customEventOnChange(Constants.cameraAxisUpXDeo, Constants.EVENT_KIND_CAMERA_AXIS_UP_X, a => +a);
customEventOnChange(Constants.cameraAxisUpYDeo, Constants.EVENT_KIND_CAMERA_AXIS_UP_Y, a => +a);
customEventOnChange(Constants.cameraAxisUpZDeo, Constants.EVENT_KIND_CAMERA_AXIS_UP_Z, a => +a);
customEventOnChange(Constants.cameraDirXDeo, Constants.EVENT_KIND_CAMERA_DIRECTION_X, a => +a);
customEventOnChange(Constants.cameraDirYDeo, Constants.EVENT_KIND_CAMERA_DIRECTION_Y, a => +a);
customEventOnChange(Constants.cameraDirZDeo, Constants.EVENT_KIND_CAMERA_DIRECTION_Z, a => +a);
customEventOnChange(Constants.cameraZoomDeo, Constants.EVENT_KIND_CAMERA_ZOOM, a => +a);
customEventOnChange(Constants.cameraMovementModeDeo, Constants.EVENT_KIND_CAMERA_MOVEMENT_MODE, a => +a);

customEventOnChange(Constants.pixelWidthDeo, Constants.EVENT_KIND_PIXEL_WIDTH, a => +a);
customEventOnChange(Constants.pixelSpreadDeo, Constants.EVENT_KIND_PIXEL_SPREAD, a => +a);
customEventOnChange(Constants.pixelHorizontalGapDeo, Constants.EVENT_KIND_PIXEL_HORIZONTAL_GAP, a => +a);
customEventOnChange(Constants.pixelVerticalGapDeo, Constants.EVENT_KIND_PIXEL_VERTICAL_GAP, a => +a);
customEventOnChange(Constants.blurLevelDeo, Constants.EVENT_KIND_BLUR_LEVEL, a => +a);
customEventOnChange(Constants.verticalLppDeo, Constants.EVENT_KIND_VERTICAL_LPP, a => +a);
customEventOnChange(Constants.horizontalLppDeo, Constants.EVENT_KIND_HORIZONTAL_LPP, a => +a);
customEventOnChange(Constants.pixelBrigthnessDeo, Constants.EVENT_KIND_PIXEL_BRIGHTNESS, a => +a);
customEventOnChange(Constants.pixelContrastDeo, Constants.EVENT_KIND_PIXEL_CONTRAST, a => +a);
customEventOnChange(Constants.featureChangePixelShadowHeightDeo, Constants.EVENT_KIND_PIXEL_SHADOW_HEIGHT, a => +a);
customEventOnChange(Constants.featureBacklightPercentDeo, Constants.EVENT_KIND_BACKLIGHT_PERCENT, a => +a);

const parseColor = (value) => parseInt('0x' + value.substring(1));
customEventOnChange(Constants.lightColorDeo, Constants.EVENT_KIND_LIGHT_COLOR, parseColor);
customEventOnChange(Constants.brightnessColorDeo, Constants.EVENT_KIND_BRIGHTNESS_COLOR, parseColor);
function customEventOnChange (deo, kind, parse) {
    const changed = () => {
        window.dispatchEvent(new CustomEvent(Constants.APP_EVENT_CUSTOM_INPUT, {
            detail: {
                value: parse(deo.value),
                kind: Constants.EVENT_KIND_PREFIX + kind
            }
        }));
    };
    deo.onchange = changed;
}
