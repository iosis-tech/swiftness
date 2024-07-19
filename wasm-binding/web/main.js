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
                const result = await wasm_verify(proof);
                document.getElementById('output').innerText = JSON.stringify(result, null, 2);
            } catch (err) {
                console.error(err);
                document.getElementById('output').innerText = `Verification failed: ${err}`;
            }
        };
        reader.readAsText(file);
    });
}

run();