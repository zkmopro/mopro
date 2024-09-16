use anyhow::{bail, Error};
use std::{collections::HashMap, panic};

use crate::GenerateProofResult;
use nova_scotia::*;

use nova_snark::{
    provider,
    traits::{circuit::TrivialTestCircuit, Group},
    CompressedSNARK, PublicParams, RecursiveSNARK, ProverKey, VerifierKey,
};
use pasta_curves;
use pasta_curves::{Fp, Fq};
use serde_json::json;
use std::env::current_dir;
use std::path::PathBuf;

#[macro_export]
macro_rules! nova_scotia_app {
    () => {

        // can be any cycle supported by Nova
        type G1 = provider::bn256_grumpkin::bn256::Point;
        type G2 = provider::bn256_grumpkin::grumpkin::Point;

        // Return value should be Result<mopro_ffi::GenerateProofResult, mopro_ffi::MoproError>
        
        fn generate_nova_scotia_proof(
            witness_generator_file: PathBuf,
            r1cs: circom::circuit::R1CS<F<G1>>,
            private_inputs: Vec<HashMap<String, serde_json::Value>>,
            start_public_input: [F<G1>; 2],
            pp: &PublicParams<G1, G2, C1<G1>, C2<G2>>,
        ) -> Result<RecursiveSNARK<G1, G2, C1<G1>, C2<G2>>, mopro_ffi::MoproError> {
            // recursively construct the input to circom witness generator
            let recursive_snark = create_recursive_circuit(
                FileLocation::PathBuf(witness_generator_file),
                r1cs,
                private_inputs,
                start_public_input.to_vec(),
                &pp,
            );
            recursive_snark.map_err(|e| mopro_ffi::MoproError::NovaScotiaError(format!("nova_scotia error: {}", e)))
        }

        // Return value should be Result<bool, mopro_ffi::MoproError>
        fn verify_nova_scotia_proof(
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

            // assert!(res.is_ok());
            res.map_err(|e| {
                mopro_ffi::MoproError::NovaScotiaError(format!("error verifying proof: {}", e))
            })
        }

        // Compress SNARK
        fn compress_snark_proof(
            recursive_snark: RecursiveSNARK<G1, G2, C1<G1>, C2<G2>>,
            pp: &PublicParams<G1, G2, C1<G1>, C2<G2>>,
            pk: &ProverKey<G1, G2, C1<G1>, C2<G2>, S<G1>, S<G2>>,
        ) -> Result<CompressedSNARK<G1, G2, C1<G1>, C2<G2>, S<G1>, S<G2>>, mopro_ffi::MoproError> {
            
            
            let res = CompressedSNARK::<_, _, _, _, S<G1>, S<G2>>::prove(&pp, &pk, &recursive_snark);

            // assert!(res.is_ok());
            res.map_err(|e| {
                mopro_ffi::MoproError::NovaScotiaError(format!("error compress proof: {}", e))
            })
        }

        fn verify_compressed_proof(
            compressed_snark: CompressedSNARK<G1, G2, C1<G1>, C2<G2>,S<G1>, S<G2>>,
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

            // assert!(res.is_ok());
            res.map_err(|e| {
                mopro_ffi::MoproError::NovaScotiaError(format!("error verifying proof: {}", e))
            })
        }
        
        
    };
}

#[cfg(test)]
mod test {
    use anyhow::{bail, Error};

    use std::fs::File;
    use std::str::FromStr;
    use std::{collections::HashMap, panic};

    use nova_scotia::*;

    use nova_snark::{
        provider,
        traits::{circuit::TrivialTestCircuit, Group},
        CompressedSNARK, PublicParams, RecursiveSNARK, ProverKey, VerifierKey,
    };

    use crate as mopro_ffi;
    use pasta_curves;
    use serde_json::json;
    use std::path::PathBuf;
    use pasta_curves::{Fp, Fq};

    nova_scotia_app!();

    const R1CS_PATH: &str = "../test-vectors/nova_scotia/toy.r1cs";
    const WASM_PATH: &str = "../test-vectors/nova_scotia/toy_js/toy.wasm";

    #[test]
    fn test_generate_and_verify_nova_scotia_proof() {
        let root = std::env::current_dir().unwrap();

        // load r1cs file
        let circuit_file = root.join(R1CS_PATH.to_string());
        let r1cs = circom::reader::load_r1cs::<G1, G2>(&FileLocation::PathBuf(circuit_file));
        //let r1cs = circom::reader::load_r1cs::<G1, G2>(&circuit_file);

        // load c++ binary or wasm file
        let witness_generator_file = root.join(WASM_PATH.to_string());



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

        // create public parameters(CRS)
        let pp = create_public_params::<G1, G2>(r1cs.clone());
        let z0_secondary = [F::<G2>::from(0)];
        
        if let Ok(proof_result) = generate_nova_scotia_proof(
            witness_generator_file,
            r1cs,
            private_inputs,
            start_public_input,
            &pp,
        ) {
            
            let result = verify_nova_scotia_proof(
                &proof_result,
                &pp,
                iteration_count,
                start_public_input,
                z0_secondary,
            );
            
            //assert!(result.is_ok());
            let (pk, vk) = CompressedSNARK::<_, _, _, _, S<G1>, S<G2>>::setup(&pp).unwrap();

            if let Ok(compressed_proof_result) = compress_snark_proof(
                proof_result,
                &pp,
                &pk,
            ) {
                
                let result = verify_compressed_proof(
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
            panic!("Failed to generate the proof!")
        }


        


        

        /* 
        let mut recursive_snark = create_recursive_circuit(
            FileLocation::PathBuf(witness_generator_file.clone()),
            r1cs.clone(),
            private_inputs,
            start_public_input.to_vec(),
            &pp,
        ).unwrap();

        println!("Verifying a RecursiveSNARK...");
        let res = recursive_snark.verify(&pp, iteration_count, &start_public_input, &z0_secondary);

        assert!(res.is_ok());
        let z_last = res.unwrap().0;

        println!("Generating a CompressedSNARK using Spartan with IPA-PC...");
        let (pk, vk) = CompressedSNARK::<_, _, _, _, S<G1>, S<G2>>::setup(&pp).unwrap();
        let res = CompressedSNARK::<_, _, _, _, S<G1>, S<G2>>::prove(&pp, &pk, &recursive_snark);
        assert!(res.is_ok());


        let compressed_snark = res.unwrap();

        // verify the compressed SNARK
        println!("Verifying a CompressedSNARK...");

        let res = compressed_snark.verify(
            &vk,
            iteration_count,
            start_public_input.to_vec(),
            z0_secondary.to_vec(),
        );
        assert!(res.is_ok());
        */

        
    }
}
