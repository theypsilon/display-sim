import { assert } from 'chai';
import sinon from 'sinon';
import { EventHandler } from '../../src/services/event_handler';
import { JSDOM } from 'jsdom';

describe('EventHandler', function () {
    before(() => {
        const dom = new JSDOM('<!DOCTYPE html><html><head></head><body><div id="my-id" class="my-class"></div></body></html>');
        global.window = dom.window;
        global.document = dom.window.document;
    });
    after(() => {
        delete global.window;
        delete global.document;
    });
    let sut, spy, element;
    beforeEach(() => {
        sut = new EventHandler();
        spy = sinon.spy();
        element = document.getElementById('my-id');
    });
    describe('listenId', function () {
        it('should call the callback after same type of the event is triggered', function () {
            sut.listenId('click', 'my-id', spy);
            element.click();
            assert.isTrue(spy.calledOnce);
        });
        it('should not call the callback after different type of the event is triggered', function () {
            sut.listenId('input', 'my-id', spy);
            element.click();
            assert.isFalse(spy.called);
        });
    });

    describe('listenClass', function () {
        it('should call the callback after same type of the event is triggered', function () {
            sut.listenClass('click', 'my-class', spy);
            element.click();
            assert.isTrue(spy.calledOnce);
        });
        it('should not call the callback after different type of the event is triggered', function () {
            sut.listenClass('input', 'my-class', spy);
            element.click();
            assert.isFalse(spy.called);
        });
    });

    describe('remove', function () {
        it('should not call the callback after id has been removed', function () {
            sut.listenId('click', 'my-id', spy);
            sut.remove('click', 'my-id');
            element.click();
            assert.isFalse(spy.called);
        });
        it('should not call the callback after class has been removed', function () {
            sut.listenClass('click', 'my-class', spy);
            sut.remove('click', 'my-class');
            element.click();
            assert.isFalse(spy.called);
        });
    });
});