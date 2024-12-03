import * as mopro_wasm from './mopro-pkg/mopro_wasm.js';

async function fetchBinaryFile(url) {
    const response = await fetch(url);
    if (!response.ok) throw new Error(`Failed to load ${url}`);
    return new Uint8Array(await response.arrayBuffer());
}

async function measureTime(callback) {
    const start = performance.now();
    const result = await callback();
    const end = performance.now();
    return { result, timeTaken: (end - start).toFixed(2) }; // milliseconds
}

async function run_plonk_test(input) {
    try {
        await mopro_wasm.default();
        await mopro_wasm.initThreadPool(navigator.hardwareConcurrency);

        const SRS_KEY = await fetchBinaryFile('./mopro-pkg/parameters/plonk_fibonacci_srs.bin');
        const PROVING_KEY = await fetchBinaryFile('./mopro-pkg/parameters/plonk_fibonacci_pk.bin');
        const VERIFYING_KEY = await fetchBinaryFile('./mopro-pkg/parameters/plonk_fibonacci_vk.bin');
        
        const { result: proofResult, timeTaken: proofTime } = await measureTime(() =>
            mopro_wasm.generate_plonk_proof(SRS_KEY, PROVING_KEY, input)
        );

        const [proof, public_input] = proofResult;
        
        const { result: verifyResult, timeTaken: verifyTime } = await measureTime(() =>
            mopro_wasm.verify_plonk_proof(SRS_KEY, VERIFYING_KEY, proof, public_input)
        );

        console.log(`plonk - proof generation time: ${proofTime} ms, verify proof time: ${verifyTime}`);

        return { isValid: verifyResult, proofTime, verifyTime };
    } catch (error) {
        console.error("Error in run_plonk_test:", error);
        throw error;
    }
}

async function run_hyperplonk_test(input) {
    try {
        await mopro_wasm.default();
        await mopro_wasm.initThreadPool(navigator.hardwareConcurrency);

        const SRS_KEY = await fetchBinaryFile('./mopro-pkg/parameters/hyperplonk_fibonacci_srs.bin');
        const PROVING_KEY = await fetchBinaryFile('./mopro-pkg/parameters/hyperplonk_fibonacci_pk.bin');
        const VERIFYING_KEY = await fetchBinaryFile('./mopro-pkg/parameters/hyperplonk_fibonacci_vk.bin');
        
        const { result: proofResult, timeTaken: proofTime } = await measureTime(() =>
            mopro_wasm.generate_hyperplonk_proof(SRS_KEY, PROVING_KEY, input)
        );

        const [proof, public_input] = proofResult;
        
        const { result: verifyResult, timeTaken: verifyTime } = await measureTime(() =>
            mopro_wasm.verify_hyperplonk_proof(SRS_KEY, VERIFYING_KEY, proof, public_input)
        );

        console.log(`hyperplonk - proof generation time: ${proofTime} ms, verify proof time: ${verifyTime}`);

        return { isValid: verifyResult, proofTime, verifyTime };
    } catch (error) {
        console.error("Error in run_hyperplonk_test:", error);
        throw error;
    }
}


async function run_gemini_test(input) {
    try {
        await mopro_wasm.default();
        await mopro_wasm.initThreadPool(navigator.hardwareConcurrency);

        const SRS_KEY = await fetchBinaryFile('./mopro-pkg/parameters/gemini_fibonacci_srs.bin');
        const PROVING_KEY = await fetchBinaryFile('./mopro-pkg/parameters/gemini_fibonacci_pk.bin');
        const VERIFYING_KEY = await fetchBinaryFile('./mopro-pkg/parameters/gemini_fibonacci_vk.bin');
        
        const { result: proofResult, timeTaken: proofTime } = await measureTime(() =>
            mopro_wasm.generate_gemini_proof(SRS_KEY, PROVING_KEY, input)
        );

        const [proof, public_input] = proofResult;
        
        const { result: verifyResult, timeTaken: verifyTime } = await measureTime(() =>
            mopro_wasm.verify_gemini_proof(SRS_KEY, VERIFYING_KEY, proof, public_input)
        );

        console.log(`gemini - proof generation time: ${proofTime} ms, verify proof time: ${verifyTime}`);

        return { isValid: verifyResult, proofTime, verifyTime };
    } catch (error) {
        console.error("Error in run_gemini_test:", error);
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