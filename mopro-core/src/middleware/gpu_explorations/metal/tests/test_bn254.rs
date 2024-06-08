#[cfg(all(test))]
mod tests {
    use crate::middleware::gpu_explorations::metal::abstraction::{
        limbs_conversion::{FromLimbs, ToLimbs},
        state::MetalState,
    };

    use ark_bn254::{Fq, G1Projective as G};
    use ark_ff::{BigInt, Field};
    use ark_std::Zero;

    use metal::MTLSize;
    use proptest::prelude::*;

    pub type FE = Fq; // Field Element

    mod unsigned_int_tests {
        use super::*;

        enum BigOrSmallInt {
            Big(BigInteger256),
            Small(usize),
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
                        Some(&[
                            (0, &a_buffer),
                            (1, &b_buffer),
                            (2, &result_buffer),
                            (3, &debug_buffer),
                        ]),
                    )
                }
                BigOrSmallInt::Small(b) => {
                    let a_buffer = state.alloc_buffer_data(&a);
                    let b_buffer = state.alloc_buffer_data(&[b]);
                    state.setup_command(
                        &pipeline,
                        Some(&[
                            (0, &a_buffer),
                            (1, &b_buffer),
                            (2, &result_buffer),
                            (3, &debug_buffer),
                        ]),
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

            BigInteger256::from_u32_limbs(&limbs)
        }

        prop_compose! {
            fn rand_u128()(n in any::<u128>()) -> BigInteger256 { BigInteger256::from_u128(n) }
        }
        prop_compose! {
            fn rand_u32()(n in any::<u32>()) -> BigInteger256 { BigInteger256::from_u32(n) }
        }

        use ark_ff::biginteger::{BigInteger, BigInteger256};
        use num_bigint::BigUint;

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

    mod fp_tests {
        use super::*;

        use proptest::collection;

        enum FEOrInt {
            Elem(FE),
            Int(u32),
        }

        fn execute_kernel(name: &str, a: &FE, b: FEOrInt) -> FE {
            let state = MetalState::new(None).unwrap();
            let pipeline = state.setup_pipeline(name).unwrap();

            let a = a.to_u32_limbs();
            let result_buffer = state.alloc_buffer::<u32>(8);

            let (command_buffer, command_encoder) = match b {
                FEOrInt::Elem(b) => {
                    let b = b.to_u32_limbs();
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
            FE::from_u32_limbs(&limbs)
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
            fn rand_field_element()(limbs in rand_limbs()) -> FE {
                FE::from_u32_limbs(&limbs)
            }
        }

        use FEOrInt::{Elem, Int};

        proptest! {
            #[test]
            fn add(a in rand_field_element(), b in rand_field_element()) {
                let mut result = Fq::default();
                objc::rc::autoreleasepool(|| {
                    result = execute_kernel("fp_bn254_add", &a, Elem(b.clone()));
                });
                let local_add = a + b;
                prop_assert_eq!(result, local_add);
            }

            #[test]
            fn sub(a in rand_field_element(), b in rand_field_element()) {
                let mut result = Fq::default();
                objc::rc::autoreleasepool(|| {
                    result = execute_kernel("fp_bn254_sub", &a, Elem(b.clone()));
                });
                let local_sub = a - b;
                prop_assert_eq!(result, local_sub);
            }

            #[test]
            fn mul(a in rand_field_element(), b in rand_field_element()) {
                let mut result = Fq::default();
                objc::rc::autoreleasepool(|| {
                    result = execute_kernel("fp_bn254_mul", &a, Elem(b.clone()));
                });
                let local_mul = a * b;
                prop_assert_eq!(result, local_mul);
            }

            #[test]
            fn neg(a in rand_field_element()) {
                let mut result = Fq::default();
                objc::rc::autoreleasepool(|| {
                    result = execute_kernel("fp_bn254_neg", &a, Int(0));
                });
                let local_neg = -a;
                prop_assert_eq!(result, local_neg);
            }

            #[test]
            fn pow(a in rand_field_element(), b in rand_u32()) {
                let mut result = Fq::default();
                objc::rc::autoreleasepool(|| {
                    result = execute_kernel("fp_bn254_pow", &a, Int(b));
                });
                let local_pow = a.pow(&[b as u64]);
                prop_assert_eq!(result, local_pow);
            }

            // // TODO: Implement inverse if needed in the future
            // #[test]
            // fn inv(a in rand_field_element()) {
            //     let mut result = Fq::default();
            //     objc::rc::autoreleasepool(|| {
            //         result = execute_kernel("test_bn254_inv", &a, Int(0));
            //     });
            //     let local_inv = a.inverse().unwrap();
            //     println!("a: {:?}", a.0);
            //     println!("a inv: {:?}", local_inv.0);
            //     println!("result: {:?}", result);
            //     prop_assert_eq!(result.into_bigint(), local_inv.0);
            // }
        }
    }

    mod ec_tests {
        use ark_ff::UniformRand;
        use ark_std::rand::thread_rng;

        use super::*;

        fn point_to_u32_limbs(p: &G) -> Vec<u32> {
            p.x.to_u32_limbs()
                .into_iter()
                .chain(p.y.to_u32_limbs())
                .chain(p.z.to_u32_limbs())
                .collect()
        }

        fn execute_kernel(name: &str, p: &G, q: &G) -> Vec<u32> {
            let state = MetalState::new(None).unwrap();
            let pipeline = state.setup_pipeline(name).unwrap();

            let p_coordinates: Vec<u32> = point_to_u32_limbs(p);
            let q_coordinates: Vec<u32> = point_to_u32_limbs(q);

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
            fn rand_point()(_n in any::<u8>()) -> G {
                let rng = &mut thread_rng();
                G::rand(rng)
            }
        }

        proptest! {
            #[test]
            fn add(p in rand_point(), q in rand_point()) {
                let mut result = vec![];
                objc::rc::autoreleasepool(|| {
                    result = execute_kernel("bn254_add", &p, &q);
                });
                let gpu_result = G::new(
                    Fq::from_u32_limbs(&result[0..8]),
                    Fq::from_u32_limbs(&result[8..16]),
                    Fq::from_u32_limbs(&result[16..24]),
                );
                let cpu_result = p + q;
                prop_assert_eq!(gpu_result, cpu_result);
            }

            #[test]
            fn add_with_self(p in rand_point()) {
                let mut result = vec![];
                objc::rc::autoreleasepool(|| {
                    result = execute_kernel("bn254_add", &p, &p);
                });
                let gpu_result = G::new(
                    Fq::from_u32_limbs(&result[0..8]),
                    Fq::from_u32_limbs(&result[8..16]),
                    Fq::from_u32_limbs(&result[16..24]),
                );
                let cpu_result = p + p;
                prop_assert_eq!(gpu_result, cpu_result);
            }

            #[test]
            fn add_with_infinity_rhs(p in rand_point()) {
                let mut result = vec![];
                let infinity = G::zero();
                objc::rc::autoreleasepool(|| {
                    result = execute_kernel("bn254_add", &p, &infinity);
                });
                let gpu_result = G::new(
                    Fq::from_u32_limbs(&result[0..8]),
                    Fq::from_u32_limbs(&result[8..16]),
                    Fq::from_u32_limbs(&result[16..24]),
                );
                let cpu_result = p + infinity;
                prop_assert_eq!(gpu_result, cpu_result);
            }

            #[test]
            fn add_with_infinity_lhs(p in rand_point()) {
                let mut result = vec![];
                let infinity = G::zero();
                objc::rc::autoreleasepool(|| {
                    result = execute_kernel("bn254_add", &infinity, &p);
                });
                let gpu_result = G::new(
                    Fq::from_u32_limbs(&result[0..8]),
                    Fq::from_u32_limbs(&result[8..16]),
                    Fq::from_u32_limbs(&result[16..24]),
                );
                let cpu_result = infinity + p;
                prop_assert_eq!(gpu_result, cpu_result);
            }
        }

        #[test]
        fn infinity_plus_infinity_should_equal_infinity() {
            let infinity = G::zero();
            let result = execute_kernel("bn254_add", &infinity, &infinity);
            let gpu_result = G::new(
                Fq::from_u32_limbs(&result[0..8]),
                Fq::from_u32_limbs(&result[8..16]),
                Fq::from_u32_limbs(&result[16..24]),
            );
            let cpu_result = infinity + infinity;
            assert_eq!(gpu_result, cpu_result);
        }
    }
}
