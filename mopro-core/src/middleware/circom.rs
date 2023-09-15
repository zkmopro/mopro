use ark_bn254::Bn254;
use ark_circom::{CircomBuilder, CircomConfig};
use ark_crypto_primitives::snark::SNARK;
use ark_groth16::Groth16;
use ark_std::rand::thread_rng;
use color_eyre::Result;

type GrothBn = Groth16<Bn254>;

// TODO: Refactor this to be a proper API with setup, prove, verify etc
// This is just a temporary function to get things working end-to-end.
// Later we call as native Rust in example, and use from mopro-ffi
pub fn run_example() -> Result<()> {
    println!("Setup");

    // Load the WASM and R1CS for witness and proof generation
    let cfg = CircomConfig::<Bn254>::new(
        "./examples/circom/target/multiplier2_js/multiplier2.wasm",
        "./examples/circom/target/multiplier2.r1cs",
    )?;

    // Insert our inputs as key value pairs
    // In Circom this is the private input (witness) and public input
    // It does not include the public output
    let mut builder = CircomBuilder::new(cfg);
    builder.push_input("a", 3);
    builder.push_input("b", 5);

    // Create an empty instance for setting it up
    let circom = builder.setup();

    // Run a trusted setup
    let mut rng = thread_rng();
    let params = GrothBn::generate_random_parameters_with_reduction(circom, &mut rng)?;

    // Get the populated instance of the circuit with the witness
    let circom = builder.build()?;

    // This is the instance, public input and public output
    // Together with the witness provided above this satisfies the circuit
    let inputs = circom.get_public_inputs().unwrap();

    // Generate the proof
    println!("Generating proof");
    let proof = GrothBn::prove(&params, circom, &mut rng)?;

    // Check that the proof is valid
    println!("Verifying proof");
    let pvk = GrothBn::process_vk(&params.vk)?;
    let proof_verified = GrothBn::verify_with_processed_vk(&pvk, &inputs, &proof)?;
    match proof_verified {
        true => println!("Proof verified"),
        false => println!("Proof not verified"),
    }
    assert!(proof_verified);

    // To provide the instance manually, i.e. public input and output in Circom:
    let c = 15;
    let a = 3;
    let inputs_manual = vec![c.into(), a.into()];

    let verified_alt = GrothBn::verify_with_processed_vk(&pvk, &inputs_manual, &proof)?;
    //println!("Proof verified (alt): {}", verified_alt);
    assert!(verified_alt);

    Ok(())
}
