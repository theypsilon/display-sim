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

const presetApertureGrille1 = 'crt-aperture-grille-1';
const presetShadowMask1 = 'crt-shadow-mask-1';
const presetShadowMask2 = 'crt-shadow-mask-2';
const presetSharp1 = 'sharp-1';
const presetDemo1 = 'demo-1';

export default {
    displayNoneClassName: 'display-none',
    scalingAutoHtmlId: 'scaling-auto',
    scaling43HtmlId: 'scaling-4:3',
    scalingCustomHtmlId: 'scaling-custom',
    scalingStretchToBothEdgesHtmlId: 'scaling-stretch-both',
    scalingStretchToNearestEdgeHtmlId: 'scaling-stretch-nearest',
    powerPreferenceDefaultHtml: 'default',
    glCanvasHtmlId: 'gl-canvas',
    topMessageHtmlId: 'top-message',
    firstPreviewImageId: 'first-preview-image',

    presetApertureGrille1,
    presetShadowMask1,
    presetShadowMask2,
    presetSharp1,
    presetDemo1,
    presetCustom: 'custom',
    properPresets: [
        presetApertureGrille1,
        presetShadowMask1,
        presetShadowMask2,
        presetSharp1,
        presetDemo1
    ],

    uiDeo: document.getElementById('ui'),
    loadingDeo: document.getElementById('loading'),
    inputFileUploadDeo: document.getElementById('file'),
    startAnimationDeo: document.getElementById('start-animation'),
    antialiasDeo: document.getElementById('antialias'),
    scalingCustomResWidthDeo: document.getElementById('scaling-custom-resolution-width'),
    scalingCustomResHeightDeo: document.getElementById('scaling-custom-resolution-height'),
    scalingCustomResButtonDeo: document.getElementById('scaling-custom-resolution-button'),
    scalingCustomArXDeo: document.getElementById('scaling-custom-aspect-ratio-x'),
    scalingCustomArYDeo: document.getElementById('scaling-custom-aspect-ratio-y'),
    scalingCustomStretchNearestDeo: document.getElementById('scaling-custom-stretch-nearest'),
    scalingCustomInputsDeo: document.getElementById('scaling-custom-inputs'),
    dropZoneDeo: document.getElementById('drop-zone'),
    selectImageList: document.getElementById('select-image-list'),
    restoreDefaultOptionsDeo: document.getElementById('restore-default-options'),

    optionPowerPreferenceSelect: document.getElementById('option-powerPreference'),
    optionScalingSelect: document.getElementById('option-scaling'),

    infoPanelBasicDeo: document.getElementById('info-panel-basic-settings'),
    infoPanelAdvancedDeo: document.getElementById('info-panel-advanced-settings'),

    toggleInfoPanelClass: document.querySelectorAll('.toggle-info-panel'),
    freeModeControlsClas: document.querySelectorAll('.free-mode-only-controls'),
    simulationUiDeo: document.getElementById('simulation-ui'),
    infoPanelDeo: document.getElementById('info-panel'),
    infoPanelAdvancedSettingsDeo: document.getElementById('info-panel-advanced-settings'),
    infoPanelContentDeo: document.getElementById('info-panel-content'),
    fpsCounterDeo: document.getElementById('fps-counter'),
    lightColorDeo: document.getElementById('light-color'),
    brightnessColorDeo: document.getElementById('brightness-color'),

    cameraPosXDeo: document.getElementById('camera-pos-x'),
    cameraPosYDeo: document.getElementById('camera-pos-y'),
    cameraPosZDeo: document.getElementById('camera-pos-z'),
    cameraDirXDeo: document.getElementById('camera-dir-x'),
    cameraDirYDeo: document.getElementById('camera-dir-y'),
    cameraDirZDeo: document.getElementById('camera-dir-z'),
    cameraAxisUpXDeo: document.getElementById('camera-axis-up-x'),
    cameraAxisUpYDeo: document.getElementById('camera-axis-up-y'),
    cameraAxisUpZDeo: document.getElementById('camera-axis-up-z'),
    cameraZoomDeo: document.getElementById('camera-zoom'),
    cameraMovementModeDeo: document.getElementById('camera-movement-mode'),

    filterPresetsDeo: document.getElementById('filter-presets'),
    // filterPresetsBasicDeo: document.getElementById('filter-presets-basic'),
    filterPresetsButtonDeoList: Array.from(document.getElementsByClassName('preset-btn')),
    filterOptionMainListDeo: document.getElementById('filter-option-main-list'),
    pixelWidthDeo: document.getElementById('pixel-width'),
    pixelHorizontalGapDeo: document.getElementById('pixel-horizontal-gap'),
    pixelVerticalGapDeo: document.getElementById('pixel-vertical-gap'),
    pixelSpreadDeo: document.getElementById('pixel-spread'),
    pixelBrigthnessDeo: document.getElementById('pixel-brightness'),
    pixelContrastDeo: document.getElementById('pixel-contrast'),
    blurLevelDeo: document.getElementById('blur-level'),
    verticalLppDeo: document.getElementById('vertical-lpp'),
    horizontalLppDeo: document.getElementById('horizontal-lpp'),
    featureQuitDeo: document.getElementById('feature-quit'),
    featureCaptureFramebufferDeo: document.getElementById('feature-capture-framebuffer'),
    featureClosePanelDeo: document.getElementById('feature-close-panel'),

    featureChangeColorRepresentationDeo: document.getElementById('feature-change-color-representation'),
    featureChangePixelGeometryDeo: document.getElementById('feature-change-pixel-geometry'),
    featureChangePixelShadowShapeDeo: document.getElementById('feature-change-pixel-shadow-shape'),
    featureChangePixelShadowHeightDeo: document.getElementById('feature-change-pixel-shadow-height'),
    featureChangeScreenCurvatureDeo: document.getElementById('feature-change-screen-curvature'),
    featureChangeScreenCurvatureBasicDeo: document.getElementById('feature-change-screen-curvature-basic'),
    featureInternalResolutionDeo: document.getElementById('feature-internal-resolution'),
    featureInternalResolutionBasicDeo: document.getElementById('feature-internal-resolution-basic'),
    featureTextureInterpolationDeo: document.getElementById('feature-texture-interpolation'),
    featureBacklightPercentDeo: document.getElementById('feature-backlight-percent'),

    featureChangeMoveSpeedDeo: document.getElementById('feature-change-move-speed'),
    featureChangeTurnSpeedDeo: document.getElementById('feature-change-turn-speed'),
    featureChangePixelSpeedDeo: document.getElementById('feature-change-pixel-speed'),
    featureCameraMovementsDeo: document.getElementById('feature-camera-movements'),
    featureCameraTurnsDeo: document.getElementById('feature-camera-turns'),
    resetCameraDeo: document.getElementById('reset-camera'),
    resetFiltersDeo: document.getElementById('reset-filters'),
    resetSpeedsDeo: document.getElementById('reset-speeds')
};
