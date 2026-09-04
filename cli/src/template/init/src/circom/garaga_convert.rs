use super::snarkjs_types::SnarkJsVerificationKey;
use garaga_rs::calldata::full_proof_with_hints::groth16::{Groth16Proof, Groth16VerificationKey};
use garaga_rs::calldata::{G1PointBigUint, G2PointBigUint};
use garaga_rs::definitions::{get_modulus_from_curve_id, CurveID};
use num_bigint::BigUint;
use std::str::FromStr;
use std::sync::LazyLock;

/// BN254 base-field modulus (Fq). Coordinates must be in `[0, p)`.
static BN254_BASE_FIELD: LazyLock<BigUint> =
    LazyLock::new(|| get_modulus_from_curve_id(CurveID::BN254));

/// BN254 scalar-field order (Fr). Public inputs must be in `[0, n)`.
/// Same value as `BN254PrimeField::get_curve_params().n` in garaga_rs.
static BN254_SCALAR_FIELD: LazyLock<BigUint> = LazyLock::new(|| {
    BigUint::parse_bytes(
        b"30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001",
        16,
    )
    .expect("valid BN254 scalar-field hex")
});

/// Parse a decimal field element and require it to lie in the BN254 base field (Fq).
pub(crate) fn parse_biguint(s: &str) -> Result<BigUint, String> {
    let value = BigUint::from_str(s).map_err(|e| format!("invalid coordinate '{s}': {e}"))?;
    if value >= *BN254_BASE_FIELD {
        return Err(format!("coordinate '{s}' is outside BN254 base field"));
    }
    Ok(value)
}

fn parse_public_input_biguint(s: &str) -> Result<BigUint, String> {
    let value = BigUint::from_str(s).map_err(|e| format!("invalid coordinate '{s}': {e}"))?;
    if value >= *BN254_SCALAR_FIELD {
        return Err(format!("public input '{s}' is outside BN254 scalar field"));
    }
    Ok(value)
}

pub(crate) fn snarkjs_g1_to_garaga(coords: &[String]) -> Result<G1PointBigUint, String> {
    if coords.len() < 2 {
        return Err("G1 point must have at least two coordinates".to_string());
    }
    Ok(G1PointBigUint {
        x: parse_biguint(&coords[0])?,
        y: parse_biguint(&coords[1])?,
    })
}

/// SnarkJS G2 layout: `[[x0, x1], [y0, y1], [z0, z1]]` → Garaga `[x0, x1, y0, y1]`.
pub(crate) fn snarkjs_g2_to_garaga(rows: &[Vec<String>]) -> Result<G2PointBigUint, String> {
    if rows.len() < 2 || rows[0].len() < 2 || rows[1].len() < 2 {
        return Err("G2 point must have two rows of two coordinates".to_string());
    }
    Ok(G2PointBigUint {
        x0: parse_biguint(&rows[0][0])?,
        x1: parse_biguint(&rows[0][1])?,
        y0: parse_biguint(&rows[1][0])?,
        y1: parse_biguint(&rows[1][1])?,
    })
}

pub(crate) fn public_inputs_to_biguint(public_inputs: &[String]) -> Result<Vec<BigUint>, String> {
    public_inputs
        .iter()
        .map(|s| parse_public_input_biguint(s))
        .collect()
}

pub(crate) fn mopro_g2_to_garaga(x: &[String], y: &[String]) -> Result<G2PointBigUint, String> {
    if x.len() < 2 || y.len() < 2 {
        return Err("G2 point must have two x and two y coordinates".to_string());
    }
    Ok(G2PointBigUint {
        x0: parse_biguint(&x[0])?,
        x1: parse_biguint(&x[1])?,
        y0: parse_biguint(&y[0])?,
        y1: parse_biguint(&y[1])?,
    })
}

pub(crate) fn to_groth16_proof_from_mopro(
    a_x: &str,
    a_y: &str,
    b_x: &[String],
    b_y: &[String],
    c_x: &str,
    c_y: &str,
    public_inputs: &[String],
) -> Result<Groth16Proof, String> {
    Ok(Groth16Proof {
        a: snarkjs_g1_to_garaga(&[a_x.to_string(), a_y.to_string()])?,
        b: mopro_g2_to_garaga(b_x, b_y)?,
        c: snarkjs_g1_to_garaga(&[c_x.to_string(), c_y.to_string()])?,
        public_inputs: public_inputs_to_biguint(public_inputs)?,
        image_id_journal_risc0: None,
        vkey_public_values_sp1: None,
    })
}

pub(crate) fn to_groth16_vk(vk: &SnarkJsVerificationKey) -> Result<Groth16VerificationKey, String> {
    let mut values: Vec<BigUint> = Vec::new();
    values.extend(snarkjs_g1_to_garaga(&vk.vk_alpha_1)?.flatten());
    values.extend(snarkjs_g2_to_garaga(&vk.vk_beta_2)?.flatten());
    values.extend(snarkjs_g2_to_garaga(&vk.vk_gamma_2)?.flatten());
    values.extend(snarkjs_g2_to_garaga(&vk.vk_delta_2)?.flatten());
    for ic_point in &vk.ic {
        values.extend(snarkjs_g1_to_garaga(ic_point)?.flatten());
    }
    Ok(Groth16VerificationKey::from(values))
}
