import {
    circularBufferCanPut,
    circularBufferCanTake,
    circularBufferPut,
    circularBufferTake,
    initCircularBuffer
} from "./circular_buffer";
import {initLock, lockAcquire} from "./semaphore";

const buffer = initCircularBuffer(25);
const acquire = lockAcquire(initLock())
const put = circularBufferPut(buffer);
const take = circularBufferTake(buffer);
const canTake = circularBufferCanTake(buffer);
const canPut = circularBufferCanPut(buffer);
const print = bufferPrinter(buffer.array);

function randomInt(max: number) {
    return Math.floor(Math.random() * Math.floor(max));
}

async function sleep(number: number) {
    return new Promise(resolve => setTimeout(resolve, number));
}

function bufferPrinter(buffer: Array<number>){
    return (state: string) => {
        console.clear();
        console.log(state);
        console.log(buffer);
    }
}

async function producer() {
    for (let i = 0; i < 1000; i++) {
        // esperar al semaforo
        print(`Producer: Esperando\nConsumer: Consumiendo`);
        const release = await acquire();
        // si hay espacio en el buffer
        if (canPut()) {
            let quantity = randomInt(5) + 1;
            print(`Producer: Produciendo(${quantity})\nConsumer: Esperando`);
            // iniciar accion de entre 500 y 1000 ms
            setTimeout(() => {
                for (let j = 0; j < quantity; j++) {
                    if (canPut()){
                        put(randomInt(9)+1);
                    }
                }
                print(`Producer: Produciendo(${quantity})\nConsumer: Esperando`);
                // liberar el semaforo
                release();
            }, randomInt(500) + 500);
        } else {
            // si no hay espacio en el buffer imprimir y liberar el semaforo
            print('Producer: Buffer lleno\nConsumer: Esperando');
            release();
        }
        // dormir entre 1000 y 1500 ms
        console.log('Producer: Durmiendo');
        await sleep(randomInt(500) + 1000);
    }
}

async function consumer() {
    for (let i = 1; i <= 1000; i++) {
        print(`Producer: Produciendo\nConsumer: Esperando`);
        const release = await acquire();
        if (canTake()) {
            let quantity = randomInt(5) + 1;
            print(`Producer: Esperando\nConsumer: Consumiendo(${quantity})`);
            setTimeout(() => {
                for (let j = 0; j < quantity; j++) {
                    if(canTake()) {
                        take();
                    }
                }
                print(`Producer: Esperando\nConsumer: Consumiendo(${quantity})`);
                release();
            }, randomInt(500) + 500);
        } else {
            print('Producer: Esperando\nConsumer: Buffer vacio');
            release();
        }
        console.log('Consumer: Durmiendo');
        await sleep(randomInt(500) + 1000);
    }
}

function main() {
    producer().then(() => console.log('Producer: Terminado'));
    consumer().then(() => console.log('Consumer: Terminado'));
}

main();