#[cfg(all(test))]
mod tests {
    use crate::middleware::gpu_explorations::metal::abstraction::state::MetalState;
    use ark_bn254::{Fq, Fr as ScalarField, G1Affine as GAffine, G1Projective as G};
    use ark_ec::AffineRepr;
    use ark_ff::{Field, PrimeField, biginteger::BigInteger256, BigInt};

    use metal::MTLSize;
    use proptest::prelude::*;

    pub type Point = GAffine;
    pub type Scalar = <ScalarField as PrimeField>::BigInt;

    // pub type F = BN254PrimeField;
    // pub type FE = FieldElement<F>;
    // pub type U = U256; // F::BaseType

    mod unsigned_int_tests {
        use super::*;

        enum BigOrSmallInt {
            Big(BigInteger256),
            Small(usize),
        }

        // implement to_u32_limbs and from_u32_limbs for BigInt<4>
        trait ToLimbs {
            fn to_u32_limbs(&self) -> Vec<u32>;
        }

        trait FromLimbs {
            fn from_u32_limbs(limbs: &[u32]) -> Self;
            fn from_u128(num: u128) -> Self;
            fn from_u32(num: u32) -> Self;
        }

        // convert from little endian to big endian
        impl ToLimbs for BigInteger256 {
            fn to_u32_limbs(&self) -> Vec<u32> {
                let mut limbs = Vec::new();                
                self.to_bytes_be().chunks(8).for_each(|chunk| {
                    let high = u32::from_be_bytes(chunk[0..4].try_into().unwrap());
                    let low = u32::from_be_bytes(chunk[4..8].try_into().unwrap());
                    limbs.push(high);
                    limbs.push(low);
                });
                limbs
            }
        }

        impl FromLimbs for BigInteger256 {
            // convert from big endian to little endian for metal
            fn from_u32_limbs(limbs: &[u32]) -> Self {
                let mut big_int = [0u64; 4];
                for (i, limb) in limbs.chunks(2).rev().enumerate() {
                    let high = u64::from(limb[0]);
                    let low = u64::from(limb[1]);
                    big_int[i] = (high << 32) | low;
                }
                BigInt(big_int)
            }
            // provide little endian u128 since arkworks use this value as well
            fn from_u128(num: u128) -> Self {
                let high = (num >> 64) as u64;
                let low = num as u64;
                BigInt([low, high, 0, 0])
            }
            // provide little endian u32 since arkworks use this value as well
            fn from_u32(num: u32) -> Self {
                BigInt([num as u64, 0, 0, 0])
            }
        }

        fn execute_kernel(name: &str, params: (BigInteger256, BigOrSmallInt)) -> BigInteger256 {
            let state = MetalState::new(None).unwrap();
            let pipeline = state.setup_pipeline(name).unwrap();

            let (a, b) = params;

            let a = a.to_u32_limbs();
            
            let result_buffer = state.alloc_buffer::<BigInteger256>(1);
            
            let debug_buffer = state.alloc_buffer::<u32>(24);
            
            let (command_buffer, command_encoder) = match b {
                BigOrSmallInt::Big(b) => {
                    let b = b.to_u32_limbs();
                    let a_buffer = state.alloc_buffer_data(&a);
                    let b_buffer = state.alloc_buffer_data(&b);
                    state.setup_command(
                        &pipeline,
                        Some(&[(0, &a_buffer), (1, &b_buffer), (2, &result_buffer), (3, &debug_buffer)]),
                    )
                }
                BigOrSmallInt::Small(b) => {
                    let a_buffer = state.alloc_buffer_data(&a);
                    let b_buffer = state.alloc_buffer_data(&[b]);
                    state.setup_command(
                        &pipeline,
                        Some(&[(0, &a_buffer), (1, &b_buffer), (2, &result_buffer), (3, &debug_buffer)]),
                    )
                }
            };

            let threadgroup_size = MTLSize::new(1, 1, 1);
            let threadgroup_count = MTLSize::new(1, 1, 1);

            command_encoder.dispatch_thread_groups(threadgroup_count, threadgroup_size);
            command_encoder.end_encoding();

            command_buffer.commit();
            command_buffer.wait_until_completed();

            let limbs = MetalState::retrieve_contents::<u32>(&result_buffer);

            // // Read the debug information
            // let debug_data = MetalState::retrieve_contents::<u32>(&debug_buffer);

            // // Print the values of a and b
            // println!("Value of a:");
            // for i in 0..8 {
            //     println!("Limb {}: 0x{:08X}", i, debug_data[i]);
            // }

            // println!("Value of b:");
            // for i in 0..8 {
            //     println!("Limb {}: 0x{:08X}", i, debug_data[i + 8]);
            // }
            // println!("Value of result:");
            // for i in 0..8 {
            //     println!("Limb {}: 0x{:08X}", i, debug_data[i + 16]);
            // }
            // println!(">>> limbs: {:?}", limbs);
            BigInteger256::from_u32_limbs(&limbs)
        }

