use ark_ff_3::prelude::*;
use ark_std_3::vec::Vec;
use ark_ff_3::ToBytes;
use ark_ff_3::FromBytes;
use ark_serialize_3::Read;
use ark_std_3::One;
use std::str::FromStr;
use ark_bls12_377 as bls377;
use ark_serialize_3::Write;
use lazy_static::*;
use ark_bls12_377::{Fq};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

lazy_static! {
    pub static ref MONT_ALPHA: Fq = Fq::from_str("80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410946").unwrap();
    pub static ref MONT_BETA: Fq = Fq::from_str("207913745465435703873309001080708636764682407053260289242004673792544811711776497012639468972230205966814119707502").unwrap();

    pub static ref ED_COEFF_A: Fq = Fq::from_str("157163064917902313978814213261261898218646390773518349738660969080500653509624033038447657619791437448628296189665").unwrap();
    pub static ref ED_COEFF_D: Fq = Fq::from_str("101501361095066780517536410023107951769097300825221174390295061910482811707540513312796446149590693954692781734188").unwrap();

    pub static ref ED_COEFF_DD: Fq = Fq::from_str("136396142414293534522166394536258004439411625840037520960350109084686791562955032044926524798337324377515360555012").unwrap();
    pub static ref ED_COEFF_K: Fq = Fq::from_str("14127858815617975033680055377622475342429738925160381380815955502653114777569241314884161457101288630590399651847").unwrap();

    pub static ref ED_COEFF_SQRT_NEG_A: Fq = Fq::from_str("237258690121739794091542072758217926613126300728951001700615245829450947395696022962309165363059235018940120114447").unwrap();
    pub static ref ED_COEFF_SQRT_NEG_A_INV: Fq = Fq::from_str("85493388116597753391764605746615521878764370024930535315959456146985744891605502660739892967955718798310698221510").unwrap();
}


#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct EdwardsAffine {
    pub x: Fq,
    pub y: Fq,
}

impl Default for EdwardsAffine {
    fn default() -> Self {
        let x = Fq::default();
        let y = Fq::default();
        Self { x, y }
    }
}

impl ToBytes for EdwardsAffine {
    #[inline]
    fn write<W: Write>(&self, mut writer: W) -> ark_std::io::Result<()> {
        self.x.write(&mut writer)?;
        self.y.write(&mut writer)
    }
}

