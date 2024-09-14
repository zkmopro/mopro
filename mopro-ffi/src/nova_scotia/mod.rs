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
use serde_json::json;
use pasta_curves;
use std::env::{current_dir};

#[macro_export]
macro_rules! nova_scotia_app {
    () => {
        // can be any cycle supported by Nova
        type G1 = pasta_curves::pallas::Point;
        type G2 = pasta_curves::vesta::Point;

        // Return value should be Result<mopro_ffi::GenerateProofResult, mopro_ffi::MoproError>
        fn generate_nova_scotia_proof(
            witness_generator_file:PathBuf,
            r1cs:R1CS<Fq>,
            private_inputs: Vec<HashMap<String, Value>>,
            start_public_input: Vec<F<G1>>,
            pp:PublicParams<G1, G2, _, _>,
        ) -> Result<RecursiveSNARK<E1, E2, C1, C2>, mopro_ffi::MoproError> {
            

            // recursively construct the input to circom witness generator
            let recursive_snark = create_recursive_circuit(
                FileLocation::PathBuf(witness_generator_file),
                r1cs,
                private_inputs,
                start_public_input.to_vec(),
                &pp,
            ).unwrap();
            recursive_snark
        }

        // Return value should be Result<bool, mopro_ffi::MoproError>
        fn verify_nova_scotia_proof(
            recursive_snark: RecursiveSNARK<G1, G2, C1<G1>, C2<G2>>,
            pp: PublicParams<G1, G2, C1<G1>, C2<G2>>,
            iteration_count: usize,
            start_public_input: &[E1::Scalar],
            z0_secondary:[Fp;1],
        ) -> Result</*(Vec<E1::Scalar>, Vec<E2::Scalar>)*/bool, mopro_ffi::MoproError> {
            let res=recursive_snark.verify(
                &pp,
                iteration_count,
                &start_public_input.clone(),
                &z0_secondary,
            );


            // assert!(res.is_ok());

            res
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
    use crate as mopro_ffi;
    use serde_json::json;

    nova_scotia_app!();

    
    const R1CS_PATH: &str = "../test-vectors/nova_scotia/";
    const WASM_PATH: &str = "../test-vectors/nova_scotia/";
    //const PRIVATE_INPUTS: Vec<HashMap<String, Value>> = vec![];
    //const START_PUBLIC_INPUT: Vec<F<G1>> = vec![];


    #[test]
    fn test_generate_and_verify_nova_scotia_proof() {


        let root = current_dir().unwrap();

        // load r1cs file
        let circuit_file = root.join(R1CS_PATH.to_string());
        let r1cs = load_r1cs::<G1, G2>(&circuit_file);

        // load c++ binary or wasm file
        let witness_generator_file = root.join(WASM_PATH.to_string());

        // create public parameters(CRS)
        let pp = create_public_params::<G1, G2>(r1cs.clone());

        //private inputs
        let iteration_count = 5;
        let mut private_inputs = Vec::new();
        for i in 0..iteration_count {
            let mut private_input = HashMap::new();
            private_input.insert("adder".to_string(), json!(i));
            private_inputs.push(private_input);
        }

        //start public input
        let start_public_input = [F::<G1>::from(10), F::<G1>::from(10)];

        //
        let pp: PublicParams<G1, G2, _, _> = create_public_params(r1cs.clone());

        //create a recursive SNARK
        let start = Instant::now();


        if let Ok(proof_result) = generate_nova_scotia_proof(
            witness_generator_file,
            r1cs,
            private_inputs,
            start_public_input,
            &pp,
        ) {
            let z0_secondary = [F::<G2>::from(0)];

            let result = verify_nova_scotia_proof(
                proof_result,
                &pp,
                iteration_count,
                &start_public_input,
                &z0_secondary,
            );
            assert!(result.is_ok());
        } else {
            panic!("Failed to generate the proof!")
        }

    }
    

}