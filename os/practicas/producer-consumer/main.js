"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const circular_buffer_1 = require("./circular_buffer");
const semaphore_1 = require("./semaphore");
const buffer = (0, circular_buffer_1.initCircularBuffer)(25);
const acquire = (0, semaphore_1.lockAcquire)((0, semaphore_1.initLock)());
const put = (0, circular_buffer_1.circularBufferPut)(buffer);
const take = (0, circular_buffer_1.circularBufferTake)(buffer);
const canTake = (0, circular_buffer_1.circularBufferCanTake)(buffer);
const canPut = (0, circular_buffer_1.circularBufferCanPut)(buffer);
function randomInt(max) {
    return Math.floor(Math.random() * Math.floor(max));
}
function heavyWork() {
    setTimeout(() => {
    }, randomInt(1000));
}
function putOrTake(i) {
    return __awaiter(this, void 0, void 0, function* () {
        const release = yield acquire();
        if (randomInt(2) === 0) {
            if (canPut()) {
                heavyWork();
                put(i);
            }
        }
        else {
            if (canTake()) {
                heavyWork();
                take();
            }
        }
        release();
    });
}
function main() {
    for (let i = 1; i <= 100; i++) {
        putOrTake(i).then(r => console.log(buffer.array));
    }
}
main();
