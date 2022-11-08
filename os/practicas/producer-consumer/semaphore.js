"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.lockRelease = exports.lockAcquire = exports.initLock = void 0;
function initLock() {
    return {
        locked: false,
        waiters: [],
    };
}
exports.initLock = initLock;
function lockAcquire(lock) {
    return () => {
        return new Promise((resolve, reject) => {
            if (lock.locked) {
                lock.waiters.push(() => resolve(lockRelease(lock)));
            }
            else {
                lock.locked = true;
                resolve(lockRelease(lock));
            }
        });
    };
}
exports.lockAcquire = lockAcquire;
function lockRelease(lock) {
    return () => {
        if (lock.waiters.length > 0) {
            // @ts-ignore
            lock.waiters.shift()();
        }
        else {
            lock.locked = false;
        }
    };
}
exports.lockRelease = lockRelease;
