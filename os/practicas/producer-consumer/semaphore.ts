export type lock = {
    locked: boolean,
    waiters: Array<() => void>,
}

export function initLock(): lock {
    return {
        locked: false,
        waiters: [],
    }
}

export function lockAcquire(lock: lock): () => Promise<() => void> {
    return () => {
        return new Promise((resolve) => {
            if (lock.locked) {
                lock.waiters.push(() => resolve(lockRelease(lock)));
            } else {
                lock.locked = true;
                resolve(lockRelease(lock));
            }
        })
    }
}

export function lockRelease(lock: lock): () => void {
    return () => {
        if (lock.waiters.length > 0) {
            // @ts-ignore
            lock.waiters.shift()();
        } else {
            lock.locked = false;
        }
    }
}
