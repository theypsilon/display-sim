import { assert } from 'chai';
import sinon from 'sinon';
import { EventHandler } from '../../src/services/event_handler';
import { JSDOM } from 'jsdom';

describe('EventHandler', function () {
    before(() => {
        global.window = new JSDOM('<!DOCTYPE html><html><head></head><body><div id="my-id" class="my-class"></div></body></html>').window;
    });
    after(() => {
        delete global.window;
    });
    let sut, spy, element;
    beforeEach(() => {
        sut = new EventHandler();
        spy = sinon.spy();
        element = window.document.getElementById('my-id');
    });
    describe('subscribeId', function () {
        it('should call the callback after same type of the event is triggered', function () {
            sut.subscribeId('click', 'my-id', spy);
            element.click();
            assert.isTrue(spy.calledOnce);
        });
        it('should not call the callback after different type of the event is triggered', function () {
            sut.subscribeId('input', 'my-id', spy);
            element.click();
            assert.isFalse(spy.called);
        });
        it('should call the callback passing the event as parameter', function () {
            sut.subscribeId('click', 'my-id', spy);
            element.click();
            assert.equal(spy.args[0][0].target, element);
        });
    });

    describe('subscribeClass', function () {
        it('should call the callback after same type of the event is triggered', function () {
            sut.subscribeClass('click', 'my-class', spy);
            element.click();
            assert.isTrue(spy.calledOnce);
        });
        it('should not call the callback after different type of the event is triggered', function () {
            sut.subscribeClass('input', 'my-class', spy);
            element.click();
            assert.isFalse(spy.called);
        });
        it('should call the callback passing the event as parameter', function () {
            sut.subscribeClass('click', 'my-class', spy);
            element.click();
            assert.equal(spy.args[0][0].target, element);
        });
    });

    describe('remove', function () {
        it('should not call the callback after id has been removed', function () {
            sut.subscribeId('click', 'my-id', spy);
            sut.remove('click', 'my-id');
            element.click();
            assert.isFalse(spy.called);
        });
        it('should not call the callback after class has been removed', function () {
            sut.subscribeClass('click', 'my-class', spy);
            sut.remove('click', 'my-class');
            element.click();
            assert.isFalse(spy.called);
        });
    });
});