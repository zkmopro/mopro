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
            fn to_u32(&self) -> u32;
        }

        trait FromLimbs {
            fn from_u32_limbs(limbs: &[u32]) -> Self;
            fn from_u128(num: u128) -> Self;
            fn from_u32(num: u32) -> Self;
        }

        impl ToLimbs for BigInteger256 {
            fn to_u32_limbs(&self) -> Vec<u32> {
                let mut limbs = Vec::new();

                self.to_bytes_be().chunks(8).rev().for_each(|chunk| {
                    let high = u32::from_be_bytes(chunk[0..4].try_into().unwrap());
                    let low = u32::from_be_bytes(chunk[4..8].try_into().unwrap());
                    limbs.push(low);
                    limbs.push(high);
                });

                /*  restore the limbs to BigInt<4>
                // let mut big_int = [0u64; 4];
                // for (i, limb) in limbs.chunks(2).enumerate() {
                //     let low = u64::from(limb[0]);
                //     let high = u64::from(limb[1]);
                //     big_int[i] = (low << 32) | high;
                // }
                // let restored_bigint = BigInt(big_int);
                // println!("restored_bigint: {:?}", restored_bigint);
                // println!("self: {:?}", self);
                // assert!(restored_bigint == *self);
                */
                limbs
            }
            fn to_u32(&self) -> u32 {
                let byte = self.0[3].to_be_bytes(); // last limb
                u32::from_be_bytes(byte[4..8].try_into().unwrap())
            }
        }

        impl FromLimbs for BigInteger256 {
            fn from_u32_limbs(limbs: &[u32]) -> Self {
                let mut big_int = [0u64; 4];
                for (i, limb) in limbs.chunks(2).enumerate() {
                    let low = u64::from(limb[0]);
                    let high = u64::from(limb[1]);
                    big_int[i] = low | (high << 32);
                    println!("low: {:x}, high: {:x}, big_int[{}]: {:x}", low, high, i, big_int[i]);
                }
                BigInt(big_int)
            }
            fn from_u128(num: u128) -> Self {
                let high = (num >> 64) as u64;
                let low = num as u64;
                // BigInt([0, 0, high, low])
                BigInt([low, high, 0, 0])
            }
            fn from_u32(num: u32) -> Self {
                // BigInt([0, 0, 0, num as u64])
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
                    println!("a: {:?}", a);
                    println!("b: {:?}", b);
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

            // Read the debug information
            let debug_data = MetalState::retrieve_contents::<u32>(&debug_buffer);

            // Print the values of a and b
            println!("Value of a:");
            for i in 0..8 {
                println!("Limb {}: 0x{:08X}", i, debug_data[i]);
            }

            println!("Value of b:");
            for i in 0..8 {
                println!("Limb {}: 0x{:08X}", i, debug_data[i + 8]);
            }
            println!("Value of result:");
            for i in 0..8 {
                println!("Limb {}: 0x{:08X}", i, debug_data[i + 16]);
            }
            println!(">>> limbs: {:?}", limbs);
            BigInteger256::from_u32_limbs(&limbs)
        }

        prop_compose! {
            fn rand_u128()(n in any::<u128>()) -> BigInteger256 { BigInteger256::from_u128(n) }
        }
        prop_compose! {
            fn rand_u32()(n in any::<u32>()) -> BigInteger256 { BigInteger256::from_u32(n) }
        }

        use ark_ff::BigInteger;
        use num_bigint::ToBigUint;
        // use ark_ff::{BigInt, BigInteger};
        // use lambdaworks_math::unsigned_integer::traits::U32Limbs;
        use BigOrSmallInt::{Big, Small};

        proptest! {
            #[test]
            fn add(a in rand_u128(), b in rand_u128()) {
                let mut result = BigInteger256::default();

                // // BigInt form
                // // [low, high, 0, 0]
                // // 2^64 - 1:  18446744073709551615
                // // 2^128 - 1: 340282366920938463463374607431768211455
                // let mut tmp_a: BigInt<4> = BigInt!("340282366920938463463374607431768211455");
                // let mut tmp_b: BigInt<4> = BigInt!("1");

                // println!("tmp_a: {:?}", tmp_a);
                // println!("tmp_b: {:?}", tmp_b);
                
                // objc::rc::autoreleasepool(|| {
                //     result = execute_kernel("test_uint_add", (tmp_a, Big(tmp_b)));
                // });
            
                // tmp_a.add_with_carry(&tmp_b);
                // if tmp_a == result {
                //     println!("result: {:?}", result);
                //     println!("ok\n");
                // } else {
                //     // show the difference between tmp and result
                //     let mut diff = tmp_a.clone();
                //     diff.sub_with_borrow(&result);
                //     println!("tmp   : {:?}", tmp_a);
                //     println!("result: {:?}", result);
                //     println!("diff: {:?}\n", diff);
                // }

                println!("a: {:?}", a);
                println!("b: {:?}", b);
                                
                objc::rc::autoreleasepool(|| {
                    result = execute_kernel("test_uint_add", (a, Big(b)));
                });

                let mut tmp = a.clone();
                tmp.add_with_carry(&b);


                if tmp == result {
                    println!("ok\n");
                } else {
                    // show the difference between tmp and result
                    let mut diff = tmp.clone();
                    diff.sub_with_borrow(&result);
                    println!("tmp   : {:?}", tmp);
                    println!("result: {:?}", result);
                    println!("diff: {:?}\n", diff);
                }

                prop_assert_eq!(result, tmp);
            }

            #[test]
            fn sub(a in rand_u128(), b in rand_u128()) {
                let mut result = BigInteger256::default();
                objc::rc::autoreleasepool(|| {
                    let a = std::cmp::max(a, b);
                    let b = std::cmp::min(a, b);

                    let result = execute_kernel("test_uint_sub", (a, Big(b)));
                });
                let mut tmp = a;
                tmp.sub_with_borrow(&b);
                prop_assert_eq!(result, tmp);
            }

            #[test]
            fn prod(a in rand_u128(), b in rand_u32()) {
                let mut result = BigInteger256::default();
                objc::rc::autoreleasepool(|| {
                    let result = execute_kernel("test_uint_prod", (a, Big(b)));
                });
                let mut tmp = a;
                tmp.muln(b.to_u32());
                prop_assert_eq!(result, tmp);
            }

            // #[test]
            // fn shl(a in rand_u128(), b in any::<usize>()) {
            //     objc::rc::autoreleasepool(|| {
            //         let b = b % 256; // so it doesn't overflow
            //         let result = execute_kernel("test_uint_shl", (a, Small(b)));
            //         prop_assert_eq!(result, a << b);
            //         Ok(())
            //     }).unwrap();
            // }

            // #[test]
            // fn shr(a in rand_u128(), b in any::<usize>()) {
            //     objc::rc::autoreleasepool(|| {
            //         let b = b % 256; // so it doesn't overflow
            //         let result = execute_kernel("test_uint_shr", (a, Small(b)));
            //         prop_assert_eq!(result, a >> b);
            //         Ok(())
            //     }).unwrap();
            // }
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
