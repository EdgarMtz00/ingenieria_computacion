export type CircularBuffer = {
    array: Array<number>,
    putIndex: number,
    takeIndex: number,
    size: number,
}

export function initCircularBuffer(size: number): CircularBuffer {
    return {
        array: new Array<number>(size).fill(0),
        putIndex: 0,
        takeIndex: 0,
        size,
    }
}

export function circularBufferPut(buffer: CircularBuffer) {
    return (value: number) => {
        buffer.array[buffer.putIndex] = value;
        buffer.putIndex = (buffer.putIndex + 1) % buffer.size;
    }
}

export function circularBufferCanPut(buffer: CircularBuffer) {
    return () => {
        return (buffer.putIndex + 1) % buffer.size !== buffer.takeIndex;
    }
}

export function circularBufferTake(buffer: CircularBuffer): () => number {
    return () => {
        const value = buffer.array[buffer.takeIndex];
        buffer.array[buffer.takeIndex] = 0;
        buffer.takeIndex = (buffer.takeIndex + 1) % buffer.size;
        return value;
    }
}

export function circularBufferCanTake(buffer: CircularBuffer) {
    return () => {
        return buffer.putIndex !== buffer.takeIndex;
    }
}
