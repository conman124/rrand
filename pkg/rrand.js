import * as wasm from './rrand_bg';

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

function freeRRand(ptr) {

    wasm.__wbg_rrand_free(ptr);
}
/**
*/
export class RRand {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeRRand(ptr);
    }

    /**
    * @param {number} seed
    * @returns {}
    */
    constructor(seed) {
        this.ptr = wasm.rrand_new(seed);
    }
    /**
    * @returns {number}
    */
    next() {
        return wasm.rrand_next(this.ptr) >>> 0;
    }
    /**
    * @param {number} upper
    * @returns {number}
    */
    next_range(upper) {
        return wasm.rrand_next_range(this.ptr, upper) >>> 0;
    }
}

