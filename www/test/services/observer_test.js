import { assert } from 'chai';
import * as sinon from 'sinon';
import { Observable } from '../../src/services/observable';

describe('Observer', () => {
    let sut, spy;
    beforeEach(() => {
        sut = Observable.make();
        spy = sinon.spy();
    });

    it('should call once the subscribed callback after calling once method fire', async () => {
        sut.subscribe(spy);
        await sut.fire();
        assert.isTrue(spy.calledOnce);
    });

    it('should not call the subscribed callback after being unsubscribed', async () => {
        sut.subscribe(spy);
        sut.unsubscribe(spy);
        await sut.fire();
        assert.isTrue(spy.notCalled);
    });

    it('should call thrice the subscribed callback after calling the method fire 3 times', async () => {
        sut.subscribe(spy);
        await sut.fire();
        await sut.fire();
        await sut.fire();
        assert.isTrue(spy.calledThrice);
    });

    it('should call the subscribed callback with the arguments given to the method fire', async () => {
        sut.subscribe(spy);
        await sut.fire('called1');
        await sut.fire('called2');
        assert.isTrue(spy.calledTwice);
        assert.isTrue(spy.firstCall.calledWith('called1'));
        assert.isTrue(spy.secondCall.calledWith('called2'));
    });

    it('should call all the callbacks when the method fire is called', async () => {
        const spy1 = spy;
        const spy2 = sinon.spy();
        sut.subscribe(spy1);
        sut.subscribe(spy2);
        await sut.fire();
        assert.isTrue(spy1.calledOnce);
        assert.isTrue(spy2.calledOnce);
    });

    it('should call all the callbacks at all times with the proper arguments when the method fire is called', async () => {
        const spy1 = spy;
        const spy2 = sinon.spy();
        sut.subscribe(spy1);
        sut.subscribe(spy2);
        await sut.fire('call1');
        await sut.fire('call2');
        assert.isTrue(spy1.calledTwice);
        assert.isTrue(spy2.calledTwice);
        assert.isTrue(spy1.firstCall.calledWith('call1'));
        assert.isTrue(spy1.secondCall.calledWith('call2'));
        assert.isTrue(spy2.firstCall.calledWith('call1'));
        assert.isTrue(spy2.secondCall.calledWith('call2'));
    });
});