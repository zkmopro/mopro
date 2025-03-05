// Initialize the WASM module and thread pool
async function initializeWasm() {
    try {
        const mopro_wasm = await import('./MoproWasmBindings/mopro_wasm_lib.js');
        await mopro_wasm.default();
        await mopro_wasm.initThreadPool(navigator.hardwareConcurrency);
        return mopro_wasm;
    } catch (error) {
        console.error("Failed to initialize WASM module or thread pool:", error);
        throw error;
    }
}

// Fetch binary file
async function fetchBinaryFile(url) {
    const response = await fetch(url);
    if (!response.ok) throw new Error(`Failed to load ${url}`);
    return new Uint8Array(await response.arrayBuffer());
}

// Measure the execution time of a given callback
async function measureTime(callback) {
    const start = performance.now();
    const result = await callback();
    const end = performance.now();
    return { result, timeTaken: (end - start).toFixed(2) }; // milliseconds
}

// Run a specific test
async function runTest(testName, input, srs, pk, vk, generateProof, verifyProof) {
    try {
        const SRS_KEY = await fetchBinaryFile(srs);
        const PROVING_KEY = await fetchBinaryFile(pk);
        const VERIFYING_KEY = await fetchBinaryFile(vk);

        const { result: proofResult, timeTaken: proofTime } = await measureTime(() =>
            generateProof(SRS_KEY, PROVING_KEY, input)
        );

        const [proof, public_input] = proofResult;

        const { result: verifyResult, timeTaken: verifyTime } = await measureTime(() =>
            verifyProof(SRS_KEY, VERIFYING_KEY, proof, public_input)
        );

        return { isValid: verifyResult, proofTime, verifyTime };
    } catch (error) {
        console.error(`Error during ${testName} test:`, error);
        throw error;
    }
}

// Finalize the test suite and display the final status
function finalizeTests(allPassed, statusDiv) {
    const finalStatus = allPassed ? "All tests passed" : "Some tests failed";
    statusDiv.textContent = `Test Status: ${finalStatus}`;
    statusDiv.dataset.status = allPassed ? "passed" : "failed";
}

// Update the test results in the table
function updateResults(testName, data, resultsTable, allPassedRef, error = null) {
    let row = document.getElementById(testName);
    if (!row) {
        row = document.createElement('tr');
        row.id = testName;
        row.innerHTML = `
            <td>${testName}</td>
            <td id="${testName}-proof-time"></td>
            <td id="${testName}-verify-time"></td>
            <td id="${testName}-pass"></td>
            <td id="${testName}-error"></td>
        `;
        resultsTable.appendChild(row);
    }

    if (error) {
        allPassedRef.value = false; // Use an object to maintain a mutable reference to `allPassed`
        document.getElementById(`${testName}-error`).textContent = error;
    } else {
        document.getElementById(`${testName}-proof-time`).textContent = data.proofTime;
        document.getElementById(`${testName}-verify-time`).textContent = data.verifyTime;
        document.getElementById(`${testName}-pass`).textContent = data.isValid ? "true" : "false";
        if (!data.isValid) allPassedRef.value = false;
    }
}

// Main function to initialize and run the tests
(async function () {
    // Initialize WASM
    const mopro_wasm = await initializeWasm();

    const testCases = [
        {
            name: "Plonk",
            input: { out: ["55"] },
            srs: './assets/plonk_fibonacci_srs.bin',
            pk: './assets/plonk_fibonacci_pk.bin',
            vk: './assets/plonk_fibonacci_vk.bin',
            generateProof: mopro_wasm.generate_plonk_proof,
            verifyProof: mopro_wasm.verify_plonk_proof,
        },
        {
            name: "HyperPlonk",
            input: { out: ["55"] },
            srs: './assets/hyperplonk_fibonacci_srs.bin',
            pk: './assets/hyperplonk_fibonacci_pk.bin',
            vk: './assets/hyperplonk_fibonacci_vk.bin',
            generateProof: mopro_wasm.generate_hyperplonk_proof,
            verifyProof: mopro_wasm.verify_hyperplonk_proof,
        },
        {
            name: "Gemini",
            input: { out: ["55"] },
            srs: './assets/gemini_fibonacci_srs.bin',
            pk: './assets/gemini_fibonacci_pk.bin',
            vk: './assets/gemini_fibonacci_vk.bin',
            generateProof: mopro_wasm.generate_gemini_proof,
            verifyProof: mopro_wasm.verify_gemini_proof,
        }
    ];

    const resultsTable = document.getElementById('test-results');
    const statusDiv = document.getElementById('test-status');

    let currentIndex = 0;
    const allPassedRef = { value: true };

    // Run each test sequentially
    while (currentIndex < testCases.length) {
        const testCase = testCases[currentIndex];
        try {
            const data = await runTest(
                testCase.name,
                testCase.input,
                testCase.srs,
                testCase.pk,
                testCase.vk,
                testCase.generateProof,
                testCase.verifyProof
            );
            updateResults(testCase.name, data, resultsTable, allPassedRef);
        } catch (error) {
            updateResults(testCase.name, null, resultsTable, allPassedRef, error.message);
        }
        currentIndex++;
    }

    // Finalize the tests
    finalizeTests(allPassedRef.value, statusDiv);
})();
