import { assert } from 'chai';
import {
    BrowserKeyState,
    HeldActionState,
    isHeldActionKey,
    isSingleActivationKey,
    nextSimulationUiMode,
    normalizeWheelDelta,
    OneFramePulseState,
    PrimaryPointerState,
    screenshotName,
    WHEEL_POINTS_PER_LINE
} from '../../../src/view/sim_page/interaction_contract';

describe('simulation interaction contract', () => {
    it('toggles between the two web panel implementations', () => {
        assert.equal(nextSimulationUiMode('webgl'), 'html');
        assert.equal(nextSimulationUiMode('html'), 'webgl');
    });

    it('normalizes wheel pixels, lines, and pages', () => {
        assert.equal(normalizeWheelDelta(12.5, 0, 720), 12.5);
        assert.equal(normalizeWheelDelta(-2, 1, 720), -2 * WHEEL_POINTS_PER_LINE);
        assert.equal(normalizeWheelDelta(1, 2, 720), 720);
    });

    it('isolates browser button activation keys without swallowing movement keys', () => {
        assert.isTrue(isHeldActionKey('Enter'));
        assert.isTrue(isHeldActionKey(' '));
        assert.isFalse(isHeldActionKey('Space'));
        for (const key of ['w', 'a', 's', 'd']) {
            assert.isFalse(isHeldActionKey(key));
        }
        assert.isTrue(isSingleActivationKey('Enter', false));
        assert.isTrue(isSingleActivationKey(' ', false));
        assert.isFalse(isSingleActivationKey('Enter', true));
    });

    it('tracks only balanced primary-pointer edges', () => {
        const state = new PrimaryPointerState();
        assert.isFalse(state.press(2, 2));
        assert.isTrue(state.press(0, 1));
        assert.isTrue(state.isDown());
        assert.isFalse(state.press(0, 1));
        assert.isFalse(state.release(2));
        assert.isTrue(state.release(0));
        assert.isFalse(state.isDown());
        assert.isFalse(state.release(0));
        assert.isTrue(state.press(0, 3));
        assert.isTrue(state.cancel());
        assert.isFalse(state.cancel());
        assert.isTrue(state.press(0, 1));
        state.reset();
        assert.isFalse(state.release(0));
    });

    it('uses the same deterministic screenshot names as native', () => {
        assert.equal(screenshotName(1), 'screenshot-1.png');
        assert.equal(screenshotName(42), 'screenshot-42.png');
        assert.throws(() => screenshotName(0));
        assert.throws(() => screenshotName(1.5));
    });

    it('emits each held transition once and drains on focus loss', () => {
        const state = new HeldActionState();
        assert.isTrue(state.press('w'));
        assert.isFalse(state.press('w'));
        assert.deepEqual(state.release('w'), { key: 'w', current: undefined });
        assert.isUndefined(state.release('w'));

        state.press('left-inc', 'current');
        state.press('right-inc');
        assert.deepEqual(state.drain(), [
            { key: 'left-inc', current: 'current' },
            { key: 'right-inc', current: undefined }
        ]);
        assert.deepEqual(state.drain(), []);
    });

    it('does not release a held action until every input source releases it', () => {
        const state = new HeldActionState();
        assert.isTrue(state.press('w', undefined, 'pointer:1'));
        assert.isFalse(state.press('w', undefined, 'keyboard:KeyW'));
        assert.isUndefined(state.release('w', 'pointer:1'));
        assert.deepEqual(state.release('w', 'keyboard:KeyW'), { key: 'w', current: undefined });

        state.press('left-inc', 'current', 'pointer:2');
        state.press('left-inc', 'current', 'keyboard:Enter');
        assert.isUndefined(state.releaseSources('left-inc', 'keyboard:'));
        assert.deepEqual(state.release('left-inc', 'pointer:2'), { key: 'left-inc', current: 'current' });
    });

    it('keeps a layout key stable until its physical key is released', () => {
        const state = new BrowserKeyState();
        assert.equal(state.press('Equal', '+'), '+');
        assert.isUndefined(state.press('Equal', '='));
        assert.equal(state.release('Equal', '='), '+');
        assert.isUndefined(state.release('Equal', '='));

        assert.equal(state.press('Quote', 'Dead'), 'Dead');
        assert.equal(state.release('Quote', "'"), 'Dead');
    });

    it('keeps a logical modifier down until both physical modifiers release', () => {
        const state = new BrowserKeyState();
        assert.equal(state.press('ShiftLeft', 'Shift', 1), 'Shift');
        assert.isUndefined(state.press('ShiftRight', 'Shift', 2));
        assert.isUndefined(state.release('ShiftLeft', 'Shift', 1));
        assert.equal(state.release('ShiftRight', 'Shift', 2), 'Shift');
    });

    it('recovers a modifier-changed release when a virtual keyboard omits code', () => {
        const state = new BrowserKeyState();
        assert.equal(state.press('', '+'), '+');
        assert.equal(state.release('', '='), '+');
    });

    it('only lets the newest one-frame pulse release a logical action', () => {
        const state = new OneFramePulseState();
        const first = state.begin('reset-filters');
        const second = state.begin('reset-filters');
        assert.isFalse(state.finish('reset-filters', first));
        assert.isTrue(state.finish('reset-filters', second));
        assert.isFalse(state.finish('reset-filters', second));

        const cancelled = state.begin('reset-camera');
        state.clear();
        assert.isFalse(state.finish('reset-camera', cancelled));
        const afterClear = state.begin('reset-camera');
        assert.isFalse(state.finish('reset-camera', cancelled));
        assert.isTrue(state.finish('reset-camera', afterClear));
    });
});
