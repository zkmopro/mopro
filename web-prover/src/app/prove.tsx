"use client";
import { useState } from "react";
import { CircuitSignals } from "snarkjs";
import Inputs from "./inputs.json";

const url = "https://mopro.vivianjeng.xyz";

async function getKeys(circuit: string) {
    const wasmUrl = new URL(`${circuit}.wasm`, url).toString();
    const zkeyUrl = new URL(`${circuit}_final.zkey`, url).toString();
    const wasm = await fetch(wasmUrl).then((r) => r.arrayBuffer());
    const zkey = await fetch(zkeyUrl).then((r) => r.arrayBuffer());
    return { wasm, zkey };
}

async function fullProve(circuit: string, inputs: CircuitSignals) {
    const _snarkjs = import("snarkjs");
    const snarkjs = await _snarkjs;
    const { wasm, zkey } = await getKeys(circuit);
    const start = +Date.now();
    const { proof, publicSignals } = await snarkjs.groth16.fullProve(
        inputs,
        new Uint8Array(wasm),
        new Uint8Array(zkey)
    );
    const end = +Date.now();
    return { proof, publicSignals, provingTime: end - start };
}

// async function verifyProof(
//     circuit: string,
//     proof: Groth16Proof,
//     publicSignals: PublicSignals
// ) {
//     const _snarkjs = import("snarkjs");
//     const vkeyUrl = new URL(`${circuit}.vkey.json`, url).toString();
//     const vkeyBuffer = await fetch(vkeyUrl).then((r) => r.arrayBuffer());
//     const vkeyString = String.fromCharCode.apply(
//         null,
//         new Uint8Array(vkeyBuffer) as any
//     );
//     const vkey = JSON.parse(vkeyString);
//     const snarkjs = await _snarkjs;
//     const start = +Date.now();
//     const valid = await snarkjs.groth16.verify(vkey, publicSignals, proof);
//     const end = +Date.now();
//     return { valid: valid, verifyingTime: end - start };
// }

export default function Prove(props: any) {
    const [proving, setProving] = useState<boolean>(false);
    const [provingTime, setProvingTime] = useState<string>("");
    const [proof, setProof] = useState<string>();
    const [publicSignals, setPublicSignals] = useState<string>("");
    const { file, inputs } = Inputs[props.circuit as keyof typeof Inputs];

    async function generateProof() {
        setProving(true);
        setProvingTime("Calculating...");
        const { proof, publicSignals, provingTime } = await fullProve(
            file,
            inputs
        );
        setProvingTime(`${provingTime / 1000} s`);
        setProof(JSON.stringify(proof));
        setPublicSignals(JSON.stringify(publicSignals));
        setProving(false);
    }

    return (
        <div>
            <div className="mb-4 mt-8">
                <h2 className="fix text-2xl font-bold mb-4">{props.circuit}</h2>
                <button
                    disabled={proving}
                    className="btn mr-4 text-slate-200 p-1 pl-3 pr-3 rounded-lg bg-[#0062c1] hover:bg-[#319aff] disabled:bg-[#319aff] disabled:cursor-not-allowed shadow-md"
                    onClick={generateProof}
                >
                    Prove
                </button>
                {/* <button className="btn">Verify</button> */}
                {provingTime && (
                    <p className="mt-2">Proving time: {provingTime}</p>
                )}
            </div>
            {proof && publicSignals && (
                <div className="bg-sky-200 dark:bg-blue-950 p-5 rounded-md shadow-md">
                    {proof && <h3 className="text-1xl font-bold">proof</h3>}
                    <p className="mt-1 break-all ">{proof}</p>
                    {publicSignals && (
                        <h3 className="text-1xl font-bold">public signals</h3>
                    )}
                    <p className="mt-1 break-all ">{publicSignals}</p>
                </div>
            )}
        </div>
    );
}
