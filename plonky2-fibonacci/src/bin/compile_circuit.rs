use std::io::Error;

use plonky2::plonk::{
    circuit_builder::CircuitBuilder,
    circuit_data::CircuitConfig,
    config::{GenericConfig, PoseidonGoldilocksConfig},
};
use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::circuit_data::CircuitData,
    util::serialization::{DefaultGateSerializer, DefaultGeneratorSerializer},
};

fn main() -> Result<(), Error> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // The arithmetic circuit.
    let initial_a = builder.add_virtual_target();
    let initial_b = builder.add_virtual_target();
    let mut prev_target = initial_a;
    let mut cur_target = initial_b;
    for _ in 0..99 {
        let temp = builder.add(prev_target, cur_target);
        prev_target = cur_target;
        cur_target = temp;
    }

    // Public inputs are the two initial values (provided below) and the result (which is generated).
    builder.register_public_input(initial_a);
    builder.register_public_input(initial_b);
    builder.register_public_input(cur_target);

    let circuit_data = builder.build::<C>();
    let gate_serializer = DefaultGateSerializer;
    let generator_serializer = DefaultGeneratorSerializer::<C, D>::default();

    let circuit_bytes = circuit_data
        .to_bytes(&gate_serializer, &generator_serializer)
        .unwrap();

    let prover_data = circuit_data.prover_data();
    let pk_bytes = prover_data
        .to_bytes(&gate_serializer, &generator_serializer)
        .unwrap();

    let circuit_data: CircuitData<GoldilocksField, C, 2> =
        CircuitData::from_bytes(&circuit_bytes, &gate_serializer, &generator_serializer).unwrap();
    let verifier_data = circuit_data.verifier_data();
    let vk_bytes = verifier_data.to_bytes(&gate_serializer).unwrap();

    std::fs::write("plonky2_fibonacci_pk.bin", pk_bytes)?;
    std::fs::write("plonky2_fibonacci_vk.bin", vk_bytes)?;

    Ok(())
}
