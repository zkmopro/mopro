use anyhow::{bail, Error};

use std::fs::File;
use std::str::FromStr;
use std::{collections::HashMap, panic};

use crate::GenerateProofResult;
use nova_scotia::{
    circom::{circuit::CircomCircuit, reader::load_r1cs},
    create_public_params, create_recursive_circuit, FileLocation, F, S,
};

use nova_snark::{
    provider,
    traits::{circuit::TrivialTestCircuit, Group},
    CompressedSNARK, PublicParams,
};

use pasta_curves;

#[macro_export]
macro_rules! nova_scotia_app {
    () => {
        // can be any cycle supported by Nova
        type G1 = pasta_curves::pallas::Point;
        type G2 = pasta_curves::vesta::Point;

        // Return value should be Result<mopro_ffi::GenerateProofResult, mopro_ffi::MoproError>
        fn generate_nova_scotia_proof(
            r1cs_path: String,
            cpp_bin_or_wasm_path: String,
            private_inputs: Vec<HashMap<String, Value>>,
            start_public_input: Vec<F<G1>>,
        ) -> Result<RecursiveSNARK<E1, E2, C1, C2>, mopro_ffi::MoproError> {
            let root = current_dir().unwrap();

            // load r1cs file
            let circuit_file = root.join(r1cs_path);
            let r1cs = load_r1cs::<G1, G2>(&circuit_file);

            // load c++ binary or wasm file
            let witness_generator_file = root.join(cpp_bin_or_wasm_path);

            // create public parameters(CRS)
            let pp = create_public_params::<G1, G2>(r1cs.clone());

            // recursively construct the input to circom witness generator
            let recursive_snark = create_recursive_circuit(
                FileLocation::PathBuf(witness_generator_file),
                r1cs,
                private_inputs,
                start_public_input.to_vec(),
                &pp,
            ).unwrap()
                .map(|(proof, inputs)| mopro_ffi::GenerateProofResult { proof, inputs })
                .map_err(|e| mopro_ffi::MoproError::NovaScotiaError(format!("Recursive Snark Error: {}", e)))
        }

        // Return value should be Result<bool, mopro_ffi::MoproError>
        fn verify_nova_scotia_proof(
            recursive_snark: RecursiveSNARK<G1, G2, C1<G1>, C2<G2>>,
            pp: &PublicParams<G1, G2, C1<G1>, C2<G2>>,
            iteration_count: usize,
            start_public_input: &[E1::Scalar],
        ) -> Result</*(Vec<E1::Scalar>, Vec<E2::Scalar>)*/bool, mopro_ffi::MoproError> {
            recursive_snark.verify(
                &pp,
                iteration_count,
                &start_public_input.clone(),
                &[F<G2>::zero()],
            ).is_ok()
                .map_err(|e| mopro_ffi::MoproError::NovaScotiaError(format!("Recursive Snark Verification Error: {}", e)))

            // assert!(res.is_ok());

            // res
        }
    };
}

#[cfg(test)]
mod test {
    use anyhow::{bail, Error};

    use std::fs::File;
    use std::str::FromStr;
    use std::{collections::HashMap, panic};

    use nova_scotia::{
        circom::{circuit::CircomCircuit, reader::load_r1cs},
        create_public_params, create_recursive_circuit, FileLocation, F, S,
    };

    use nova_snark::{
        provider,
        traits::{circuit::TrivialTestCircuit, Group},
        CompressedSNARK, PublicParams,
    };

    use pasta_curves;
    type G1 = pasta_curves::pallas::Point;
    type G2 = pasta_curves::vesta::Point;

    use crate as mopro_ffi;

    fn test_nova_scotia_macro() {
        nova_scotia_app!();

        const R1CS_PATH: &str = "../test-vectors/nova_scotia/";
        const WASN_PATH: &str = "../test-vectors/nova_scotia/";
        const PRIVATE_INPUTS: Vec<HashMap<String, Value>> = vec![];
        const START_PUBLIC_INPUT: Vec<F<G1>> = vec![];

        let result = generate_nova_scotia_proof(R1CS_PATH.to_string(), WASN_PATH.to_string(), PRIVATE_INPUTS, START_PUBLIC_INPUT);

        assert!(result.is_ok());
    }
}