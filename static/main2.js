import init from './potato-catcher.js';

function displayPrompt() {
    const info = document.createElement('div');
    info.innerText = 'Click or press any key to start the game and sound';
    info.style.position = 'absolute';
    info.style.top = '50%';
    info.style.left = '50%';
    info.style.transform = 'translate(-50%, -50%)';
    info.style.padding = '10px';
    info.style.backgroundColor = 'rgba(0, 0, 0, 0.8)';
    info.style.color = 'white';
    info.style.borderRadius = '5px';
    info.style.textAlign = 'center';
    document.body.appendChild(info);
    return info;
}

async function runWasm() {
    try {
        // Set up the audio context and audio file
        const audio = new Audio('./hah.ogg');

        // Function to play the audio and initialize WASM after user interaction
        const startApp = async () => {
            // Remove the prompt
            document.body.removeChild(info);
            // Remove event listeners
            document.removeEventListener('click', startApp);
            document.removeEventListener('keydown', startApp);

            try {
                // Play audio
                audio.play();

                // Initialize the WASM module using the init function from wasm-bindgen
                await init('https://storage.googleapis.com/potato-catcher-wasm/potato-catcher_bg.wasm');
                console.log('WASM initialized');
            } catch (err) {
                console.error('Failed to load WASM file or play audio:', err);
            }
        };

        // Add event listeners for user interaction
        document.addEventListener('click', startApp, { once: true });
        document.addEventListener('keydown', startApp, { once: true });

        // Display the prompt to the user
        const info = displayPrompt();
    } catch (err) {
        console.error('Failed to load initial resources:', err);
    }
}

runWasm();