        prop_compose! {
            fn rand_u128()(n in any::<u128>()) -> BigInteger256 { BigInteger256::from_u128(n) }
        }
        prop_compose! {
            fn rand_u32()(n in any::<u32>()) -> BigInteger256 { BigInteger256::from_u32(n) }
        }

        use ark_ff::biginteger::{BigInteger,BigInteger256};
        use num_bigint::BigUint;
        // use lambdaworks_math::unsigned_integer::traits::U32Limbs;
        use BigOrSmallInt::{Big, Small};

        proptest! {
            #[test]
            fn add(a in rand_u128(), b in rand_u128()) {
                let mut result = BigInteger256::default();
                objc::rc::autoreleasepool(|| {
                    result = execute_kernel("test_uint_add", (a, Big(b)));
                });
                let mut local_add = a;
                local_add.add_with_carry(&b);
                prop_assert_eq!(result, local_add);
            }

            #[test]
            fn sub(a in rand_u128(), b in rand_u128()) {
                let mut result = BigInteger256::default();
                let (a, b) = if a < b { (b, a) } else { (a, b) };
                objc::rc::autoreleasepool(|| {
                    result = execute_kernel("test_uint_sub", (a, Big(b)));
                });
                let mut local_sub = a;
                local_sub.sub_with_borrow(&b);
                prop_assert_eq!(result, local_sub);
            }

            #[test]
            fn prod(a in rand_u128(), b in rand_u32()) {
                let mut result = BigInteger256::default();
                objc::rc::autoreleasepool(|| {
                    result = execute_kernel("test_uint_prod", (a, Big(b)));
                });
                let local_prod = BigUint::from(a) * BigUint::from(b);
                let mut base_bigint: [u64; 4] = [0; 4];
                for (i, limb) in local_prod.to_u64_digits().iter().enumerate() {
                    base_bigint[i] = *limb;
                }
                let local_prod: BigInt<4> = BigInt(base_bigint);
                prop_assert_eq!(result, local_prod);
            }

            #[test]
            fn shl(a in rand_u128(), b in any::<usize>()) {
                let mut result = BigInteger256::default();
                let b = b % 256; // so it doesn't overflow
                objc::rc::autoreleasepool(|| {
                    result = execute_kernel("test_uint_shl", (a, Small(b)));
                });
                let mut local_shl = a;
                local_shl.muln(b as u32);
                prop_assert_eq!(result, local_shl);
            }

            #[test]
            fn shr(a in rand_u128(), b in any::<usize>()) {
                let mut result = BigInteger256::default();
                let b = b % 256; // so it doesn't overflow
                objc::rc::autoreleasepool(|| {
                    result = execute_kernel("test_uint_shr", (a, Small(b)));
                });
                let mut local_shr = a;
                local_shr.divn(b as u32);
                prop_assert_eq!(result, local_shr);
            }
        }
    }

    
    mod fp_ez_tests {
        use super::*;
        fn execute_kernel(name: &str, out: &mut [u32]) {
            let state = MetalState::new(None).unwrap();
            let pipeline = state.setup_pipeline(name).unwrap();
        
            let result_buffer = state.alloc_buffer_data(out);
        
            let (command_buffer, command_encoder) = state.setup_command(&pipeline, Some(&[(0, &result_buffer)]));
        
            let threadgroup_size = MTLSize::new(1, 1, 1);
            let threadgroup_count = MTLSize::new(1, 1, 1);
        
            command_encoder.dispatch_thread_groups(threadgroup_count, threadgroup_size);
            command_encoder.end_encoding();
        
            command_buffer.commit();
            command_buffer.wait_until_completed();
        

            let limbs = MetalState::retrieve_contents::<u32>(&result_buffer);
            // put limbs into out
            for (i, limb) in limbs.iter().enumerate() {
                out[i] = *limb;
            }
            print!("{:?}", out);
        }

        /*
            8   test_bn254_add 
            4   test_bn254_sub
            55  test_bn254_mul
            1   test_bn254_inversion
            0   test_bn254_neg
            3   test_bn254_mont_reduction
            125 test_bn254_exp
            1   test_bn254_eq
            1   test_bn254_ineq
        */
        #[test]
        fn add() {
            let mut result = [0u32; 1];
            execute_kernel("test_bn254_add", &mut result);
            // assert_eq!(result[0], 8)
        }

