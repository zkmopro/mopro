use std::marker::PhantomData;

use halo2_proofs::circuit::{Layouter, SimpleFloorPlanner};
use halo2_proofs::halo2curves::ff::PrimeField;
use halo2_proofs::plonk::{Circuit, Column, ConstraintSystem, Error, Instance};
use itertools::Itertools;
use sha3::{Digest, Keccak256};

use crate::util::{SKIP_FIRST_PASS, value_to_option};
use crate::util::eth_types::Field;
use crate::vanilla::{KeccakAssignedRow, KeccakCircuitConfig, KeccakConfigParams};
use crate::vanilla::keccak_packed_multi::{get_keccak_capacity, KeccakAssignedValue};
use crate::vanilla::param::{NUM_BYTES_PER_WORD, NUM_ROUNDS, NUM_WORDS_TO_ABSORB};
use crate::vanilla::witness::multi_keccak;

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

    fn params(&self) -> Self::Params {
        self.config
    }

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure_with_params(meta: &mut ConstraintSystem<F>, params: Self::Params) -> Self::Config {
        // MockProver complains if you only have columns in SecondPhase, so let's just make an empty column in FirstPhase
        meta.advice_column();
        
        let input = meta.instance_column();
        let keccak_config = KeccakCircuitConfig::new(meta, params);
        
        CircuitConfig {  input, keccak_config, _marker: PhantomData }
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
        config.keccak_config.load_aux_tables(&mut layouter, params.k)?;
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
                    self.num_rows.map(|nr| get_keccak_capacity(nr, params.rows_per_round)),
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
                self.constraint_public_inputs(layouter.namespace(|| "public inputs"), assigned_row, &config);
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
        KeccakCircuit { config, inputs, num_rows, _marker: PhantomData, verify_output, use_instance }
    }

    fn verify_output_witnesses(&self, assigned_rows: &[KeccakAssignedRow<F>]) {
        let mut input_offset = 0;
        // only look at last row in each round
        // first round is dummy, so ignore
        // only look at last round per absorb of RATE_IN_BITS
        for assigned_row in
            assigned_rows.iter().step_by(self.config.rows_per_round).step_by(NUM_ROUNDS + 1).skip(1)
        {
            let KeccakAssignedRow { is_final, hash_lo, hash_hi, .. } = assigned_row.clone();
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
    
    
    fn constraint_public_inputs(&self, mut layouter: impl Layouter<F>, assigned_rows: &[KeccakAssignedRow<F>], config: &<KeccakCircuit<F> as Circuit<F>>::Config) {
        let rows_per_round = self.config.rows_per_round;
        let mut input_offset = 0;
        let mut total_offset = 0;
        let mut input_byte_offset = 0;
        
        let instance = pack_input_to_instance::<F>(&self.inputs);

        // first round is dummy, so ignore
        for absorb_chunk in &assigned_rows.chunks(rows_per_round).skip(1).chunks(NUM_ROUNDS + 1) {
            let mut absorbed = false;
            for (round_idx, assigned_rows) in absorb_chunk.enumerate() {
                for (row_idx, assigned_row) in assigned_rows.iter().enumerate() {
                    let KeccakAssignedRow { is_final, word_value, .. } =
                        assigned_row.clone();
                    let is_final_val = extract_value(is_final).ne(&F::ZERO);
                    let word_value_val = extract_u128(word_value.clone());
                    // let bytes_left_val = extract_u128(bytes_left);
                    // Padded inputs - all empty.
                    // TODO - consider if it should be checked
                    // if input_offset >= self.inputs.len() {
                    //     assert_eq!(word_value_val, 0);
                    //     assert_eq!(bytes_left_val, 0);
                    //     continue;
                    // }
                    
                    // If we reached to the end of this chunk, skip it
                    if input_offset >= self.inputs.len() {
                        continue;
                    }
                    
                    let input_len = self.inputs[input_offset].len();
                    if round_idx == NUM_ROUNDS && row_idx == 0 && is_final_val {
                        absorbed = true;
                    }
                    if row_idx == 0 {
                        // TODO - consider if it should be checked
                        // assert_eq!(bytes_left_val, input_len as u128 - input_byte_offset as u128);
                        
                        // Only these rows could contain inputs.
                        let end = if round_idx < NUM_WORDS_TO_ABSORB {
                            std::cmp::min(input_byte_offset + NUM_BYTES_PER_WORD, input_len)
                        } else {
                            input_byte_offset
                        };
                        
                        // let mut expected_val_le_bytes =
                        //     self.inputs[input_offset][input_byte_offset..end].to_vec().clone();
                        // expected_val_le_bytes.resize(NUM_BYTES_PER_WORD, 0);
                        // assert_eq!(
                        //     word_value_val,
                        //     u64::from_le_bytes(expected_val_le_bytes.try_into().unwrap()) as u128,
                        // );
                        
                        // Check if the packed value is equal to the expected value
                        if F::from_u128(word_value_val) != instance[total_offset] {
                            dbg!(format!("Input offset: {:?}", input_offset));
                            dbg!(format!("Input byte offset: {:?}", input_byte_offset));
                            dbg!(format!("Expected value: {:?}", &self.inputs[input_offset][input_byte_offset..end].to_vec().clone()));
                            dbg!(format!("Word value: {:?}", word_value.clone()));
                            dbg!(format!("Total offset: {:?}", total_offset));
                            dbg!(format!("Packed value: {:?}", instance[total_offset]));
                        }
                        
                        layouter.constrain_instance(word_value.cell(), config.input, total_offset).unwrap();
                        
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
        for absorb_chunk in &assigned_rows.chunks(rows_per_round).skip(1).chunks(NUM_ROUNDS + 1) {
            let mut absorbed = false;
            for (round_idx, assigned_rows) in absorb_chunk.enumerate() {
                for (row_idx, assigned_row) in assigned_rows.iter().enumerate() {
                    let KeccakAssignedRow { is_final, word_value, bytes_left, .. } =
                        assigned_row.clone();
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
                        assert_eq!(bytes_left_val, input_len as u128 - input_byte_offset as u128);
                        // Only these rows could contain inputs.
                        let end = if round_idx < NUM_WORDS_TO_ABSORB {
                            std::cmp::min(input_byte_offset + NUM_BYTES_PER_WORD, input_len)
                        } else {
                            input_byte_offset
                        };
                        let mut expected_val_le_bytes =
                            self.inputs[input_offset][input_byte_offset..end].to_vec().clone();
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

/// This function follows the packing technique done by the keccak circuit
/// By for each high level vector, combining NUM_BYTES_PER_WORD bytes into a single field element
/// If the vectors ends short of that, the result is resized to NUM_BYTES_PER_WORD with 0s
/// The result is then converted to a u64 from little endian bytes
pub fn pack_input_to_instance<F: PrimeField>(input: &Vec<Vec<u8>>) -> Vec<F> {
    input
        .iter()
        .map(|input| {
            let mut packed = vec![];
            for chunk in input.chunks(NUM_BYTES_PER_WORD) {
                let mut chunk = chunk.to_vec();
                chunk.resize(NUM_BYTES_PER_WORD, 0);
                let val = F::from(u64::from_le_bytes(chunk.try_into().unwrap()));
                packed.push(val);
            }
            packed
        })
        .flatten()
        .collect()
    }