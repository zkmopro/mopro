use ark_bn254::FqConfig;
use ark_ff::{
    biginteger::{arithmetic as fa, BigInt},
    fields::models::{MontBackend, MontConfig},
    Fp,
};

// Reference: https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/models/fp/montgomery_backend.rs#L373-L389
const N: usize = 4;
pub fn into_bigint(a: Fp<MontBackend<FqConfig, N>, N>) -> BigInt<N> {
    let a = a.0;
    raw_reduction(a)
}

pub fn raw_reduction(a: BigInt<N>) -> BigInt<N> {
    let mut r = a.0; // parse into [u64; N]

    // Montgomery Reduction
    for i in 0..N {
        let k = r[i].wrapping_mul(<FqConfig as MontConfig<N>>::INV);
        let mut carry = 0;

        fa::mac_with_carry(
            r[i],
            k,
            <FqConfig as MontConfig<N>>::MODULUS.0[0],
            &mut carry,
        );
        for j in 1..N {
            r[(j + i) % N] = fa::mac_with_carry(
                r[(j + i) % N],
                k,
                <FqConfig as MontConfig<N>>::MODULUS.0[j],
                &mut carry,
            );
        }
        r[i % N] = carry;
    }
    BigInt::new(r)
}
