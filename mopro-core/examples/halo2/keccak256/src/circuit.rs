use std::marker::PhantomData;

use halo2_proofs::circuit::{Layouter, SimpleFloorPlanner};
use halo2_proofs::halo2curves::ff::PrimeField;
use halo2_proofs::plonk::{Circuit, Column, ConstraintSystem, Error, Instance};
use itertools::Itertools;
use sha3::{Digest, Keccak256};

use crate::util::eth_types::Field;
use crate::util::{value_to_option, SKIP_FIRST_PASS};
use crate::vanilla::keccak_packed_multi::{get_keccak_capacity, KeccakAssignedValue};
use crate::vanilla::param::{NUM_BYTES_PER_WORD, NUM_ROUNDS, NUM_WORDS_TO_ABSORB};
use crate::vanilla::witness::multi_keccak;
use crate::vanilla::{KeccakAssignedRow, KeccakCircuitConfig, KeccakConfigParams};

#[derive(Clone, Debug)]
pub struct CircuitConfig<F> {
    pub input: Column<Instance>, // TODO - make it possible to pass arbitrary amount, not 2.
    pub keccak_config: KeccakCircuitConfig<F>,
    _marker: PhantomData<F>,
}

/// KeccakCircuit
#[derive(Default, Clone, Debug)]
pub struct KeccakCircuit<F: Field> {
    config: KeccakConfigParams,
    inputs: Vec<Vec<u8>>,
    num_rows: Option<usize>,
    verify_output: bool,
    use_instance: bool,
    _marker: PhantomData<F>,
}

impl<F: Field> Circuit<F> for KeccakCircuit<F> {
    type Config = CircuitConfig<F>;
    type FloorPlanner = SimpleFloorPlanner;
    type Params = KeccakConfigParams;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn params(&self) -> Self::Params {
        self.config
    }

    fn configure_with_params(meta: &mut ConstraintSystem<F>, params: Self::Params) -> Self::Config {
        // MockProver complains if you only have columns in SecondPhase, so let's just make an empty column in FirstPhase
        meta.advice_column();

        let input = meta.instance_column();
        let keccak_config = KeccakCircuitConfig::new(meta, params);

        CircuitConfig {
            input,
            keccak_config,
            _marker: PhantomData,
        }
    }

    fn configure(_: &mut ConstraintSystem<F>) -> Self::Config {
        unreachable!()
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let params = config.keccak_config.parameters;
        config
            .keccak_config
            .load_aux_tables(&mut layouter, params.k)?;
        let mut first_pass = SKIP_FIRST_PASS;
        let mut cache = vec![];
        layouter.assign_region(
            || "keccak circuit",
            |mut region| {
                if first_pass {
                    first_pass = false;
                    return Ok(());
                }
                let (witness, _) = multi_keccak(
                    &self.inputs,
                    self.num_rows
                        .map(|nr| get_keccak_capacity(nr, params.rows_per_round)),
                    params,
                );
                let assigned_rows = config.keccak_config.assign(&mut region, &witness);
                cache.push(assigned_rows.clone());
                if self.verify_output {
                    self.verify_output_witnesses(&assigned_rows);
                    self.verify_input_witnesses(&assigned_rows);
                }

                Ok(())
            },
        )?;

        if self.use_instance {
            for assigned_row in cache.iter() {
                self.constraint_public_inputs(
                    layouter.namespace(|| "public inputs"),
                    assigned_row,
                    &config,
                );
            }
        }

        Ok(())
    }
}

impl<F: Field> KeccakCircuit<F> {
    /// Creates a new circuit instance
    pub fn new(
        config: KeccakConfigParams,
        num_rows: Option<usize>,
        inputs: Vec<Vec<u8>>,
        verify_output: bool,
        use_instance: bool,
    ) -> Self {
        KeccakCircuit {
            config,
            inputs,
            num_rows,
            _marker: PhantomData,
            verify_output,
            use_instance,
        }
    }

