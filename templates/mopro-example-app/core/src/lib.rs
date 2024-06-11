// NOTE: The following is a basic end to end example

#[cfg(test)]
mod tests {
    use ark_bn254::Fr;
    use mopro_core::middleware::circom::serialization::SerializableInputs;
    use mopro_core::middleware::circom::CircomState;
    use num_bigint::BigInt;
    use std::collections::HashMap;

    #[test]
    fn test_prove_verify_simple() {
        let wasm_path = "./circuits/multiplier2/target/multiplier2_js/multiplier2.wasm";
        let zkey_path = "./circuits/multiplier2/target/multiplier2_final.zkey";

        // Instantiate CircomState
        let mut circom_state = CircomState::new();

        // Initialize
        let init_res = circom_state.initialize(zkey_path, wasm_path);
        assert!(init_res.is_ok());

        let _serialized_pk = init_res.unwrap();

        // Prepare inputs
        let mut inputs = HashMap::new();
        let a = 3;
        let b = 5;
        let c = a * b;
        inputs.insert("a".to_string(), vec![BigInt::from(a)]);
        inputs.insert("b".to_string(), vec![BigInt::from(b)]);
        // output = [public output c, public input a]
        let expected_output = vec![Fr::from(c), Fr::from(a)];
        let serialized_outputs = SerializableInputs(expected_output);

        // Proof generation
        let generate_proof_res = circom_state.generate_proof(inputs);

        // Check and print the error if there is one
        if let Err(e) = &generate_proof_res {
            println!("Error: {:?}", e);
        }

        assert!(generate_proof_res.is_ok());

        let (serialized_proof, serialized_inputs) = generate_proof_res.unwrap();

        // Check output
        assert_eq!(serialized_inputs, serialized_outputs);

        // Proof verification
        let verify_res = circom_state.verify_proof(serialized_proof, serialized_inputs);
        assert!(verify_res.is_ok());
        assert!(verify_res.unwrap()); // Verifying that the proof was indeed verified
    }
}
