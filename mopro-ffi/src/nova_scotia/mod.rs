#[macro_export]
macro_rules! nova_scotia_app {
    () => {
        fn generate_recursive_snark_proof(
            witness_generator_file: PathBuf,
            r1cs: circom::circuit::R1CS<F<G1>>,
            private_inputs: Vec<HashMap<String, serde_json::Value>>,
            start_public_input: [F<G1>; 2],
            pp: &PublicParams<G1, G2, C1<G1>, C2<G2>>,
        ) -> Result<RecursiveSNARK<G1, G2, C1<G1>, C2<G2>>, mopro_ffi::MoproError> {
            let res = create_recursive_circuit(
                FileLocation::PathBuf(witness_generator_file),
                r1cs,
                private_inputs,
                start_public_input.to_vec(),
                &pp,
            );

            res.map_err(|e| {
                mopro_ffi::MoproError::NovaScotiaError(format!("nova_scotia error: {}", e))
            })
        }

        fn verify_recursive_snark_proof(
            recursive_snark: &RecursiveSNARK<G1, G2, C1<G1>, C2<G2>>,
            pp: &PublicParams<G1, G2, C1<G1>, C2<G2>>,
            iteration_count: usize,
            start_public_input: [F<G1>; 2],
            z0_secondary: [F<G2>; 1],
        ) -> Result<(Vec<F<G1>>, Vec<F<G2>>), mopro_ffi::MoproError> {
            let res = recursive_snark.verify(
                &pp,
                iteration_count,
                &start_public_input.clone(),
                &z0_secondary,
            );

            res.map_err(|e| {
                mopro_ffi::MoproError::NovaScotiaError(format!("error verifying proof: {}", e))
            })
        }

        fn compress_snark_proof(
            recursive_snark: RecursiveSNARK<G1, G2, C1<G1>, C2<G2>>,
            pp: &PublicParams<G1, G2, C1<G1>, C2<G2>>,
            pk: &ProverKey<G1, G2, C1<G1>, C2<G2>, S<G1>, S<G2>>,
        ) -> Result<CompressedSNARK<G1, G2, C1<G1>, C2<G2>, S<G1>, S<G2>>, mopro_ffi::MoproError> {
            let res =
                CompressedSNARK::<_, _, _, _, S<G1>, S<G2>>::prove(&pp, &pk, &recursive_snark);

            res.map_err(|e| {
                mopro_ffi::MoproError::NovaScotiaError(format!("error compress proof: {}", e))
            })
        }

        fn verify_compressed_snark_proof(
            compressed_snark: CompressedSNARK<G1, G2, C1<G1>, C2<G2>, S<G1>, S<G2>>,
            vk: &VerifierKey<G1, G2, C1<G1>, C2<G2>, S<G1>, S<G2>>,
            iteration_count: usize,
            start_public_input: [F<G1>; 2],
            z0_secondary: [F<G2>; 1],
        ) -> Result<(Vec<F<G1>>, Vec<F<G2>>), mopro_ffi::MoproError> {
            let res = compressed_snark.verify(
                &vk,
                iteration_count,
                start_public_input.clone().to_vec(),
                z0_secondary.to_vec(),
            );

            res.map_err(|e| {
                mopro_ffi::MoproError::NovaScotiaError(format!("error verifying proof: {}", e))
            })
        }
    };
}

#[cfg(test)]
mod test {
    use crate as mopro_ffi;
    use serde_json::json;
    use std::collections::HashMap;
    use std::path::PathBuf;

    use nova_scotia::*;

    use nova_snark::{
        provider, CompressedSNARK, ProverKey, PublicParams, RecursiveSNARK, VerifierKey,
    };

    nova_scotia_app!();

    const R1CS_PATH: &str = "../test-vectors/nova_scotia/fibonacci.r1cs";
    const WASM_PATH: &str = "../test-vectors/nova_scotia/fibonacci.wasm";

    // Define curve cycle, can be any curve cycle supported by Nova
    type G1 = provider::bn256_grumpkin::bn256::Point;
    type G2 = provider::bn256_grumpkin::grumpkin::Point;

    #[test]
    fn test_generate_and_verify_nova_scotia_proof() {
        let root = std::env::current_dir().unwrap();

        // Load r1cs file
        let circuit_file = root.join(R1CS_PATH.to_string());
        let r1cs = circom::reader::load_r1cs::<G1, G2>(&FileLocation::PathBuf(circuit_file));

        // Load c++ binary or wasm file
        let witness_generator_file = root.join(WASM_PATH.to_string());

        // Generate private inputs
        /*
        each folding steps (step_in[0], step_in[1]):
        step_out[0] <== step_in[0] + adder;
        step_out[1] <== step_in[0] + step_in[1];

        adder is the private input (auxiliary input) that we have.

        step_in[0], step_in[1], adder
            10,        10,        0
            10,        20,        1
            11,        30,        2
            13,        41,        3
            16,        54,        4
            20,        70,        5 <-- state of things when we output results

        */
        let iteration_count = 5;
        let mut private_inputs = Vec::new();
        for i in 0..iteration_count {
            let mut private_input = HashMap::new();
            private_input.insert("adder".to_string(), json!(i));
            private_inputs.push(private_input);
        }

        // Set starting public input
        let start_public_input = [F::<G1>::from(10), F::<G1>::from(10)];

        // Create public parameters(CRS)
        let pp = create_public_params::<G1, G2>(r1cs.clone());

        let z0_secondary = [F::<G2>::from(0)];

        if let Ok(proof_result) = generate_recursive_snark_proof(
            witness_generator_file,
            r1cs,
            private_inputs,
            start_public_input,
            &pp,
        ) {
            let result = verify_recursive_snark_proof(
                &proof_result,
                &pp,
                iteration_count,
                start_public_input,
                z0_secondary,
            );

            assert!(result.is_ok());
            let (pk, vk) = CompressedSNARK::<_, _, _, _, S<G1>, S<G2>>::setup(&pp).unwrap();

            if let Ok(compressed_proof_result) = compress_snark_proof(proof_result, &pp, &pk) {
                let result = verify_compressed_snark_proof(
                    compressed_proof_result,
                    &vk,
                    iteration_count,
                    start_public_input,
                    z0_secondary,
                );

                assert!(result.is_ok());
            } else {
                panic!("Failed to generate the compressed proof!")
            }
        } else {
            panic!("Failed to generate the recursive proof!")
        }
    }
}
