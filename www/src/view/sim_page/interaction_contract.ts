/* Copyright (c) 2019-2024 José manuel Barroso Galindo <theypsilon@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 */

export const WHEEL_POINTS_PER_LINE = 100;

export type SimulationUiMode = 'webgl' | 'html';

export function nextSimulationUiMode (mode: SimulationUiMode): SimulationUiMode {
    return mode === 'webgl' ? 'html' : 'webgl';
}

export function screenshotName (index: number): string {
    if (!Number.isSafeInteger(index) || index < 1) {
        throw new Error(`Invalid screenshot index: ${index}`);
    }
    return `screenshot-${index}.png`;
}

export function normalizeWheelDelta (deltaY: number, deltaMode: number, pageHeight: number): number {
    switch (deltaMode) {
    case 1: return deltaY * WHEEL_POINTS_PER_LINE;
    case 2: return deltaY * pageHeight;
    default: return deltaY;
    }
}

/** Tracks only the primary simulation pointer and emits balanced edges. */
export class PrimaryPointerState {
    private _down = false;

    press (button: number, buttons: number): boolean {
        if (this._down || button !== 0 || (buttons & 1) === 0) {
            return false;
        }
        this._down = true;
        return true;
    }

    release (button: number): boolean {
        return button === 0 && this.cancel();
    }

    cancel (): boolean {
        if (!this._down) {
            return false;
        }
        this._down = false;
        return true;
    }

    reset (): void {
        this._down = false;
    }

    isDown (): boolean {
        return this._down;
    }
}

export function isHeldActionKey (key: string): boolean {
    return key === 'Enter' || key === ' ';
}

export function isSingleActivationKey (key: string, repeat: boolean): boolean {
    return !repeat && isHeldActionKey(key);
}

const simulationHandledKeyboardEvents = new WeakSet<Event>();

/**
 * Marks Enter/Space as belonging to a focused UI control. The event must
 * still bubble to the window listener so an earlier simulation key-down can
 * receive its matching key-up after focus moves.
 */
export function handleSimulationActivationKey (event: KeyboardEvent): void {
    if (isHeldActionKey(event.key)) {
        simulationHandledKeyboardEvents.add(event);
    }
}

export function isSimulationKeyboardEventHandled (event: Event): boolean {
    return simulationHandledKeyboardEvents.has(event);
}

export interface HeldAction {
    key: string;
    current?: string;
}

interface HeldActionEntry {
    action: HeldAction;
    sources: Set<string>;
}

/**
 * De-duplicates the transitions produced by pointer capture, pointer leave,
 * keyboard repeat, and window blur. Native and web controls both emit one
 * key-down when an action starts and one key-up when it ends.
 */
export class HeldActionState {
    private readonly _held = new Map<string, HeldActionEntry>();

    press (key: string, current?: string, source = 'default'): boolean {
        const entry = this._held.get(key);
        if (entry) {
            entry.sources.add(source);
            return false;
        }
        this._held.set(key, {
            action: { key, current },
            sources: new Set([source])
        });
        return true;
    }

    release (key: string, source = 'default'): HeldAction | undefined {
        const entry = this._held.get(key);
        if (!entry || !entry.sources.delete(source) || entry.sources.size !== 0) {
            return undefined;
        }
        this._held.delete(key);
        return entry.action;
    }

    releaseSources (key: string, sourcePrefix: string): HeldAction | undefined {
        const entry = this._held.get(key);
        if (!entry) {
            return undefined;
        }
        for (const source of entry.sources) {
            if (source.startsWith(sourcePrefix)) {
                entry.sources.delete(source);
            }
        }
        if (entry.sources.size !== 0) {
            return undefined;
        }
        this._held.delete(key);
        return entry.action;
    }

    drain (): HeldAction[] {
        const actions = Array.from(this._held.values(), entry => entry.action);
        this._held.clear();
        return actions;
    }
}

/**
 * Keeps one browser `key` value for the complete physical-key lifetime.
 * Logical keys are reference-counted so releasing left Shift cannot release
 * right Shift (and likewise for main-keyboard/numpad equivalents).
 */
export class BrowserKeyState {
    private readonly _active = new Map<string, {key: string, routed: boolean}>();
    private readonly _logicalCounts = new Map<string, number>();

    press (code: string, key: string, location = 0, routeToSimulation = true): string | undefined {
        const physical = this.physicalId(code, key, location);
        const active = this._active.get(physical);
        if (active !== undefined) {
            return undefined;
        }
        this._active.set(physical, {key, routed: routeToSimulation});
        if (!routeToSimulation) {
            return undefined;
        }
        const count = this._logicalCounts.get(key) || 0;
        this._logicalCounts.set(key, count + 1);
        return count === 0 ? key : undefined;
    }

    release (code: string, fallback: string, location = 0): string | undefined {
        let physical = this.physicalId(code, fallback, location);
        let active = this._active.get(physical);

        // `KeyboardEvent.code` is normally stable and non-empty. For virtual
        // keyboards that omit it, recover a modifier-changed release when the
        // location identifies exactly one active physical key.
        if (active === undefined && !code) {
            const prefix = `fallback:${location}:`;
            const candidates = Array.from(this._active.keys()).filter(candidate => candidate.startsWith(prefix));
            if (candidates.length === 1) {
                physical = candidates[0];
                active = this._active.get(physical);
            }
        }
        if (active === undefined) {
            return undefined;
        }

        this._active.delete(physical);
        if (!active.routed) {
            return undefined;
        }
        const key = active.key;
        const count = this._logicalCounts.get(key) || 0;
        if (count > 1) {
            this._logicalCounts.set(key, count - 1);
            return undefined;
        }
        this._logicalCounts.delete(key);
        return key;
    }

    clear (): void {
        this._active.clear();
        this._logicalCounts.clear();
    }

    private physicalId (code: string, key: string, location: number): string {
        return code ? `code:${code}` : `fallback:${location}:${key}`;
    }
}

/** Prevents an older animation-frame release from ending a newer pulse. */
export class OneFramePulseState {
    private readonly _generations = new Map<string, number>();
    private _nextGeneration = 0;

    begin (key: string): number {
        const generation = ++this._nextGeneration;
        this._generations.set(key, generation);
        return generation;
    }

    isActive (key: string): boolean {
        return this._generations.has(key);
    }

    finish (key: string, generation: number): boolean {
        if (this._generations.get(key) !== generation) {
            return false;
        }
        this._generations.delete(key);
        return true;
    }

    clear (): void {
        this._generations.clear();
    }
}
