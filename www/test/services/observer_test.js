import { assert } from 'chai';
import sinon from 'sinon';
import { Observer } from '../../src/services/observer';

describe('Observer', () => {
    let sut, spy;
    beforeEach(() => {
        sut = Observer.make();
        spy = sinon.spy();
    });

    it('should call once the subscribed callback after calling once method fire', () => {
        sut.subscribe(spy);
        sut.fire();
        assert.isTrue(spy.calledOnce);
    });

    it('should call thrice the subscribed callback after calling the method fire 3 times', () => {
        sut.subscribe(spy);
        sut.fire();
        sut.fire();
        sut.fire();
        assert.isTrue(spy.calledThrice);
    });

    it('should call the subscribed callback with the arguments given to the method fire', () => {
        sut.subscribe(spy);
        sut.fire('called1');
        sut.fire('called2');
        assert.isTrue(spy.calledTwice);
        assert.isTrue(spy.firstCall.calledWith('called1'));
        assert.isTrue(spy.secondCall.calledWith('called2'));
    });

    it('should call all the callbacks when the method fire is called', () => {
        const spy1 = spy;
        const spy2 = sinon.spy();
        sut.subscribe(spy1);
        sut.subscribe(spy2);
        sut.fire();
        assert.isTrue(spy1.calledOnce);
        assert.isTrue(spy2.calledOnce);
    });

    it('should call all the callbacks at all times with the proper arguments when the method fire is called', () => {
        const spy1 = spy;
        const spy2 = sinon.spy();
        sut.subscribe(spy1);
        sut.subscribe(spy2);
        sut.fire('call1');
        sut.fire('call2');
        assert.isTrue(spy1.calledTwice);
        assert.isTrue(spy2.calledTwice);
        assert.isTrue(spy1.firstCall.calledWith('call1'));
        assert.isTrue(spy1.secondCall.calledWith('call2'));
        assert.isTrue(spy2.firstCall.calledWith('call1'));
        assert.isTrue(spy2.secondCall.calledWith('call2'));
    });
});