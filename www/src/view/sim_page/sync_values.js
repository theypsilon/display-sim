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

window.addEventListener('app-event.camera_update', event => {
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
    { deo: Constants.cameraZoomDeo, eventId: 'app-event.change_camera_zoom' },
    { deo: Constants.cameraMovementModeDeo, eventId: 'app-event.change_camera_movement_mode' },
    { deo: Constants.pixelWidthDeo, eventId: 'app-event.change_pixel_width' },
    { deo: Constants.pixelHorizontalGapDeo, eventId: 'app-event.change_pixel_horizontal_gap' },
    { deo: Constants.pixelVerticalGapDeo, eventId: 'app-event.change_pixel_vertical_gap' },
    { deo: Constants.pixelSpreadDeo, eventId: 'app-event.change_pixel_spread' },
    { deo: Constants.pixelBrigthnessDeo, eventId: 'app-event.change_pixel_brightness' },
    { deo: Constants.pixelContrastDeo, eventId: 'app-event.change_pixel_contrast' },
    { deo: Constants.blurLevelDeo, eventId: 'app-event.change_blur_level' },
    { deo: Constants.verticalLppDeo, eventId: 'app-event.change_vertical_lpp' },
    { deo: Constants.horizontalLppDeo, eventId: 'app-event.change_horizontal_lpp' },
    { deo: Constants.lightColorDeo, eventId: 'app-event.change_light_color' },
    { deo: Constants.brightnessColorDeo, eventId: 'app-event.change_brightness_color' },
    { deo: Constants.featureChangeMoveSpeedDeo, eventId: 'app-event.change_movement_speed' },
    { deo: Constants.featureChangePixelSpeedDeo, eventId: 'app-event.change_pixel_speed' },
    { deo: Constants.featureChangeTurnSpeedDeo, eventId: 'app-event.change_turning_speed' },

    { deo: Constants.featureChangeColorRepresentationDeo, eventId: 'app-event.color_representation' },
    { deo: Constants.featureChangePixelGeometryDeo, eventId: 'app-event.pixel_geometry' },
    { deo: Constants.featureChangePixelShadowShapeDeo, eventId: 'app-event.pixel_shadow_shape' },
    { deo: Constants.featureChangePixelShadowHeightDeo, eventId: 'app-event.pixel_shadow_height' },
    { deo: Constants.featureBacklightPercentDeo, eventId: 'app-event.backlight_percent' },
    { deo: Constants.featureInternalResolutionDeo, eventId: 'app-event.internal_resolution' },
    { deo: Constants.featureInternalResolutionBasicDeo, eventId: 'app-event.internal_resolution' },
    { deo: Constants.featureTextureInterpolationDeo, eventId: 'app-event.texture_interpolation' },
    { deo: Constants.featureChangeScreenCurvatureDeo, eventId: 'app-event.screen_curvature' },
    { deo: Constants.featureChangeScreenCurvatureBasicDeo, eventId: 'app-event.screen_curvature' }
].forEach(({ deo, eventId }) => {
    if (!deo) throw new Error('Wrong deo on defining: ' + eventId);
    window.addEventListener(eventId, event => {
        deo.value = event.detail;
        if (event.id === 'app-event.change_camera_movement_mode') {
            switch (event.detail) {
            case 'Lock on Display':
                deo.title = 'The camera will move around the picture, always looking at it';
                Constants.freeModeControlsClas.forEach(deo => deo.classList.add(Constants.displayNoneClassName));
                break;
            case 'Free Flight':
                deo.title = 'The camera can move without any restriction in the whole 3D space with plane-like controls';
                Constants.freeModeControlsClas.forEach(deo => deo.classList.remove(Constants.displayNoneClassName));
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

customEventOnChange(Constants.cameraPosXDeo, 'camera_pos_x', a => +a);
customEventOnChange(Constants.cameraPosYDeo, 'camera_pos_y', a => +a);
customEventOnChange(Constants.cameraPosZDeo, 'camera_pos_z', a => +a);
customEventOnChange(Constants.cameraAxisUpXDeo, 'camera_axis_up_x', a => +a);
customEventOnChange(Constants.cameraAxisUpYDeo, 'camera_axis_up_y', a => +a);
customEventOnChange(Constants.cameraAxisUpZDeo, 'camera_axis_up_z', a => +a);
customEventOnChange(Constants.cameraDirXDeo, 'camera_direction_x', a => +a);
customEventOnChange(Constants.cameraDirYDeo, 'camera_direction_y', a => +a);
customEventOnChange(Constants.cameraDirZDeo, 'camera_direction_z', a => +a);
customEventOnChange(Constants.cameraZoomDeo, 'camera_zoom', a => +a);
customEventOnChange(Constants.cameraMovementModeDeo, 'camera_movement_mode', a => +a);

customEventOnChange(Constants.pixelWidthDeo, 'pixel_width', a => +a);
customEventOnChange(Constants.pixelSpreadDeo, 'pixel_spread', a => +a);
customEventOnChange(Constants.pixelHorizontalGapDeo, 'pixel_horizontal_gap', a => +a);
customEventOnChange(Constants.pixelVerticalGapDeo, 'pixel_vertical_gap', a => +a);
customEventOnChange(Constants.blurLevelDeo, 'blur_level', a => +a);
customEventOnChange(Constants.verticalLppDeo, 'vertical_lpp', a => +a);
customEventOnChange(Constants.horizontalLppDeo, 'horizontal_lpp', a => +a);
customEventOnChange(Constants.pixelBrigthnessDeo, 'pixel_brightness', a => +a);
customEventOnChange(Constants.pixelContrastDeo, 'pixel_contrast', a => +a);
customEventOnChange(Constants.featureChangePixelShadowHeightDeo, 'pixel_shadow_height', a => +a);
customEventOnChange(Constants.featureBacklightPercentDeo, 'backlight_percent', a => +a);

const parseColor = (value) => parseInt('0x' + value.substring(1));
customEventOnChange(Constants.lightColorDeo, 'light_color', parseColor);
customEventOnChange(Constants.brightnessColorDeo, 'brightness_color', parseColor);
function customEventOnChange (deo, kind, parse) {
    const changed = () => {
        window.dispatchEvent(new CustomEvent('app-event.custom_input_event', {
            detail: {
                value: parse(deo.value),
                kind: 'event_kind:' + kind
            }
        }));
    };
    deo.onchange = changed;
}
