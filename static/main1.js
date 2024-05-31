import init from './potato-catcher.js';

async function runWasm() {
    try {
        await init('https://storage.googleapis.com/potato-catcher-wasm/potato-catcher_bg.wasm');

        // Set up the audio context and audio file
        const audio = new Audio('./static/hah.ogg');

        // Function to play the audio after user interaction
        const playAudio = () => {
            audio.play();
        };

        // Add an event listener to start the audio on user interaction
        document.addEventListener('click', playAudio, { once: true });
        document.addEventListener('keydown', playAudio, { once: true });

        // Inform the user to interact with the page
        const info = document.createElement('div');
        info.innerText = 'Click or press any key to start the game and sound';
        document.body.appendChild(info);
    } catch (err) {
        console.error('Failed to load WASM file:', err);
    }
}

runWasm();