        #[test]
        fn sub() {
            let mut result = [0u32; 1];
            execute_kernel("test_bn254_sub", &mut result);
            // assert_eq!(result[0], 4)
        }

        #[test]
        fn mul() {
            let mut result = [0u32; 1];
            execute_kernel("test_bn254_mul", &mut result);
            // assert_eq!(result[0], 55)
        }

        #[test]
        fn inversion() {
            let mut result = [0u32; 1];
            execute_kernel("test_bn254_inversion", &mut result);
            // assert_eq!(result[0], 1)
        }

        #[test]
        fn neg() {
            let mut result = [0u32; 1];
            execute_kernel("test_bn254_neg", &mut result);
            // assert_eq!(result[0], 0)
        }

        #[test]
        fn mont_reduction() {
            let mut result = [0u32; 1];
            execute_kernel("test_bn254_mont_reduction", &mut result);
            // assert_eq!(result[0], 3)
        }

        #[test]
        fn exp() {
            let mut result = [0u32; 1];
            execute_kernel("test_bn254_exp", &mut result);
            // assert_eq!(result[0], 125)
        }
        
        #[test]
        fn eq() {
            let mut result = [0u32; 1];
            execute_kernel("test_bn254_eq", &mut result);
            // assert_eq!(result[0], 1)
        }
        #[test]
        fn ineq() {
            let mut result = [0u32; 1];
            execute_kernel("test_bn254_ineq", &mut result);
            // assert_eq!(result[0], 1)
        }
    }

    /*
      mod fp_tests {
        use lambdaworks_math::unsigned_integer::traits::U32Limbs;
        use proptest::collection;

        use super::*;

        enum FEOrInt {
            Elem(FE),
            Int(u32),
        }

        fn execute_kernel(name: &str, a: &FE, b: FEOrInt) -> FE {
            let state = MetalState::new(None).unwrap();
            let pipeline = state.setup_pipeline(name).unwrap();

            // conversion needed because of possible difference of endianess between host and
            // device (Metal's UnsignedInteger has 32bit limbs).
            let a = a.value().to_u32_limbs();
            let result_buffer = state.alloc_buffer::<u32>(8);

            let (command_buffer, command_encoder) = match b {
                FEOrInt::Elem(b) => {
                    let b = b.value().to_u32_limbs();
                    let a_buffer = state.alloc_buffer_data(&a);
                    let b_buffer = state.alloc_buffer_data(&b);

                    state.setup_command(
                        &pipeline,
                        Some(&[(0, &a_buffer), (1, &b_buffer), (2, &result_buffer)]),
                    )
                }
                FEOrInt::Int(b) => {
                    let a_buffer = state.alloc_buffer_data(&a);
                    let b_buffer = state.alloc_buffer_data(&[b]);

                    state.setup_command(
                        &pipeline,
                        Some(&[(0, &a_buffer), (1, &b_buffer), (2, &result_buffer)]),
                    )
                }
            };

            let threadgroup_size = MTLSize::new(1, 1, 1);
            let threadgroup_count = MTLSize::new(1, 1, 1);

            command_encoder.dispatch_thread_groups(threadgroup_count, threadgroup_size);
            command_encoder.end_encoding();

            command_buffer.commit();
            command_buffer.wait_until_completed();

            let limbs = MetalState::retrieve_contents::<u32>(&result_buffer);
            FE::from_raw(&U::from_u32_limbs(&limbs))
        }

        prop_compose! {
            fn rand_u32()(n in any::<u32>()) -> u32 { n }
        }

        prop_compose! {
            fn rand_limbs()(vec in collection::vec(rand_u32(), 8)) -> Vec<u32> {
                vec
            }
        }

        prop_compose! {
            fn rand_felt()(limbs in rand_limbs()) -> FE {
                FE::from(&U256::from_u32_limbs(&limbs))
            }
        }

        use FEOrInt::{Elem, Int};

        proptest! {
            #[test]
            fn add(a in rand_felt(), b in rand_felt()) {
                objc::rc::autoreleasepool(|| {
                    let result = execute_kernel("test_fpbn254_add", &a, Elem(b.clone()));
                    prop_assert_eq!(result, a + b);
                    Ok(())
                }).unwrap();
            }

            #[test]
            fn sub(a in rand_felt(), b in rand_felt()) {
                objc::rc::autoreleasepool(|| {
                    let result = execute_kernel("test_fpbn254_sub", &a, Elem(b.clone()));
                    prop_assert_eq!(result, a - b);
                    Ok(())
                }).unwrap();
            }

            #[test]
            fn mul(a in rand_felt(), b in rand_felt()) {
                objc::rc::autoreleasepool(|| {
                    let result = execute_kernel("test_fpbn254_mul", &a, Elem(b.clone()));
                    prop_assert_eq!(result, a * b);
                    Ok(())
                }).unwrap();
            }

            #[test]
            fn pow(a in rand_felt(), b in rand_u32()) {
                objc::rc::autoreleasepool(|| {
                    let result = execute_kernel("test_fpbn254_pow", &a, Int(b));
                    prop_assert_eq!(result, a.pow(b));
                    Ok(())
                }).unwrap();
            }
        }
    }
    */

