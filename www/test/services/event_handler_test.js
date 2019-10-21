import assert from 'assert';
import { EventHandler } from '../../src/services/event_handler';
import { JSDOM } from 'jsdom';

const dom = new JSDOM('<!DOCTYPE html><html><head></head><body><div id="my-id" class="my-class"></div></body></html>');
global.window = dom.window;
global.document = dom.window.document;

describe('EventHandler', function () {
    let sut;
    beforeEach(() => {
        sut = new EventHandler();
    });
    const element = document.getElementById('my-id');
    describe('listen', function () {
        it('should call the callback after same type of the event is triggered', function () {
            let actual = 0;
            sut.listen('click', 'my-id', () => { actual = 42; });
            element.click();
            assert.strictEqual(actual, 42);
        });
        it('should not call the callback after different type of the event is triggered', function () {
            let actual = 0;
            sut.listen('input', 'my-id', () => { actual = 42; });
            element.click();
            assert.strictEqual(actual, 0);
        });
    });

    describe('listenMatch', function () {
        it('should call the callback after same type of the event is triggered', function () {
            let actual = 0;
            sut.listenMatch('click', '.my-class', () => { actual = 42; });
            element.click();
            assert.strictEqual(actual, 42);
        });
        it('should not call the callback after different type of the event is triggered', function () {
            let actual = 0;
            sut.listenMatch('input', '.my-class', () => { actual = 42; });
            element.click();
            assert.strictEqual(actual, 0);
        });
    });

    describe('remove', function () {
        it('should not call the callback after id has been removed', function () {
            let actual = 0;
            sut.listen('click', 'my-id', () => { actual = 42; });
            sut.remove('click', 'my-id');
            element.click();
            assert.strictEqual(actual, 0);
        });
        it('should not call the callback after class has been removed', function () {
            let actual = 0;
            sut.listenMatch('click', '.my-class', () => { actual = 42; });
            sut.remove('click', '.my-class');
            element.click();
            assert.strictEqual(actual, 0);
        });
    });
});