impl FromBytes for EdwardsAffine {
    #[inline]
    fn read<R: Read>(mut reader: R) -> ark_std::io::Result<Self> {
        let x = Fq::read(&mut reader)?;
        let y = Fq::read(&mut reader)?;

        Ok(Self { x, y })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct EdwardsProjective {
    pub x: Fq,
    pub y: Fq,
    pub z: Fq,
}

/// for Edwards curves
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct ExEdwardsAffine {
    pub x: Fq,
    pub y: Fq,
    pub t: Fq,
}

impl Default for ExEdwardsAffine {
    fn default() -> Self {
        Self {
            x: Fq::zero(),
            y: Fq::one(),
            t: Fq::zero(),
        }
    }
}

impl ToBytes for ExEdwardsAffine {
    #[inline]
    fn write<W: Write>(&self, mut writer: W) -> ark_std::io::Result<()> {
        self.x.write(&mut writer)?;
        self.y.write(&mut writer)?;
        self.t.write(&mut writer)
    }
}

impl FromBytes for ExEdwardsAffine {
    #[inline]
    fn read<R: Read>(mut reader: R) -> ark_std::io::Result<Self> {
        let x = Fq::read(&mut reader)?;
        let y = Fq::read(&mut reader)?;
        let t = Fq::read(&mut reader)?;

        Ok(Self { x, y, t })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct ExEdwardsProjective {
    pub x: Fq,
    pub y: Fq,
    pub t: Fq,
    pub z: Fq,
}

impl ExEdwardsProjective {
    pub fn zero() -> ExEdwardsProjective {
        ExEdwardsProjective {
            x: Fq::zero(),
            y: Fq::one(),
            t: Fq::zero(),
            z: Fq::one(),
        }
    }
}

#[inline]
fn get_alpha_beta() -> (Fq, Fq) {
    (*MONT_ALPHA, *MONT_BETA)
}

#[inline]
fn get_a_d() -> (Fq, Fq) {
    (*ED_COEFF_A, *ED_COEFF_D)
}

#[inline]
fn get_dd_k() -> (Fq, Fq) {
    (*ED_COEFF_DD, *ED_COEFF_K)
}


#[inline]
fn get_sqrt_neg_a() -> (Fq, Fq) {
    (*ED_COEFF_SQRT_NEG_A, *ED_COEFF_SQRT_NEG_A_INV)
}

/// we don't introduce new Rust struct here, instead we reuse ExEdwardsAffine to represent the new coordinates under -x^2 + y^2 = 1 + dd * x^2 * y^2
// #[allow(unused)]
pub fn edwards_to_neg_one_a(ed: ExEdwardsAffine) -> ExEdwardsAffine {
    let (divisor, _) = get_sqrt_neg_a();

    let t = ed.x * divisor * ed.y;

    ExEdwardsAffine {
        x: ed.x * divisor,
        y: ed.y,
        t,
    }
}

/// we don't introduce new Rust struct here, instead we reuse extend edwards affine to represent the new coordinates under -x^2 + y^2 = 1 + dd * x^2 * y^2
#[allow(unused)]
pub fn edwards_from_neg_one_a(ed: ExEdwardsAffine) -> ExEdwardsAffine {
    let (_, multiplier) = get_sqrt_neg_a();

    let t = ed.x * multiplier.clone() * ed.y;

    ExEdwardsAffine {
        x: ed.x * multiplier,
        y: ed.y,
        t,
    }
}

#[allow(unused)]
pub fn sw_to_edwards(g: EdwardsAffine) -> ExEdwardsAffine {
    let (alpha, beta) = get_alpha_beta();

    if g.x == Fq::zero() && g.y == Fq::zero() {
        return ExEdwardsAffine { x: Fq::zero(), y: Fq::one(), t: Fq::zero() };
    }

    // first convert sw to montgomery form
    let mont_x = (g.x - alpha) / beta.clone();
    let mont_y = g.y / beta;

    // then from mont to edwards form
    let one = Fq::one();

    if mont_y == Fq::zero() || (mont_x + one == Fq::zero()) {
        return ExEdwardsAffine { x: Fq::zero(), y: Fq::one(), t: Fq::zero() };
    }

    let ed_x = mont_x / mont_y;
    let ed_y = (mont_x - one.clone()) / (mont_x + one);
    let ed_t = ed_x * ed_y;

    ExEdwardsAffine {
        x: ed_x,
        y: ed_y,
        t: ed_t,
    }
}

#[allow(unused)]
pub fn edwards_to_sw(ed: ExEdwardsAffine) -> EdwardsAffine {
    let (alpha, beta) = get_alpha_beta();

    // if it is infinity point on Twisted Edwards, just return infinity point on Short Weierstrass Curve
    if ed.y == Fq::one() || ed.x == Fq::zero() {
        return EdwardsAffine { x: Fq::zero(), y: Fq::zero() };
    }

    // first convert ed form to mont form
    let one = Fq::one();
    let mont_x = (one.clone() + ed.y) / (one.clone() - ed.y);
    let mont_y = (one + ed.y) / (ed.x - ed.x * ed.y);

    // then from mont form to sw form
    let g_x = mont_x * beta.clone() + alpha;
    let g_y = mont_y * beta;

    EdwardsAffine { x: g_x, y: g_y }
}

#[allow(unused)]
pub fn edwards_to_sw_proj(ed: ExEdwardsAffine) -> EdwardsProjective {
    let (alpha, beta) = get_alpha_beta();

    // if it is infinity point on Twisted Edwards, just return infinity point on Short Weierstrass Curve
    if ed.y == Fq::one() || ed.x == Fq::zero() {
        return EdwardsProjective { x: Fq::zero(), y: Fq::one(), z: Fq::zero() };
    }

    // first convert ed form to mont form
    let one = Fq::one();
    let mont_x = (one.clone() + ed.y) / (one.clone() - ed.y);
    let mont_y = (one + ed.y) / (ed.x - ed.x * ed.y);

    // then from mont form to sw form
    let g_x = mont_x * beta.clone() + alpha;
    let g_y = mont_y * beta;

    EdwardsProjective {
        x: g_x,
        y: g_y,
        z: Fq::one(),
    }
}


#[allow(unused)]
fn edwards_affine_to_proj(ed: ExEdwardsAffine) -> ExEdwardsProjective {
    ExEdwardsProjective {
        x: ed.x,
        y: ed.y,
        t: ed.x * ed.y,
        z: Fq::one(),
    }
}

#[allow(unused)]
pub fn edwards_proj_to_affine(ed: ExEdwardsProjective) -> ExEdwardsAffine {

    if ed.z == Fq::zero() {
        return ExEdwardsAffine { x: Fq::zero(), y: Fq::one(), t: Fq::zero() };
    }

    let x = ed.x / ed.z;
    let y = ed.y / ed.z;

    ExEdwardsAffine { x, y, t: x * y }
}

#[allow(non_snake_case)]
pub fn edwards_add_projective(ed1: ExEdwardsProjective, ed2: ExEdwardsProjective) -> ExEdwardsProjective {
    let (_, k) = get_dd_k();  //get_d_k

    let x1 = ed1.x;
    let x2 = ed2.x;
    let y1 = ed1.y;
    let y2 = ed2.y;

    let t1 = ed1.t;
    let t2 = ed2.t;

    let z1 = ed1.z;
    let z2 = ed2.z;

    // doing add arithmetic
    let A = (y1 - x1) * (y2 - x2);
    let B = (y1 + x1) * (y2 + x2);
    let C = k * t1 * t2;
    let D = z1 * z2;
    let D = D + D;
    let E = B - A;
    let F = D - C;
    let G = D + C;
    let H = B + A;

    let x3 = E * F;
    let y3 = G * H;
    let z3 = F * G;

    let t3 = E * H; //x3*y3
    ExEdwardsProjective {
        x: x3,
        y: y3,
        t: t3,
        z: z3,
    }
}

#[allow(dead_code)]
pub fn edwards_affine_to_projective(ed: ExEdwardsAffine) -> ExEdwardsProjective {
    ExEdwardsProjective {
        x: ed.x,
        y: ed.y,
        t: ed.x * ed.y,
        z: Fq::one(),
    }
}

#[allow(non_snake_case)]
pub fn edwards_add_mix_a(ed1: ExEdwardsProjective, ed2: ExEdwardsAffine) -> ExEdwardsProjective {
    let x1 = ed1.x;
    let x2 = ed2.x;
    let y1 = ed1.y;
    let y2 = ed2.y;

    let t1 = ed1.t;
    let t2 = ed2.t;

    // let z1 = ed1.z;
    // let z2 = Fq::one();

    // doing add_mix arithmetic
    let A = (y1 - &x1) * (y2 + &x2);
    let B = (y1 + &x1) * (y2 - &x2);

    let F = B - &A;
    if F.is_zero() { // use double
        return edwards_double(ed1);
    }

    //let C = ed1.z * t2 + ed1.z * t2;
    let C = (ed1.z * &t2).double();
    let D = t1.double();
    let E = D + &C;
    // let F = B - &A;
    let G = B + &A;
    let H = D - &C;

    let x3 = E * &F;
    let y3 = G * &H;
    let t3 = E * &H;
    let z3 = F * &G;

    ExEdwardsProjective {
        x: x3,
        y: y3,
        t: t3,
        z: z3,
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn edwards_double(ed: ExEdwardsProjective) -> ExEdwardsProjective {
    let A = ed.x * ed.x;
    let B = ed.y * ed.y;
    let C = ed.z * ed.z;
    let C = C + C;
    let D = -A;

    let E = ed.x + ed.y;
    let E = E * E;
    let E = E - A - B;

    let G = D + B;
    let F = G - C;
    let H = D - B;

    ExEdwardsProjective {
        x: E * F,
        y: G * H,
        t: E * H,
        z: F * G,
    }
}

/// The result of this function is only approximately `ln(a)`
/// [`Explanation of usage`]
///
/// [`Explanation of usage`]: https://github.com/scipr-lab/zexe/issues/79#issue-556220473
fn ln_without_floats(a: usize) -> usize {
    // log2(a) * ln(2)
    (ark_std::log2(a) * 69 / 100) as usize
}

pub fn multi_scalar_mul(
    bases: &[ExEdwardsAffine],
    scalars: &[<bls377::Fr as PrimeField>::BigInt],
) -> ExEdwardsProjective {
    let size = ark_std::cmp::min(bases.len(), scalars.len());
    let scalars = &scalars[..size];
    let bases = &bases[..size];

    let scalars_and_bases_iter = scalars.iter().zip(bases).filter(|(s, _)| !s.is_zero());

    let c = if size < 32 {
        3
    } else {
        ln_without_floats(size) + 1
    };

    let num_bits: usize = 253;

    let fr_one = bls377::Fr::one().into_repr(); //<bls377::Fr as PrimeField>::BigInt
    let zero: ExEdwardsProjective = ExEdwardsProjective::zero();
    let window_starts: Vec<_> = (0..num_bits).step_by(c).collect();

    // Each window is of size `c`.
    // We divide up the bits 0..num_bits into windows of size `c`, and
    // in parallel process each such window.
    let window_sums: Vec<_> = ark_std::cfg_into_iter!(window_starts)
        .map(|w_start| {
            let mut res = zero;

            // We don't need the "zero" bucket, so we only have 2^c - 1 buckets.
            let mut buckets = vec![zero; (1 << c) - 1];
            // This clone is cheap, because the iterator contains just a
            // pointer and an index into the original vectors.
            scalars_and_bases_iter.clone().for_each(|(&scalar, base)| {
                if scalar == fr_one {
                    // We only process unit scalars once in the first window.
                    if w_start == 0 {
                        res = edwards_add_mix_a(res, base.clone());
                    }
                } else {
                    let mut scalar = scalar;

                    // We right-shift by w_start, thus getting rid of the
                    // lower bits.
                    scalar.divn(w_start as u32);
                    // We mod the remaining bits by 2^{window size}, thus taking `c` bits.
                    let scalar = scalar.as_ref()[0] % (1 << c);

                    // If the scalar is non-zero, we update the corresponding
                    // bucket.
                    // (Recall that `buckets` doesn't have a zero bucket.)
                    if scalar != 0 {
                        buckets[(scalar - 1) as usize] = edwards_add_mix_a(buckets[(scalar - 1) as usize], base.clone());
                    }
                }
            });

            // Compute sum_{i in 0..num_buckets} (sum_{j in i..num_buckets} bucket[j])
            // This is computed below for b buckets, using 2b curve additions.
            //
            // We could first normalize `buckets` and then use mixed-addition
            // here, but that's slower for the kinds of groups we care about
            // (Short Weierstrass curves and Twisted Edwards curves).
            // In the case of Short Weierstrass curves,
            // mixed addition saves ~4 field multiplications per addition.
            // However normalization (with the inversion batched) takes ~6
            // field multiplications per element,
            // hence batch normalization is a slowdown.

            // `running_sum` = sum_{j in i..num_buckets} bucket[j],
            // where we iterate backward from i = num_buckets to 0.

            let mut running_sum = ExEdwardsProjective::zero();
            buckets.into_iter().rev().for_each(|b| {
                running_sum = edwards_add_projective(running_sum, b);
                res = edwards_add_projective(res, running_sum);
            });
            res
        })
        .collect();

    // We store the sum for the lowest window.
    let lowest = *window_sums.first().unwrap();

    // We're traversing windows from high to low.
    let end = window_sums[1..]
        .iter()
        .rev()
        .fold(zero, |mut total, sum_i| {
            total = edwards_add_projective(total, *sum_i);
            for _ in 0..c {
                total = edwards_add_projective(total, total);
            }
            total
        });

    let ret = edwards_add_projective(lowest, end);
    ret
}