    fn verify_output_witnesses(&self, assigned_rows: &[KeccakAssignedRow<F>]) {
        let mut input_offset = 0;
        // only look at last row in each round
        // first round is dummy, so ignore
        // only look at last round per absorb of RATE_IN_BITS
        for assigned_row in assigned_rows
            .iter()
            .step_by(self.config.rows_per_round)
            .step_by(NUM_ROUNDS + 1)
            .skip(1)
        {
            let KeccakAssignedRow {
                is_final,
                hash_lo,
                hash_hi,
                ..
            } = assigned_row.clone();
            let is_final_val = extract_value(is_final).ne(&F::ZERO);
            let hash_lo_val = extract_u128(hash_lo);
            let hash_hi_val = extract_u128(hash_hi);

            if input_offset < self.inputs.len() && is_final_val {
                // out is in big endian.
                let out = Keccak256::digest(&self.inputs[input_offset]);
                let lo = u128::from_be_bytes(out[16..].try_into().unwrap());
                let hi = u128::from_be_bytes(out[..16].try_into().unwrap());
                assert_eq!(lo, hash_lo_val);
                assert_eq!(hi, hash_hi_val);
                input_offset += 1;
            }
        }
    }

    fn constraint_public_inputs(
        &self,
        mut layouter: impl Layouter<F>,
        assigned_rows: &[KeccakAssignedRow<F>],
        config: &<KeccakCircuit<F> as Circuit<F>>::Config,
    ) {
        let rows_per_round = self.config.rows_per_round;
        let mut input_offset = 0;
        let mut total_offset = 0;
        let mut input_byte_offset = 0;

        // first round is dummy, so ignore
        for absorb_chunk in &assigned_rows
            .chunks(rows_per_round)
            .skip(1)
            .chunks(NUM_ROUNDS + 1)
        {
            let mut absorbed = false;
            for (round_idx, assigned_rows) in absorb_chunk.enumerate() {
                for (row_idx, assigned_row) in assigned_rows.iter().enumerate() {
                    let KeccakAssignedRow {
                        is_final,
                        word_value,
                        ..
                    } = assigned_row.clone();
                    let is_final_val = extract_value(is_final).ne(&F::ZERO);

                    // If we reached to the end of this chunk, skip it
                    if input_offset >= self.inputs.len() {
                        continue;
                    }

                    let input_len = self.inputs[input_offset].len();

                    if input_byte_offset >= input_len {
                        continue;
                    }
                    if round_idx == NUM_ROUNDS && row_idx == 0 && is_final_val {
                        absorbed = true;
                    }
                    if row_idx == 0 {
                        // Only these rows could contain inputs.
                        let end = if round_idx < NUM_WORDS_TO_ABSORB {
                            std::cmp::min(input_byte_offset + NUM_BYTES_PER_WORD, input_len)
                        } else {
                            input_byte_offset
                        };

                        layouter
                            .constrain_instance(word_value.cell(), config.input, total_offset)
                            .unwrap();
                        total_offset += 1;

                        input_byte_offset = end;
                    }
                }
            }
            if absorbed {
                input_offset += 1;
                input_byte_offset = 0;
            }
        }
    }

    fn verify_input_witnesses(&self, assigned_rows: &[KeccakAssignedRow<F>]) {
        let rows_per_round = self.config.rows_per_round;
        let mut input_offset = 0;
        let mut input_byte_offset = 0;
        // first round is dummy, so ignore
        for absorb_chunk in &assigned_rows
            .chunks(rows_per_round)
            .skip(1)
            .chunks(NUM_ROUNDS + 1)
        {
            let mut absorbed = false;
            for (round_idx, assigned_rows) in absorb_chunk.enumerate() {
                for (row_idx, assigned_row) in assigned_rows.iter().enumerate() {
                    let KeccakAssignedRow {
                        is_final,
                        word_value,
                        bytes_left,
                        ..
                    } = assigned_row.clone();
                    let is_final_val = extract_value(is_final).ne(&F::ZERO);
                    let word_value_val = extract_u128(word_value);
                    let bytes_left_val = extract_u128(bytes_left);
                    // Padded inputs - all empty.
                    if input_offset >= self.inputs.len() {
                        assert_eq!(word_value_val, 0);
                        assert_eq!(bytes_left_val, 0);
                        continue;
                    }
                    let input_len = self.inputs[input_offset].len();
                    if round_idx == NUM_ROUNDS && row_idx == 0 && is_final_val {
                        absorbed = true;
                    }
                    if row_idx == 0 {
                        assert_eq!(
                            bytes_left_val,
                            input_len as u128 - input_byte_offset as u128
                        );
                        // Only these rows could contain inputs.
                        let end = if round_idx < NUM_WORDS_TO_ABSORB {
                            std::cmp::min(input_byte_offset + NUM_BYTES_PER_WORD, input_len)
                        } else {
                            input_byte_offset
                        };
                        let mut expected_val_le_bytes = self.inputs[input_offset]
                            [input_byte_offset..end]
                            .to_vec()
                            .clone();
                        expected_val_le_bytes.resize(NUM_BYTES_PER_WORD, 0);
                        assert_eq!(
                            word_value_val,
                            u64::from_le_bytes(expected_val_le_bytes.try_into().unwrap()) as u128,
                        );
                        input_byte_offset = end;
                    }
                }
            }
            if absorbed {
                input_offset += 1;
                input_byte_offset = 0;
            }
        }
    }
}

