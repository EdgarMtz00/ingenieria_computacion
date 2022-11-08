"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.circularBufferCanTake = exports.circularBufferTake = exports.circularBufferCanPut = exports.circularBufferPut = exports.initCircularBuffer = void 0;
function initCircularBuffer(size) {
    return {
        array: new Array(size).fill(0),
        putIndex: 0,
        takeIndex: 0,
        size,
    };
}
exports.initCircularBuffer = initCircularBuffer;
function circularBufferPut(buffer) {
    return (value) => {
        buffer.array[buffer.putIndex] = value;
        buffer.putIndex = (buffer.putIndex + 1) % buffer.size;
    };
}
exports.circularBufferPut = circularBufferPut;
function circularBufferCanPut(buffer) {
    return () => {
        return (buffer.putIndex + 1) % buffer.size !== buffer.takeIndex;
    };
}
exports.circularBufferCanPut = circularBufferCanPut;
function circularBufferTake(buffer) {
    return () => {
        const value = buffer.array[buffer.takeIndex];
        buffer.array[buffer.takeIndex] = 0;
        buffer.takeIndex = (buffer.takeIndex + 1) % buffer.size;
        return value;
    };
}
exports.circularBufferTake = circularBufferTake;
function circularBufferCanTake(buffer) {
    return () => {
        return buffer.putIndex !== buffer.takeIndex;
    };
}
exports.circularBufferCanTake = circularBufferCanTake;
