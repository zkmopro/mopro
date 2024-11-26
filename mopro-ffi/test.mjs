import { generate_proof, verify_proof } from "./pkg/mopro_ffi.js";
import fs from "fs";
// Function to convert file content at a path to Uint8Array
function pathToUint8Array(filePath) {
    const buffer = fs.readFileSync(filePath); // Read the file as a Buffer
    return new Uint8Array(buffer); // Convert the Buffer to a Uint8Array
}

async function run() {
    const srs = "../test-vectors/halo2/plonk_fibonacci_srs.bin";
    const pk = "../test-vectors/halo2/plonk_fibonacci_pk.bin";
    const vk = "../test-vectors/halo2/plonk_fibonacci_vk.bin";
    const input = {
        out: ["55"],
    };
    const start = Date.now();
    const output = generate_proof(
        pathToUint8Array(srs),
        pathToUint8Array(pk),
        input
    );
    const end = Date.now();
    console.log(`Proof time: ${end - start} ms`);
    const proof = output[0];
    const publicInputs = output[1];

    const valid = verify_proof(
        pathToUint8Array(srs),
        pathToUint8Array(vk),
        proof,
        publicInputs
    );

    return valid;
}

run().then((t) => console.log(t));