fn extract_value<F: Field>(assigned_value: KeccakAssignedValue<F>) -> F {
    let assigned = *value_to_option(assigned_value.value()).unwrap();
    match assigned {
        halo2_proofs::plonk::Assigned::Zero => F::ZERO,
        halo2_proofs::plonk::Assigned::Trivial(f) => f,
        _ => panic!("value should be trival"),
    }
}

fn extract_u128<F: Field>(assigned_value: KeccakAssignedValue<F>) -> u128 {
    let le_bytes = extract_value(assigned_value).to_bytes_le();
    let hi = u128::from_le_bytes(le_bytes[16..].try_into().unwrap());
    assert_eq!(hi, 0);
    u128::from_le_bytes(le_bytes[..16].try_into().unwrap())
}

/// Packs each input byte array into field elements for use in cryptographic computations,
/// specifically mimicking the packing technique utilized in the keccak circuit.
/// Each high-level vector's bytes are combined into a single field element up to `NUM_BYTES_PER_WORD`.
/// Bytes arrays shorter than `NUM_BYTES_PER_WORD` are zero-padded to this length.
/// The field element is derived from these bytes interpreted as a little-endian u64.
pub fn pack_input_to_instance<F: PrimeField>(input: &[Vec<u8>]) -> Vec<F> {
    input
        .iter()
        .flat_map(|input_vec| {
            input_vec.chunks(NUM_BYTES_PER_WORD).map(|chunk| {
                let mut buf = [0u8; NUM_BYTES_PER_WORD]; // Create a buffer initialized to zero
                buf[..chunk.len()].copy_from_slice(chunk); // Copy bytes from the chunk
                let val = u64::from_le_bytes(buf); // Convert little-endian bytes to u64
                F::from(val) // Convert u64 to field element
            })
        })
        .collect()
}

/// Converts field elements to a vector of bytes.
/// Currently converts each field element to a single byte.
/// TODO - optimize by packing multiple bytes into field elements
pub fn unpack_input<F: Field>(instance: &[F]) -> Vec<u8> {
    instance
        .iter()
        .map(|x| x.to_bytes_le()[0])
        .collect::<Vec<u8>>()
}

#[cfg(test)]
mod test {
    use crate::circuit::{pack_input_to_instance, unpack_input};
    use halo2_proofs::halo2curves::bn256::Fr;
    use test_case::test_case;

    #[test_case(vec![0u8, 151u8, 200u8, 255u8] ; "4 Different Elements")]
    #[test_case(vec![] ; "Empty case")]
    fn test_unpack_input(input: Vec<u8>) {
        // Convert the input to field elements
        let f_input = input
            .iter()
            .map(|x| Fr::from(*x as u64))
            .collect::<Vec<Fr>>();

        // Convert the field elements back to bytes
        let output = unpack_input(&f_input);
        assert_eq!(input, output);
    }

    #[test_case(vec![0u8, 0u8, 0u8, 0u8], vec![Fr::from(0u64)] ; "Zero to Zero")]
    #[test_case(vec![1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8], vec![Fr::from(4294967297u64)] ; "Max size single element")]
    #[test_case(vec![1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 10u8], vec![Fr::from(4294967297u64), Fr::from(10u64)] ; "Two sized output")]
    fn test_pack_input_to_instance(input: Vec<u8>, expected: Vec<Fr>) {
        // Convert the input to field elements
        let f_input = pack_input_to_instance::<Fr>(&vec![input]);

        // 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8 in little endian is

        // Check that the field elements match the expected values
        assert_eq!(f_input, expected);
    }
}