    /*   
    mod ec_tests {
        use lambdaworks_math::unsigned_integer::traits::U32Limbs;

        use super::*;

        pub type P = ShortWeierstrassProjectivePoint<BN254Curve>;

        fn execute_kernel(name: &str, p: &P, q: &P) -> Vec<u32> {
            let state = MetalState::new(None).unwrap();
            let pipeline = state.setup_pipeline(name).unwrap();

            // conversion needed because of possible difference of endianess between host and
            // device (Metal's UnsignedInteger has 32bit limbs).
            let p_coordinates: Vec<u32> = p
                .coordinates()
                .into_iter()
                .map(|felt| felt.value().to_u32_limbs())
                .flatten()
                .collect();
            let q_coordinates: Vec<u32> = q
                .coordinates()
                .into_iter()
                .map(|felt| felt.value().to_u32_limbs())
                .flatten()
                .collect();
            let p_buffer = state.alloc_buffer_data(&p_coordinates);
            let q_buffer = state.alloc_buffer_data(&q_coordinates);
            let result_buffer = state.alloc_buffer::<u32>(24);

            let (command_buffer, command_encoder) = state.setup_command(
                &pipeline,
                Some(&[(0, &p_buffer), (1, &q_buffer), (2, &result_buffer)]),
            );

            let threadgroup_size = MTLSize::new(1, 1, 1);
            let threadgroup_count = MTLSize::new(1, 1, 1);

            command_encoder.dispatch_thread_groups(threadgroup_count, threadgroup_size);
            command_encoder.end_encoding();

            command_buffer.commit();
            command_buffer.wait_until_completed();

            MetalState::retrieve_contents::<u32>(&result_buffer)
        }

        prop_compose! {
            fn rand_u128()(n in any::<u128>()) -> u128 { n }
        }

        prop_compose! {
            fn rand_point()(n in rand_u128()) -> P {
                BN254Curve::generator().operate_with_self(n)
            }
        }

        proptest! {
            #[test]
            fn add(p in rand_point(), q in rand_point()) {
                objc::rc::autoreleasepool(|| {
                    let result = execute_kernel("bn254_add", &p, &q);
                    let cpu_result = p
                        .operate_with(&q)
                        .to_u32_limbs();
                    prop_assert_eq!(result, cpu_result);
                    Ok(())
                }).unwrap();
            }

            #[test]
            fn add_with_self(p in rand_point()) {
                objc::rc::autoreleasepool(|| {
                    let result = execute_kernel("bn254_add", &p, &p);
                    let cpu_result: Vec<u32> = p
                        .operate_with_self(2_u64)
                        .to_u32_limbs();
                    prop_assert_eq!(result, cpu_result);
                    Ok(())
                }).unwrap();
            }

            #[test]
            fn add_with_infinity_rhs(p in rand_point()) {
                objc::rc::autoreleasepool(|| {
                    let infinity = p.operate_with_self(0_u64);
                    let result = execute_kernel("bn254_add", &p, &infinity);
                    let cpu_result: Vec<u32> = p
                        .operate_with(&infinity)
                        .to_u32_limbs();
                    prop_assert_eq!(result, cpu_result);
                    Ok(())
                }).unwrap();
            }

            #[test]
            fn add_with_infinity_lhs(p in rand_point()) {
                objc::rc::autoreleasepool(|| {
                    let infinity = p.operate_with_self(0_u64);
                    let result = execute_kernel("bn254_add", &infinity, &p);
                    let cpu_result: Vec<u32> = infinity
                        .operate_with(&p)
                        .to_u32_limbs();
                    prop_assert_eq!(result, cpu_result);
                    Ok(())
                }).unwrap();
            }
        }

        #[test]
        fn infinity_plus_infinity_should_equal_infinity() {
            let infinity = BN254Curve::generator().operate_with_self(0_u64);
            let result = execute_kernel("bn254_add", &infinity, &infinity);
            let cpu_result: Vec<u32> = infinity.operate_with(&infinity).to_u32_limbs();
            assert_eq!(result, cpu_result);
        }
    }
    */
}
