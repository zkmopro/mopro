import * as halo2_plonk_wasm from './halo2-plonk-fibonacci/mopro_wasm.js';
import * as halo2_hyperplonk_wasm from './halo2-hyperplonk-fibonacci/mopro_wasm.js';
import * as halo2_gemini_wasm from './halo2-gemini-fibonacci/mopro_wasm.js';

async function fetchBinaryFile(url) {
    const response = await fetch(url);
    if (!response.ok) throw new Error(`Failed to load ${url}`);
    return new Uint8Array(await response.arrayBuffer());
}

async function run_plonk_test(input) {
    try {
        await halo2_plonk_wasm.default();

        const SRS_KEY = await fetchBinaryFile('./halo2-plonk-fibonacci/parameters/plonk_fibonacci_srs.bin');
        const PROVING_KEY = await fetchBinaryFile('./halo2-plonk-fibonacci/parameters/plonk_fibonacci_pk.bin');
        const VERIFYING_KEY = await fetchBinaryFile('./halo2-plonk-fibonacci/parameters/plonk_fibonacci_vk.bin');
        
        const result = await halo2_plonk_wasm.generate_proof(SRS_KEY, PROVING_KEY, input);

        let proof = result[0];
        let public_input = result[1];
        
        const IsValidProof = await halo2_plonk_wasm.verify_proof(SRS_KEY, VERIFYING_KEY, proof, public_input);

        return IsValidProof;
    } catch (error) {
        console.error("Error in run_test:", error);
        throw error;
    }
}

async function run_hyperplonk_test(input) {
    try {
        await halo2_hyperplonk_wasm.default();

        const SRS_KEY = await fetchBinaryFile('./halo2-hyperplonk-fibonacci/parameters/hyperplonk_fibonacci_srs.bin');
        const PROVING_KEY = await fetchBinaryFile('./halo2-hyperplonk-fibonacci/parameters/hyperplonk_fibonacci_pk.bin');
        const VERIFYING_KEY = await fetchBinaryFile('./halo2-hyperplonk-fibonacci/parameters/hyperplonk_fibonacci_vk.bin');
        
        const result = await halo2_hyperplonk_wasm.generate_proof(SRS_KEY, PROVING_KEY, input);

        let proof = result[0];
        let public_input = result[1];

        const IsValidProof = await halo2_hyperplonk_wasm.verify_proof(SRS_KEY, VERIFYING_KEY, proof, public_input);

        return IsValidProof;
    } catch (error) {
        console.error("Error in run_test:", error);
        throw error;
    }
}


async function run_gemini_test(input) {
    try {
        await halo2_gemini_wasm.default();

        const SRS_KEY = await fetchBinaryFile('./halo2-gemini-fibonacci/parameters/gemini_fibonacci_srs.bin');
        const PROVING_KEY = await fetchBinaryFile('./halo2-gemini-fibonacci/parameters/gemini_fibonacci_pk.bin');
        const VERIFYING_KEY = await fetchBinaryFile('./halo2-gemini-fibonacci/parameters/gemini_fibonacci_vk.bin');
        
        const result = await halo2_gemini_wasm.generate_proof(SRS_KEY, PROVING_KEY, input);

        let proof = result[0];
        let public_input = result[1];

        const IsValidProof = await halo2_gemini_wasm.verify_proof(SRS_KEY, VERIFYING_KEY, proof, public_input);

        return IsValidProof;
    } catch (error) {
        console.error("Error in run_test:", error);
        throw error;
    }
}


// Handle messages from the main thread
self.addEventListener('message', async (event) => {
    const { testName, input } = event.data;
    try {
        if (testName === 'Plonk') {
            const result = await run_plonk_test(input);
            self.postMessage({ testName, data: result });
        } else if (testName == 'HyperPlonk') {
            const result = await run_hyperplonk_test(input);
            self.postMessage({ testName, data: result });
        } else if (testName == 'Gemini') {
            const result = await run_gemini_test(input);
            self.postMessage({ testName, data: result });
        }
    } catch (error) {
        self.postMessage({ testName, error: error.message });
    }
});