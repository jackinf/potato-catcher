import init from './potato-catcher.js';

async function runWasm() {
    try {
        // Initialize the WASM module using the init function from wasm-bindgen
        await init('https://storage.googleapis.com/potato-catcher-wasm/potato-catcher_bg.wasm');
    } catch (err) {
        console.error('Failed to load WASM file:', err);
    }
}

runWasm();
