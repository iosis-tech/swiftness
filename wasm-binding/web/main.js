import init, { wasm_verify } from '../pkg/wasm_binding.js';

async function run() {
    await init();  // Initialize the Wasm module

    document.getElementById('verify-button').addEventListener('click', async () => {
        const fileInput = document.getElementById('file-input');
        const file = fileInput.files[0];

        if (!file) {
            alert('Please select a file.');
            return;
        }

        const reader = new FileReader();
        reader.onload = async function() {
            const proof = reader.result;
            try {
                const startTime = Date.now(); // Start timing
                const result = await wasm_verify(proof);
                const endTime = Date.now(); // End timing
                const elapsedTime = endTime - startTime; // Calculate elapsed time

                const parsedResult = JSON.parse(result);
                
                // Display program_hash and output_hash
                document.getElementById('program-hash').innerText = `Program Hash: ${parsedResult[0]}`;
                document.getElementById('output-hash').innerText = `Output Hash: ${parsedResult[1]}`;
                document.getElementById('verification-time').innerText = `Verification Time: ${elapsedTime} ms`;
            } catch (err) {
                console.error(err);
                document.getElementById('output').innerText = `Verification failed: ${err}`;
            }
        };
        reader.readAsText(file);
    });
}

run();
