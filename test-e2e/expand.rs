#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
extern crate core;
use mopro_ffi::{GenerateProofResult, MoproError, ProofCalldata, G1, G2};
fn generate_circom_proof(
    in0: String,
    in1: std::collections::HashMap<String, Vec<String>>,
) -> Result<mopro_ffi::GenerateProofResult, mopro_ffi::MoproError> {
    let name = match std::path::Path::new(in0.as_str()).file_name() {
        Some(v) => v,
        None => {
            return Err(
                mopro_ffi::MoproError::CircomError({
                    let res = ::alloc::fmt::format(
                        format_args!("failed to parse file name from zkey_path"),
                    );
                    res
                }),
            );
        }
    };
    let witness_fn = get_circom_wtns_fn(name.to_str().unwrap())?;
    mopro_ffi::generate_circom_proof_wtns(in0, in1, witness_fn.clone())
        .map_err(|e| mopro_ffi::MoproError::CircomError({
            let res = ::alloc::fmt::format(format_args!("Unknown ZKEY: {0}", e));
            res
        }))
}
fn verify_circom_proof(
    in0: String,
    in1: Vec<u8>,
    in2: Vec<u8>,
) -> Result<bool, mopro_ffi::MoproError> {
    mopro_ffi::verify_circom_proof(in0, in1, in2)
        .map_err(|e| {
            mopro_ffi::MoproError::CircomError({
                let res = ::alloc::fmt::format(
                    format_args!("Verification error: {0}", e),
                );
                res
            })
        })
}
fn to_ethereum_proof(in0: Vec<u8>) -> mopro_ffi::ProofCalldata {
    mopro_ffi::to_ethereum_proof(in0)
}
fn to_ethereum_inputs(in0: Vec<u8>) -> Vec<String> {
    mopro_ffi::to_ethereum_inputs(in0)
}
fn generate_halo2_proof(
    in0: String,
    in1: String,
    in2: std::collections::HashMap<String, Vec<String>>,
) -> Result<mopro_ffi::GenerateProofResult, mopro_ffi::MoproError> {
    let name = std::path::Path::new(in1.as_str()).file_name().unwrap();
    let proving_fn = get_halo2_proving_circuit(name.to_str().unwrap())
        .map_err(|e| {
            mopro_ffi::MoproError::Halo2Error({
                let res = ::alloc::fmt::format(
                    format_args!("error getting proving circuit: {0}", e),
                );
                res
            })
        })?;
    proving_fn(&in0, &in1, in2)
        .map(|(proof, inputs)| mopro_ffi::GenerateProofResult {
            proof,
            inputs,
        })
        .map_err(|e| mopro_ffi::MoproError::Halo2Error({
            let res = ::alloc::fmt::format(format_args!("halo2 error: {0}", e));
            res
        }))
}
fn verify_halo2_proof(
    in0: String,
    in1: String,
    in2: Vec<u8>,
    in3: Vec<u8>,
) -> Result<bool, mopro_ffi::MoproError> {
    let name = std::path::Path::new(in1.as_str()).file_name().unwrap();
    let verifying_fn = get_halo2_verifying_circuit(name.to_str().unwrap())
        .map_err(|e| {
            mopro_ffi::MoproError::Halo2Error({
                let res = ::alloc::fmt::format(
                    format_args!("error getting verification circuit: {0}", e),
                );
                res
            })
        })?;
    verifying_fn(&in0, &in1, in2, in3)
        .map_err(|e| {
            mopro_ffi::MoproError::Halo2Error({
                let res = ::alloc::fmt::format(
                    format_args!("error verifying proof: {0}", e),
                );
                res
            })
        })
}
#[allow(dead_code)]
mod __unused {
    const _: &[u8] = b"[package]\nname = \"test-e2e\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[lib]\nname = \"mopro_bindings\"\ncrate-type = [\"lib\", \"cdylib\", \"staticlib\"]\n\n[[bin]]\nname = \"ios\"\n\n[[bin]]\nname = \"android\"\n\n[dependencies]\nmopro-ffi = { path = \"../mopro-ffi\", features = [\"halo2\", \"circom\"] }\nuniffi = \"0.28.0\"\n\n# Circom dependencies\nrust-witness = \"0.1.0\"\nnum-bigint = { version = \"0.4.0\" }\n\n# Halo2 dependencies\nhalo2-fibonacci = { git = \"https://github.com/ElusAegis/halo2-fibonacci-sample.git\" }\nhalo2-keccak-256 = { git = \"https://github.com/ElusAegis/halo2-keccak-stable.git\" }\nhyperplonk-fibonacci = { package = \"hyperplonk-fibonacci\", git = \"https://github.com/sifnoc/plonkish-fibonacci-sample.git\" }\ngemini-fibonacci = { package = \"gemini-fibonacci\", git = \"https://github.com/sifnoc/plonkish-fibonacci-sample.git\" }\n\n[build-dependencies]\nmopro-ffi = { path = \"../mopro-ffi\" }\nuniffi = { version = \"0.28.0\", features = [\"build\"] }\n\n# Circom dependencies\nrust-witness = \"0.1.0\"\n\n[dev-dependencies]\nuniffi = { version = \"0.28.0\", features = [\"bindgen-tests\"] }\n";
}
#[doc(hidden)]
pub struct UniFfiTag;
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn ffi_mopro_bindings_uniffi_contract_version() -> u32 {
    26u32
}
/// Export namespace metadata.
///
/// See `uniffi_bindgen::macro_metadata` for how this is used.
const UNIFFI_META_CONST_NAMESPACE_MOPRO: ::uniffi::MetadataBuffer = ::uniffi::MetadataBuffer::from_code(
        ::uniffi::metadata::codes::NAMESPACE,
    )
    .concat_str("mopro_bindings")
    .concat_str("mopro");
#[doc(hidden)]
#[no_mangle]
pub static UNIFFI_META_NAMESPACE_MOPRO: [u8; UNIFFI_META_CONST_NAMESPACE_MOPRO.size] = UNIFFI_META_CONST_NAMESPACE_MOPRO
    .into_array();
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn ffi_mopro_bindings_rustbuffer_alloc(
    size: u64,
    call_status: &mut uniffi::RustCallStatus,
) -> uniffi::RustBuffer {
    uniffi::ffi::uniffi_rustbuffer_alloc(size, call_status)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rustbuffer_from_bytes(
    bytes: uniffi::ForeignBytes,
    call_status: &mut uniffi::RustCallStatus,
) -> uniffi::RustBuffer {
    uniffi::ffi::uniffi_rustbuffer_from_bytes(bytes, call_status)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rustbuffer_free(
    buf: uniffi::RustBuffer,
    call_status: &mut uniffi::RustCallStatus,
) {
    uniffi::ffi::uniffi_rustbuffer_free(buf, call_status);
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rustbuffer_reserve(
    buf: uniffi::RustBuffer,
    additional: u64,
    call_status: &mut uniffi::RustCallStatus,
) -> uniffi::RustBuffer {
    uniffi::ffi::uniffi_rustbuffer_reserve(buf, additional, call_status)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_poll_u8(
    handle: ::uniffi::Handle,
    callback: ::uniffi::RustFutureContinuationCallback,
    data: u64,
) {
    ::uniffi::ffi::rust_future_poll::<u8, crate::UniFfiTag>(handle, callback, data);
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_cancel_u8(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_cancel::<u8, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_complete_u8(
    handle: ::uniffi::Handle,
    out_status: &mut ::uniffi::RustCallStatus,
) -> u8 {
    ::uniffi::ffi::rust_future_complete::<u8, crate::UniFfiTag>(handle, out_status)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_free_u8(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_free::<u8, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_poll_i8(
    handle: ::uniffi::Handle,
    callback: ::uniffi::RustFutureContinuationCallback,
    data: u64,
) {
    ::uniffi::ffi::rust_future_poll::<i8, crate::UniFfiTag>(handle, callback, data);
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_cancel_i8(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_cancel::<i8, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_complete_i8(
    handle: ::uniffi::Handle,
    out_status: &mut ::uniffi::RustCallStatus,
) -> i8 {
    ::uniffi::ffi::rust_future_complete::<i8, crate::UniFfiTag>(handle, out_status)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_free_i8(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_free::<i8, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_poll_u16(
    handle: ::uniffi::Handle,
    callback: ::uniffi::RustFutureContinuationCallback,
    data: u64,
) {
    ::uniffi::ffi::rust_future_poll::<u16, crate::UniFfiTag>(handle, callback, data);
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_cancel_u16(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_cancel::<u16, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_complete_u16(
    handle: ::uniffi::Handle,
    out_status: &mut ::uniffi::RustCallStatus,
) -> u16 {
    ::uniffi::ffi::rust_future_complete::<u16, crate::UniFfiTag>(handle, out_status)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_free_u16(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_free::<u16, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_poll_i16(
    handle: ::uniffi::Handle,
    callback: ::uniffi::RustFutureContinuationCallback,
    data: u64,
) {
    ::uniffi::ffi::rust_future_poll::<i16, crate::UniFfiTag>(handle, callback, data);
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_cancel_i16(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_cancel::<i16, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_complete_i16(
    handle: ::uniffi::Handle,
    out_status: &mut ::uniffi::RustCallStatus,
) -> i16 {
    ::uniffi::ffi::rust_future_complete::<i16, crate::UniFfiTag>(handle, out_status)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_free_i16(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_free::<i16, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_poll_u32(
    handle: ::uniffi::Handle,
    callback: ::uniffi::RustFutureContinuationCallback,
    data: u64,
) {
    ::uniffi::ffi::rust_future_poll::<u32, crate::UniFfiTag>(handle, callback, data);
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_cancel_u32(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_cancel::<u32, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_complete_u32(
    handle: ::uniffi::Handle,
    out_status: &mut ::uniffi::RustCallStatus,
) -> u32 {
    ::uniffi::ffi::rust_future_complete::<u32, crate::UniFfiTag>(handle, out_status)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_free_u32(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_free::<u32, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_poll_i32(
    handle: ::uniffi::Handle,
    callback: ::uniffi::RustFutureContinuationCallback,
    data: u64,
) {
    ::uniffi::ffi::rust_future_poll::<i32, crate::UniFfiTag>(handle, callback, data);
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_cancel_i32(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_cancel::<i32, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_complete_i32(
    handle: ::uniffi::Handle,
    out_status: &mut ::uniffi::RustCallStatus,
) -> i32 {
    ::uniffi::ffi::rust_future_complete::<i32, crate::UniFfiTag>(handle, out_status)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_free_i32(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_free::<i32, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_poll_u64(
    handle: ::uniffi::Handle,
    callback: ::uniffi::RustFutureContinuationCallback,
    data: u64,
) {
    ::uniffi::ffi::rust_future_poll::<u64, crate::UniFfiTag>(handle, callback, data);
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_cancel_u64(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_cancel::<u64, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_complete_u64(
    handle: ::uniffi::Handle,
    out_status: &mut ::uniffi::RustCallStatus,
) -> u64 {
    ::uniffi::ffi::rust_future_complete::<u64, crate::UniFfiTag>(handle, out_status)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_free_u64(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_free::<u64, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_poll_i64(
    handle: ::uniffi::Handle,
    callback: ::uniffi::RustFutureContinuationCallback,
    data: u64,
) {
    ::uniffi::ffi::rust_future_poll::<i64, crate::UniFfiTag>(handle, callback, data);
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_cancel_i64(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_cancel::<i64, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_complete_i64(
    handle: ::uniffi::Handle,
    out_status: &mut ::uniffi::RustCallStatus,
) -> i64 {
    ::uniffi::ffi::rust_future_complete::<i64, crate::UniFfiTag>(handle, out_status)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_free_i64(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_free::<i64, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_poll_f32(
    handle: ::uniffi::Handle,
    callback: ::uniffi::RustFutureContinuationCallback,
    data: u64,
) {
    ::uniffi::ffi::rust_future_poll::<f32, crate::UniFfiTag>(handle, callback, data);
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_cancel_f32(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_cancel::<f32, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_complete_f32(
    handle: ::uniffi::Handle,
    out_status: &mut ::uniffi::RustCallStatus,
) -> f32 {
    ::uniffi::ffi::rust_future_complete::<f32, crate::UniFfiTag>(handle, out_status)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_free_f32(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_free::<f32, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_poll_f64(
    handle: ::uniffi::Handle,
    callback: ::uniffi::RustFutureContinuationCallback,
    data: u64,
) {
    ::uniffi::ffi::rust_future_poll::<f64, crate::UniFfiTag>(handle, callback, data);
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_cancel_f64(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_cancel::<f64, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_complete_f64(
    handle: ::uniffi::Handle,
    out_status: &mut ::uniffi::RustCallStatus,
) -> f64 {
    ::uniffi::ffi::rust_future_complete::<f64, crate::UniFfiTag>(handle, out_status)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_free_f64(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_free::<f64, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_poll_pointer(
    handle: ::uniffi::Handle,
    callback: ::uniffi::RustFutureContinuationCallback,
    data: u64,
) {
    ::uniffi::ffi::rust_future_poll::<
        *const ::std::ffi::c_void,
        crate::UniFfiTag,
    >(handle, callback, data);
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_cancel_pointer(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_cancel::<
        *const ::std::ffi::c_void,
        crate::UniFfiTag,
    >(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_complete_pointer(
    handle: ::uniffi::Handle,
    out_status: &mut ::uniffi::RustCallStatus,
) -> *const ::std::ffi::c_void {
    ::uniffi::ffi::rust_future_complete::<
        *const ::std::ffi::c_void,
        crate::UniFfiTag,
    >(handle, out_status)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_free_pointer(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_free::<
        *const ::std::ffi::c_void,
        crate::UniFfiTag,
    >(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_poll_rust_buffer(
    handle: ::uniffi::Handle,
    callback: ::uniffi::RustFutureContinuationCallback,
    data: u64,
) {
    ::uniffi::ffi::rust_future_poll::<
        ::uniffi::RustBuffer,
        crate::UniFfiTag,
    >(handle, callback, data);
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_cancel_rust_buffer(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_cancel::<::uniffi::RustBuffer, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_complete_rust_buffer(
    handle: ::uniffi::Handle,
    out_status: &mut ::uniffi::RustCallStatus,
) -> ::uniffi::RustBuffer {
    ::uniffi::ffi::rust_future_complete::<
        ::uniffi::RustBuffer,
        crate::UniFfiTag,
    >(handle, out_status)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_free_rust_buffer(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_free::<::uniffi::RustBuffer, crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_poll_void(
    handle: ::uniffi::Handle,
    callback: ::uniffi::RustFutureContinuationCallback,
    data: u64,
) {
    ::uniffi::ffi::rust_future_poll::<(), crate::UniFfiTag>(handle, callback, data);
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_cancel_void(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_cancel::<(), crate::UniFfiTag>(handle)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_complete_void(
    handle: ::uniffi::Handle,
    out_status: &mut ::uniffi::RustCallStatus,
) -> () {
    ::uniffi::ffi::rust_future_complete::<(), crate::UniFfiTag>(handle, out_status)
}
#[allow(clippy::missing_safety_doc, missing_docs)]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_mopro_bindings_rust_future_free_void(
    handle: ::uniffi::Handle,
) {
    ::uniffi::ffi::rust_future_free::<(), crate::UniFfiTag>(handle)
}
#[allow(missing_docs)]
#[doc(hidden)]
pub const fn uniffi_reexport_hack() {}
#[allow(unused)]
#[doc(hidden)]
pub trait UniffiCustomTypeConverter {
    type Builtin;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self>
    where
        Self: Sized;
    fn from_custom(obj: Self) -> Self::Builtin;
}
/// Export info about the UDL while used to create us
/// See `uniffi_bindgen::macro_metadata` for how this is used.
const UNIFFI_META_CONST_UDL_MOPRO: ::uniffi::MetadataBuffer = ::uniffi::MetadataBuffer::from_code(
        ::uniffi::metadata::codes::UDL_FILE,
    )
    .concat_str("mopro_bindings")
    .concat_str("mopro")
    .concat_str("mopro");
#[doc(hidden)]
#[no_mangle]
pub static UNIFFI_META_UDL_MOPRO: [u8; UNIFFI_META_CONST_UDL_MOPRO.size] = UNIFFI_META_CONST_UDL_MOPRO
    .into_array();
const _: fn() = || {
    fn assert_impl_all<T: ?Sized + ::std::cmp::Eq + ::std::hash::Hash>() {}
    assert_impl_all::<String>();
};
#[automatically_derived]
unsafe impl ::uniffi::Lower<crate::UniFfiTag> for MoproError {
    type FfiType = ::uniffi::RustBuffer;
    fn write(obj: Self, buf: &mut ::std::vec::Vec<u8>) {
        let error_msg = ::std::string::ToString::to_string(&obj);
        match obj {
            Self::CircomError { .. } => {
                ::uniffi::deps::bytes::BufMut::put_i32(buf, 1);
                <::std::string::String as ::uniffi::Lower<
                    crate::UniFfiTag,
                >>::write(error_msg, buf);
            }
            Self::Halo2Error { .. } => {
                ::uniffi::deps::bytes::BufMut::put_i32(buf, 2);
                <::std::string::String as ::uniffi::Lower<
                    crate::UniFfiTag,
                >>::write(error_msg, buf);
            }
        }
    }
    fn lower(obj: Self) -> ::uniffi::RustBuffer {
        <Self as ::uniffi::Lower<crate::UniFfiTag>>::lower_into_rust_buffer(obj)
    }
}
#[automatically_derived]
unsafe impl ::uniffi::Lift<crate::UniFfiTag> for MoproError {
    type FfiType = ::uniffi::RustBuffer;
    fn try_read(
        buf: &mut &[::std::primitive::u8],
    ) -> ::uniffi::deps::anyhow::Result<Self> {
        {
            ::core::panicking::panic_fmt(format_args!("Can\'t lift flat errors"));
        }
    }
    fn try_lift(v: ::uniffi::RustBuffer) -> ::uniffi::deps::anyhow::Result<Self> {
        {
            ::core::panicking::panic_fmt(format_args!("Can\'t lift flat errors"));
        }
    }
}
#[automatically_derived]
impl ::uniffi::TypeId<crate::UniFfiTag> for MoproError {
    const TYPE_ID_META: ::uniffi::MetadataBuffer = ::uniffi::MetadataBuffer::from_code(
            ::uniffi::metadata::codes::TYPE_ENUM,
        )
        .concat_str("mopro_bindings")
        .concat_str("MoproError");
}
unsafe impl ::uniffi_core::LowerError<crate::UniFfiTag> for MoproError {
    fn lower_error(obj: Self) -> ::uniffi_core::RustBuffer {
        <Self as ::uniffi_core::Lower<crate::UniFfiTag>>::lower_into_rust_buffer(obj)
    }
}
impl ::uniffi_core::ConvertError<crate::UniFfiTag> for MoproError {
    fn try_convert_unexpected_callback_error(
        e: ::uniffi_core::UnexpectedUniFFICallbackError,
    ) -> ::uniffi_core::deps::anyhow::Result<Self> {
        {
            pub trait GetConverterGeneric {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterGeneric;
            }
            impl<T> GetConverterGeneric for &T {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterGeneric {
                    ::uniffi_core::UnexpectedUniFFICallbackErrorConverterGeneric
                }
            }
            pub trait GetConverterSpecialized {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterSpecialized;
            }
            impl<T: Into<MoproError>> GetConverterSpecialized for T {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterSpecialized {
                    ::uniffi_core::UnexpectedUniFFICallbackErrorConverterSpecialized
                }
            }
            (&e).get_converter().try_convert_unexpected_callback_error(e)
        }
    }
}
#[automatically_derived]
unsafe impl ::uniffi::FfiConverter<crate::UniFfiTag> for G1 {
    type FfiType = ::uniffi_core::RustBuffer;
    fn lower(v: Self) -> ::uniffi_core::RustBuffer {
        let mut buf = ::std::vec::Vec::new();
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::write(v, &mut buf);
        ::uniffi_core::RustBuffer::from_vec(buf)
    }
    fn try_lift(buf: ::uniffi_core::RustBuffer) -> ::uniffi_core::Result<Self> {
        let vec = buf.destroy_into_vec();
        let mut buf = vec.as_slice();
        let value = <Self as ::uniffi_core::FfiConverter<
            crate::UniFfiTag,
        >>::try_read(&mut buf)?;
        match ::uniffi_core::deps::bytes::Buf::remaining(&buf) {
            0 => Ok(value),
            n => {
                return ::anyhow::__private::Err({
                    let error = ::anyhow::__private::format_err(
                        format_args!(
                            "junk data left in buffer after lifting (count: {0})", n
                        ),
                    );
                    error
                });
            }
        }
    }
    fn write(obj: Self, buf: &mut ::std::vec::Vec<u8>) {
        <String as ::uniffi::Lower<crate::UniFfiTag>>::write(obj.x, buf);
        <String as ::uniffi::Lower<crate::UniFfiTag>>::write(obj.y, buf);
    }
    fn try_read(
        buf: &mut &[::std::primitive::u8],
    ) -> ::uniffi::deps::anyhow::Result<Self> {
        Ok(Self {
            x: <String as ::uniffi::Lift<crate::UniFfiTag>>::try_read(buf)?,
            y: <String as ::uniffi::Lift<crate::UniFfiTag>>::try_read(buf)?,
        })
    }
    const TYPE_ID_META: ::uniffi::MetadataBuffer = ::uniffi::MetadataBuffer::from_code(
            ::uniffi::metadata::codes::TYPE_RECORD,
        )
        .concat_str("mopro_bindings")
        .concat_str("G1");
}
unsafe impl ::uniffi_core::Lower<crate::UniFfiTag> for G1 {
    type FfiType = <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::FfiType;
    fn lower(obj: Self) -> Self::FfiType {
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::lower(obj)
    }
    fn write(obj: Self, buf: &mut ::std::vec::Vec<u8>) {
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::write(obj, buf)
    }
}
unsafe impl ::uniffi_core::Lift<crate::UniFfiTag> for G1 {
    type FfiType = <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::FfiType;
    fn try_lift(v: Self::FfiType) -> ::uniffi_core::deps::anyhow::Result<Self> {
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::try_lift(v)
    }
    fn try_read(buf: &mut &[u8]) -> ::uniffi_core::deps::anyhow::Result<Self> {
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::try_read(buf)
    }
}
unsafe impl ::uniffi_core::LowerReturn<crate::UniFfiTag> for G1 {
    type ReturnType = <Self as ::uniffi_core::Lower<crate::UniFfiTag>>::FfiType;
    fn lower_return(
        obj: Self,
    ) -> ::uniffi_core::deps::anyhow::Result<
        Self::ReturnType,
        ::uniffi_core::RustBuffer,
    > {
        Ok(<Self as ::uniffi_core::Lower<crate::UniFfiTag>>::lower(obj))
    }
}
unsafe impl ::uniffi_core::LowerError<crate::UniFfiTag> for G1 {
    fn lower_error(obj: Self) -> ::uniffi_core::RustBuffer {
        <Self as ::uniffi_core::Lower<crate::UniFfiTag>>::lower_into_rust_buffer(obj)
    }
}
unsafe impl ::uniffi_core::LiftReturn<crate::UniFfiTag> for G1 {
    type ReturnType = <Self as ::uniffi_core::Lift<crate::UniFfiTag>>::FfiType;
    fn try_lift_successful_return(v: Self::ReturnType) -> ::uniffi_core::Result<Self> {
        <Self as ::uniffi_core::Lift<crate::UniFfiTag>>::try_lift(v)
    }
}
unsafe impl ::uniffi_core::LiftRef<crate::UniFfiTag> for G1 {
    type LiftType = Self;
}
impl ::uniffi_core::ConvertError<crate::UniFfiTag> for G1 {
    fn try_convert_unexpected_callback_error(
        e: ::uniffi_core::UnexpectedUniFFICallbackError,
    ) -> ::uniffi_core::deps::anyhow::Result<Self> {
        {
            pub trait GetConverterGeneric {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterGeneric;
            }
            impl<T> GetConverterGeneric for &T {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterGeneric {
                    ::uniffi_core::UnexpectedUniFFICallbackErrorConverterGeneric
                }
            }
            pub trait GetConverterSpecialized {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterSpecialized;
            }
            impl<T: Into<G1>> GetConverterSpecialized for T {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterSpecialized {
                    ::uniffi_core::UnexpectedUniFFICallbackErrorConverterSpecialized
                }
            }
            (&e).get_converter().try_convert_unexpected_callback_error(e)
        }
    }
}
impl ::uniffi_core::TypeId<crate::UniFfiTag> for G1 {
    const TYPE_ID_META: ::uniffi_core::MetadataBuffer = <Self as ::uniffi_core::FfiConverter<
        crate::UniFfiTag,
    >>::TYPE_ID_META;
}
#[automatically_derived]
unsafe impl ::uniffi::FfiConverter<crate::UniFfiTag> for G2 {
    type FfiType = ::uniffi_core::RustBuffer;
    fn lower(v: Self) -> ::uniffi_core::RustBuffer {
        let mut buf = ::std::vec::Vec::new();
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::write(v, &mut buf);
        ::uniffi_core::RustBuffer::from_vec(buf)
    }
    fn try_lift(buf: ::uniffi_core::RustBuffer) -> ::uniffi_core::Result<Self> {
        let vec = buf.destroy_into_vec();
        let mut buf = vec.as_slice();
        let value = <Self as ::uniffi_core::FfiConverter<
            crate::UniFfiTag,
        >>::try_read(&mut buf)?;
        match ::uniffi_core::deps::bytes::Buf::remaining(&buf) {
            0 => Ok(value),
            n => {
                return ::anyhow::__private::Err({
                    let error = ::anyhow::__private::format_err(
                        format_args!(
                            "junk data left in buffer after lifting (count: {0})", n
                        ),
                    );
                    error
                });
            }
        }
    }
    fn write(obj: Self, buf: &mut ::std::vec::Vec<u8>) {
        <std::vec::Vec<String> as ::uniffi::Lower<crate::UniFfiTag>>::write(obj.x, buf);
        <std::vec::Vec<String> as ::uniffi::Lower<crate::UniFfiTag>>::write(obj.y, buf);
    }
    fn try_read(
        buf: &mut &[::std::primitive::u8],
    ) -> ::uniffi::deps::anyhow::Result<Self> {
        Ok(Self {
            x: <std::vec::Vec<
                String,
            > as ::uniffi::Lift<crate::UniFfiTag>>::try_read(buf)?,
            y: <std::vec::Vec<
                String,
            > as ::uniffi::Lift<crate::UniFfiTag>>::try_read(buf)?,
        })
    }
    const TYPE_ID_META: ::uniffi::MetadataBuffer = ::uniffi::MetadataBuffer::from_code(
            ::uniffi::metadata::codes::TYPE_RECORD,
        )
        .concat_str("mopro_bindings")
        .concat_str("G2");
}
unsafe impl ::uniffi_core::Lower<crate::UniFfiTag> for G2 {
    type FfiType = <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::FfiType;
    fn lower(obj: Self) -> Self::FfiType {
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::lower(obj)
    }
    fn write(obj: Self, buf: &mut ::std::vec::Vec<u8>) {
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::write(obj, buf)
    }
}
unsafe impl ::uniffi_core::Lift<crate::UniFfiTag> for G2 {
    type FfiType = <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::FfiType;
    fn try_lift(v: Self::FfiType) -> ::uniffi_core::deps::anyhow::Result<Self> {
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::try_lift(v)
    }
    fn try_read(buf: &mut &[u8]) -> ::uniffi_core::deps::anyhow::Result<Self> {
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::try_read(buf)
    }
}
unsafe impl ::uniffi_core::LowerReturn<crate::UniFfiTag> for G2 {
    type ReturnType = <Self as ::uniffi_core::Lower<crate::UniFfiTag>>::FfiType;
    fn lower_return(
        obj: Self,
    ) -> ::uniffi_core::deps::anyhow::Result<
        Self::ReturnType,
        ::uniffi_core::RustBuffer,
    > {
        Ok(<Self as ::uniffi_core::Lower<crate::UniFfiTag>>::lower(obj))
    }
}
unsafe impl ::uniffi_core::LowerError<crate::UniFfiTag> for G2 {
    fn lower_error(obj: Self) -> ::uniffi_core::RustBuffer {
        <Self as ::uniffi_core::Lower<crate::UniFfiTag>>::lower_into_rust_buffer(obj)
    }
}
unsafe impl ::uniffi_core::LiftReturn<crate::UniFfiTag> for G2 {
    type ReturnType = <Self as ::uniffi_core::Lift<crate::UniFfiTag>>::FfiType;
    fn try_lift_successful_return(v: Self::ReturnType) -> ::uniffi_core::Result<Self> {
        <Self as ::uniffi_core::Lift<crate::UniFfiTag>>::try_lift(v)
    }
}
unsafe impl ::uniffi_core::LiftRef<crate::UniFfiTag> for G2 {
    type LiftType = Self;
}
impl ::uniffi_core::ConvertError<crate::UniFfiTag> for G2 {
    fn try_convert_unexpected_callback_error(
        e: ::uniffi_core::UnexpectedUniFFICallbackError,
    ) -> ::uniffi_core::deps::anyhow::Result<Self> {
        {
            pub trait GetConverterGeneric {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterGeneric;
            }
            impl<T> GetConverterGeneric for &T {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterGeneric {
                    ::uniffi_core::UnexpectedUniFFICallbackErrorConverterGeneric
                }
            }
            pub trait GetConverterSpecialized {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterSpecialized;
            }
            impl<T: Into<G2>> GetConverterSpecialized for T {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterSpecialized {
                    ::uniffi_core::UnexpectedUniFFICallbackErrorConverterSpecialized
                }
            }
            (&e).get_converter().try_convert_unexpected_callback_error(e)
        }
    }
}
impl ::uniffi_core::TypeId<crate::UniFfiTag> for G2 {
    const TYPE_ID_META: ::uniffi_core::MetadataBuffer = <Self as ::uniffi_core::FfiConverter<
        crate::UniFfiTag,
    >>::TYPE_ID_META;
}
#[automatically_derived]
unsafe impl ::uniffi::FfiConverter<crate::UniFfiTag> for GenerateProofResult {
    type FfiType = ::uniffi_core::RustBuffer;
    fn lower(v: Self) -> ::uniffi_core::RustBuffer {
        let mut buf = ::std::vec::Vec::new();
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::write(v, &mut buf);
        ::uniffi_core::RustBuffer::from_vec(buf)
    }
    fn try_lift(buf: ::uniffi_core::RustBuffer) -> ::uniffi_core::Result<Self> {
        let vec = buf.destroy_into_vec();
        let mut buf = vec.as_slice();
        let value = <Self as ::uniffi_core::FfiConverter<
            crate::UniFfiTag,
        >>::try_read(&mut buf)?;
        match ::uniffi_core::deps::bytes::Buf::remaining(&buf) {
            0 => Ok(value),
            n => {
                return ::anyhow::__private::Err({
                    let error = ::anyhow::__private::format_err(
                        format_args!(
                            "junk data left in buffer after lifting (count: {0})", n
                        ),
                    );
                    error
                });
            }
        }
    }
    fn write(obj: Self, buf: &mut ::std::vec::Vec<u8>) {
        <Vec<u8> as ::uniffi::Lower<crate::UniFfiTag>>::write(obj.proof, buf);
        <Vec<u8> as ::uniffi::Lower<crate::UniFfiTag>>::write(obj.inputs, buf);
    }
    fn try_read(
        buf: &mut &[::std::primitive::u8],
    ) -> ::uniffi::deps::anyhow::Result<Self> {
        Ok(Self {
            proof: <Vec<u8> as ::uniffi::Lift<crate::UniFfiTag>>::try_read(buf)?,
            inputs: <Vec<u8> as ::uniffi::Lift<crate::UniFfiTag>>::try_read(buf)?,
        })
    }
    const TYPE_ID_META: ::uniffi::MetadataBuffer = ::uniffi::MetadataBuffer::from_code(
            ::uniffi::metadata::codes::TYPE_RECORD,
        )
        .concat_str("mopro_bindings")
        .concat_str("GenerateProofResult");
}
unsafe impl ::uniffi_core::Lower<crate::UniFfiTag> for GenerateProofResult {
    type FfiType = <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::FfiType;
    fn lower(obj: Self) -> Self::FfiType {
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::lower(obj)
    }
    fn write(obj: Self, buf: &mut ::std::vec::Vec<u8>) {
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::write(obj, buf)
    }
}
unsafe impl ::uniffi_core::Lift<crate::UniFfiTag> for GenerateProofResult {
    type FfiType = <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::FfiType;
    fn try_lift(v: Self::FfiType) -> ::uniffi_core::deps::anyhow::Result<Self> {
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::try_lift(v)
    }
    fn try_read(buf: &mut &[u8]) -> ::uniffi_core::deps::anyhow::Result<Self> {
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::try_read(buf)
    }
}
unsafe impl ::uniffi_core::LowerReturn<crate::UniFfiTag> for GenerateProofResult {
    type ReturnType = <Self as ::uniffi_core::Lower<crate::UniFfiTag>>::FfiType;
    fn lower_return(
        obj: Self,
    ) -> ::uniffi_core::deps::anyhow::Result<
        Self::ReturnType,
        ::uniffi_core::RustBuffer,
    > {
        Ok(<Self as ::uniffi_core::Lower<crate::UniFfiTag>>::lower(obj))
    }
}
unsafe impl ::uniffi_core::LowerError<crate::UniFfiTag> for GenerateProofResult {
    fn lower_error(obj: Self) -> ::uniffi_core::RustBuffer {
        <Self as ::uniffi_core::Lower<crate::UniFfiTag>>::lower_into_rust_buffer(obj)
    }
}
unsafe impl ::uniffi_core::LiftReturn<crate::UniFfiTag> for GenerateProofResult {
    type ReturnType = <Self as ::uniffi_core::Lift<crate::UniFfiTag>>::FfiType;
    fn try_lift_successful_return(v: Self::ReturnType) -> ::uniffi_core::Result<Self> {
        <Self as ::uniffi_core::Lift<crate::UniFfiTag>>::try_lift(v)
    }
}
unsafe impl ::uniffi_core::LiftRef<crate::UniFfiTag> for GenerateProofResult {
    type LiftType = Self;
}
impl ::uniffi_core::ConvertError<crate::UniFfiTag> for GenerateProofResult {
    fn try_convert_unexpected_callback_error(
        e: ::uniffi_core::UnexpectedUniFFICallbackError,
    ) -> ::uniffi_core::deps::anyhow::Result<Self> {
        {
            pub trait GetConverterGeneric {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterGeneric;
            }
            impl<T> GetConverterGeneric for &T {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterGeneric {
                    ::uniffi_core::UnexpectedUniFFICallbackErrorConverterGeneric
                }
            }
            pub trait GetConverterSpecialized {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterSpecialized;
            }
            impl<T: Into<GenerateProofResult>> GetConverterSpecialized for T {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterSpecialized {
                    ::uniffi_core::UnexpectedUniFFICallbackErrorConverterSpecialized
                }
            }
            (&e).get_converter().try_convert_unexpected_callback_error(e)
        }
    }
}
impl ::uniffi_core::TypeId<crate::UniFfiTag> for GenerateProofResult {
    const TYPE_ID_META: ::uniffi_core::MetadataBuffer = <Self as ::uniffi_core::FfiConverter<
        crate::UniFfiTag,
    >>::TYPE_ID_META;
}
#[automatically_derived]
unsafe impl ::uniffi::FfiConverter<crate::UniFfiTag> for ProofCalldata {
    type FfiType = ::uniffi_core::RustBuffer;
    fn lower(v: Self) -> ::uniffi_core::RustBuffer {
        let mut buf = ::std::vec::Vec::new();
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::write(v, &mut buf);
        ::uniffi_core::RustBuffer::from_vec(buf)
    }
    fn try_lift(buf: ::uniffi_core::RustBuffer) -> ::uniffi_core::Result<Self> {
        let vec = buf.destroy_into_vec();
        let mut buf = vec.as_slice();
        let value = <Self as ::uniffi_core::FfiConverter<
            crate::UniFfiTag,
        >>::try_read(&mut buf)?;
        match ::uniffi_core::deps::bytes::Buf::remaining(&buf) {
            0 => Ok(value),
            n => {
                return ::anyhow::__private::Err({
                    let error = ::anyhow::__private::format_err(
                        format_args!(
                            "junk data left in buffer after lifting (count: {0})", n
                        ),
                    );
                    error
                });
            }
        }
    }
    fn write(obj: Self, buf: &mut ::std::vec::Vec<u8>) {
        <G1 as ::uniffi::Lower<crate::UniFfiTag>>::write(obj.a, buf);
        <G2 as ::uniffi::Lower<crate::UniFfiTag>>::write(obj.b, buf);
        <G1 as ::uniffi::Lower<crate::UniFfiTag>>::write(obj.c, buf);
    }
    fn try_read(
        buf: &mut &[::std::primitive::u8],
    ) -> ::uniffi::deps::anyhow::Result<Self> {
        Ok(Self {
            a: <G1 as ::uniffi::Lift<crate::UniFfiTag>>::try_read(buf)?,
            b: <G2 as ::uniffi::Lift<crate::UniFfiTag>>::try_read(buf)?,
            c: <G1 as ::uniffi::Lift<crate::UniFfiTag>>::try_read(buf)?,
        })
    }
    const TYPE_ID_META: ::uniffi::MetadataBuffer = ::uniffi::MetadataBuffer::from_code(
            ::uniffi::metadata::codes::TYPE_RECORD,
        )
        .concat_str("mopro_bindings")
        .concat_str("ProofCalldata");
}
unsafe impl ::uniffi_core::Lower<crate::UniFfiTag> for ProofCalldata {
    type FfiType = <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::FfiType;
    fn lower(obj: Self) -> Self::FfiType {
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::lower(obj)
    }
    fn write(obj: Self, buf: &mut ::std::vec::Vec<u8>) {
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::write(obj, buf)
    }
}
unsafe impl ::uniffi_core::Lift<crate::UniFfiTag> for ProofCalldata {
    type FfiType = <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::FfiType;
    fn try_lift(v: Self::FfiType) -> ::uniffi_core::deps::anyhow::Result<Self> {
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::try_lift(v)
    }
    fn try_read(buf: &mut &[u8]) -> ::uniffi_core::deps::anyhow::Result<Self> {
        <Self as ::uniffi_core::FfiConverter<crate::UniFfiTag>>::try_read(buf)
    }
}
unsafe impl ::uniffi_core::LowerReturn<crate::UniFfiTag> for ProofCalldata {
    type ReturnType = <Self as ::uniffi_core::Lower<crate::UniFfiTag>>::FfiType;
    fn lower_return(
        obj: Self,
    ) -> ::uniffi_core::deps::anyhow::Result<
        Self::ReturnType,
        ::uniffi_core::RustBuffer,
    > {
        Ok(<Self as ::uniffi_core::Lower<crate::UniFfiTag>>::lower(obj))
    }
}
unsafe impl ::uniffi_core::LowerError<crate::UniFfiTag> for ProofCalldata {
    fn lower_error(obj: Self) -> ::uniffi_core::RustBuffer {
        <Self as ::uniffi_core::Lower<crate::UniFfiTag>>::lower_into_rust_buffer(obj)
    }
}
unsafe impl ::uniffi_core::LiftReturn<crate::UniFfiTag> for ProofCalldata {
    type ReturnType = <Self as ::uniffi_core::Lift<crate::UniFfiTag>>::FfiType;
    fn try_lift_successful_return(v: Self::ReturnType) -> ::uniffi_core::Result<Self> {
        <Self as ::uniffi_core::Lift<crate::UniFfiTag>>::try_lift(v)
    }
}
unsafe impl ::uniffi_core::LiftRef<crate::UniFfiTag> for ProofCalldata {
    type LiftType = Self;
}
impl ::uniffi_core::ConvertError<crate::UniFfiTag> for ProofCalldata {
    fn try_convert_unexpected_callback_error(
        e: ::uniffi_core::UnexpectedUniFFICallbackError,
    ) -> ::uniffi_core::deps::anyhow::Result<Self> {
        {
            pub trait GetConverterGeneric {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterGeneric;
            }
            impl<T> GetConverterGeneric for &T {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterGeneric {
                    ::uniffi_core::UnexpectedUniFFICallbackErrorConverterGeneric
                }
            }
            pub trait GetConverterSpecialized {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterSpecialized;
            }
            impl<T: Into<ProofCalldata>> GetConverterSpecialized for T {
                fn get_converter(
                    &self,
                ) -> ::uniffi_core::UnexpectedUniFFICallbackErrorConverterSpecialized {
                    ::uniffi_core::UnexpectedUniFFICallbackErrorConverterSpecialized
                }
            }
            (&e).get_converter().try_convert_unexpected_callback_error(e)
        }
    }
}
impl ::uniffi_core::TypeId<crate::UniFfiTag> for ProofCalldata {
    const TYPE_ID_META: ::uniffi_core::MetadataBuffer = <Self as ::uniffi_core::FfiConverter<
        crate::UniFfiTag,
    >>::TYPE_ID_META;
}
#[doc(hidden)]
#[no_mangle]
extern "C" fn uniffi_mopro_bindings_fn_func_generate_circom_proof(
    zkey_path: <String as ::uniffi::Lift<crate::UniFfiTag>>::FfiType,
    circuit_inputs: <std::collections::HashMap<
        String,
        std::vec::Vec<String>,
    > as ::uniffi::Lift<crate::UniFfiTag>>::FfiType,
    call_status: &mut ::uniffi::RustCallStatus,
) -> <::std::result::Result<
    GenerateProofResult,
    MoproError,
> as ::uniffi::LowerReturn<crate::UniFfiTag>>::ReturnType {
    {
        let lvl = ::log::Level::Debug;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("generate_circom_proof"),
                lvl,
                &("mopro_bindings", "mopro_bindings", ::log::__private_api::loc()),
                (),
            );
        }
    };
    let uniffi_lift_args = move || Ok((
        match <String as ::uniffi::Lift<crate::UniFfiTag>>::try_lift(zkey_path) {
            Ok(v) => v,
            Err(e) => return Err(("zkey_path", e)),
        },
        match <std::collections::HashMap<
            String,
            std::vec::Vec<String>,
        > as ::uniffi::Lift<crate::UniFfiTag>>::try_lift(circuit_inputs) {
            Ok(v) => v,
            Err(e) => return Err(("circuit_inputs", e)),
        },
    ));
    ::uniffi::rust_call(
        call_status,
        || {
            <::std::result::Result<
                GenerateProofResult,
                MoproError,
            > as ::uniffi::LowerReturn<
                crate::UniFfiTag,
            >>::lower_return(
                match uniffi_lift_args() {
                    Ok(uniffi_args) => {
                        let uniffi_result = generate_circom_proof(
                            uniffi_args.0,
                            uniffi_args.1,
                        );
                        uniffi_result.map_err(::std::convert::Into::into)
                    }
                    Err((arg_name, anyhow_error)) => {
                        <::std::result::Result<
                            GenerateProofResult,
                            MoproError,
                        > as ::uniffi::LowerReturn<
                            crate::UniFfiTag,
                        >>::handle_failed_lift(arg_name, anyhow_error)
                    }
                },
            )
        },
    )
}
#[doc(hidden)]
#[no_mangle]
extern "C" fn uniffi_mopro_bindings_fn_func_generate_halo2_proof(
    srs_path: <String as ::uniffi::Lift<crate::UniFfiTag>>::FfiType,
    pk_path: <String as ::uniffi::Lift<crate::UniFfiTag>>::FfiType,
    circuit_inputs: <std::collections::HashMap<
        String,
        std::vec::Vec<String>,
    > as ::uniffi::Lift<crate::UniFfiTag>>::FfiType,
    call_status: &mut ::uniffi::RustCallStatus,
) -> <::std::result::Result<
    GenerateProofResult,
    MoproError,
> as ::uniffi::LowerReturn<crate::UniFfiTag>>::ReturnType {
    {
        let lvl = ::log::Level::Debug;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("generate_halo2_proof"),
                lvl,
                &("mopro_bindings", "mopro_bindings", ::log::__private_api::loc()),
                (),
            );
        }
    };
    let uniffi_lift_args = move || Ok((
        match <String as ::uniffi::Lift<crate::UniFfiTag>>::try_lift(srs_path) {
            Ok(v) => v,
            Err(e) => return Err(("srs_path", e)),
        },
        match <String as ::uniffi::Lift<crate::UniFfiTag>>::try_lift(pk_path) {
            Ok(v) => v,
            Err(e) => return Err(("pk_path", e)),
        },
        match <std::collections::HashMap<
            String,
            std::vec::Vec<String>,
        > as ::uniffi::Lift<crate::UniFfiTag>>::try_lift(circuit_inputs) {
            Ok(v) => v,
            Err(e) => return Err(("circuit_inputs", e)),
        },
    ));
    ::uniffi::rust_call(
        call_status,
        || {
            <::std::result::Result<
                GenerateProofResult,
                MoproError,
            > as ::uniffi::LowerReturn<
                crate::UniFfiTag,
            >>::lower_return(
                match uniffi_lift_args() {
                    Ok(uniffi_args) => {
                        let uniffi_result = generate_halo2_proof(
                            uniffi_args.0,
                            uniffi_args.1,
                            uniffi_args.2,
                        );
                        uniffi_result.map_err(::std::convert::Into::into)
                    }
                    Err((arg_name, anyhow_error)) => {
                        <::std::result::Result<
                            GenerateProofResult,
                            MoproError,
                        > as ::uniffi::LowerReturn<
                            crate::UniFfiTag,
                        >>::handle_failed_lift(arg_name, anyhow_error)
                    }
                },
            )
        },
    )
}
#[doc(hidden)]
#[no_mangle]
extern "C" fn uniffi_mopro_bindings_fn_func_to_ethereum_inputs(
    inputs: <Vec<u8> as ::uniffi::Lift<crate::UniFfiTag>>::FfiType,
    call_status: &mut ::uniffi::RustCallStatus,
) -> <std::vec::Vec<String> as ::uniffi::LowerReturn<crate::UniFfiTag>>::ReturnType {
    {
        let lvl = ::log::Level::Debug;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("to_ethereum_inputs"),
                lvl,
                &("mopro_bindings", "mopro_bindings", ::log::__private_api::loc()),
                (),
            );
        }
    };
    let uniffi_lift_args = move || Ok((
        match <Vec<u8> as ::uniffi::Lift<crate::UniFfiTag>>::try_lift(inputs) {
            Ok(v) => v,
            Err(e) => return Err(("inputs", e)),
        },
    ));
    ::uniffi::rust_call(
        call_status,
        || {
            <std::vec::Vec<
                String,
            > as ::uniffi::LowerReturn<
                crate::UniFfiTag,
            >>::lower_return(
                match uniffi_lift_args() {
                    Ok(uniffi_args) => {
                        let uniffi_result = to_ethereum_inputs(uniffi_args.0);
                        uniffi_result
                    }
                    Err((arg_name, anyhow_error)) => {
                        <std::vec::Vec<
                            String,
                        > as ::uniffi::LowerReturn<
                            crate::UniFfiTag,
                        >>::handle_failed_lift(arg_name, anyhow_error)
                    }
                },
            )
        },
    )
}
#[doc(hidden)]
#[no_mangle]
extern "C" fn uniffi_mopro_bindings_fn_func_to_ethereum_proof(
    proof: <Vec<u8> as ::uniffi::Lift<crate::UniFfiTag>>::FfiType,
    call_status: &mut ::uniffi::RustCallStatus,
) -> <ProofCalldata as ::uniffi::LowerReturn<crate::UniFfiTag>>::ReturnType {
    {
        let lvl = ::log::Level::Debug;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("to_ethereum_proof"),
                lvl,
                &("mopro_bindings", "mopro_bindings", ::log::__private_api::loc()),
                (),
            );
        }
    };
    let uniffi_lift_args = move || Ok((
        match <Vec<u8> as ::uniffi::Lift<crate::UniFfiTag>>::try_lift(proof) {
            Ok(v) => v,
            Err(e) => return Err(("proof", e)),
        },
    ));
    ::uniffi::rust_call(
        call_status,
        || {
            <ProofCalldata as ::uniffi::LowerReturn<
                crate::UniFfiTag,
            >>::lower_return(
                match uniffi_lift_args() {
                    Ok(uniffi_args) => {
                        let uniffi_result = to_ethereum_proof(uniffi_args.0);
                        uniffi_result
                    }
                    Err((arg_name, anyhow_error)) => {
                        <ProofCalldata as ::uniffi::LowerReturn<
                            crate::UniFfiTag,
                        >>::handle_failed_lift(arg_name, anyhow_error)
                    }
                },
            )
        },
    )
}
#[doc(hidden)]
#[no_mangle]
extern "C" fn uniffi_mopro_bindings_fn_func_verify_circom_proof(
    zkey_path: <String as ::uniffi::Lift<crate::UniFfiTag>>::FfiType,
    proof: <Vec<u8> as ::uniffi::Lift<crate::UniFfiTag>>::FfiType,
    public_input: <Vec<u8> as ::uniffi::Lift<crate::UniFfiTag>>::FfiType,
    call_status: &mut ::uniffi::RustCallStatus,
) -> <::std::result::Result<
    bool,
    MoproError,
> as ::uniffi::LowerReturn<crate::UniFfiTag>>::ReturnType {
    {
        let lvl = ::log::Level::Debug;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("verify_circom_proof"),
                lvl,
                &("mopro_bindings", "mopro_bindings", ::log::__private_api::loc()),
                (),
            );
        }
    };
    let uniffi_lift_args = move || Ok((
        match <String as ::uniffi::Lift<crate::UniFfiTag>>::try_lift(zkey_path) {
            Ok(v) => v,
            Err(e) => return Err(("zkey_path", e)),
        },
        match <Vec<u8> as ::uniffi::Lift<crate::UniFfiTag>>::try_lift(proof) {
            Ok(v) => v,
            Err(e) => return Err(("proof", e)),
        },
        match <Vec<u8> as ::uniffi::Lift<crate::UniFfiTag>>::try_lift(public_input) {
            Ok(v) => v,
            Err(e) => return Err(("public_input", e)),
        },
    ));
    ::uniffi::rust_call(
        call_status,
        || {
            <::std::result::Result<
                bool,
                MoproError,
            > as ::uniffi::LowerReturn<
                crate::UniFfiTag,
            >>::lower_return(
                match uniffi_lift_args() {
                    Ok(uniffi_args) => {
                        let uniffi_result = verify_circom_proof(
                            uniffi_args.0,
                            uniffi_args.1,
                            uniffi_args.2,
                        );
                        uniffi_result.map_err(::std::convert::Into::into)
                    }
                    Err((arg_name, anyhow_error)) => {
                        <::std::result::Result<
                            bool,
                            MoproError,
                        > as ::uniffi::LowerReturn<
                            crate::UniFfiTag,
                        >>::handle_failed_lift(arg_name, anyhow_error)
                    }
                },
            )
        },
    )
}
#[doc(hidden)]
#[no_mangle]
extern "C" fn uniffi_mopro_bindings_fn_func_verify_halo2_proof(
    srs_path: <String as ::uniffi::Lift<crate::UniFfiTag>>::FfiType,
    vk_path: <String as ::uniffi::Lift<crate::UniFfiTag>>::FfiType,
    proof: <Vec<u8> as ::uniffi::Lift<crate::UniFfiTag>>::FfiType,
    public_input: <Vec<u8> as ::uniffi::Lift<crate::UniFfiTag>>::FfiType,
    call_status: &mut ::uniffi::RustCallStatus,
) -> <::std::result::Result<
    bool,
    MoproError,
> as ::uniffi::LowerReturn<crate::UniFfiTag>>::ReturnType {
    {
        let lvl = ::log::Level::Debug;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("verify_halo2_proof"),
                lvl,
                &("mopro_bindings", "mopro_bindings", ::log::__private_api::loc()),
                (),
            );
        }
    };
    let uniffi_lift_args = move || Ok((
        match <String as ::uniffi::Lift<crate::UniFfiTag>>::try_lift(srs_path) {
            Ok(v) => v,
            Err(e) => return Err(("srs_path", e)),
        },
        match <String as ::uniffi::Lift<crate::UniFfiTag>>::try_lift(vk_path) {
            Ok(v) => v,
            Err(e) => return Err(("vk_path", e)),
        },
        match <Vec<u8> as ::uniffi::Lift<crate::UniFfiTag>>::try_lift(proof) {
            Ok(v) => v,
            Err(e) => return Err(("proof", e)),
        },
        match <Vec<u8> as ::uniffi::Lift<crate::UniFfiTag>>::try_lift(public_input) {
            Ok(v) => v,
            Err(e) => return Err(("public_input", e)),
        },
    ));
    ::uniffi::rust_call(
        call_status,
        || {
            <::std::result::Result<
                bool,
                MoproError,
            > as ::uniffi::LowerReturn<
                crate::UniFfiTag,
            >>::lower_return(
                match uniffi_lift_args() {
                    Ok(uniffi_args) => {
                        let uniffi_result = verify_halo2_proof(
                            uniffi_args.0,
                            uniffi_args.1,
                            uniffi_args.2,
                            uniffi_args.3,
                        );
                        uniffi_result.map_err(::std::convert::Into::into)
                    }
                    Err((arg_name, anyhow_error)) => {
                        <::std::result::Result<
                            bool,
                            MoproError,
                        > as ::uniffi::LowerReturn<
                            crate::UniFfiTag,
                        >>::handle_failed_lift(arg_name, anyhow_error)
                    }
                },
            )
        },
    )
}
#[no_mangle]
#[doc(hidden)]
pub extern "C" fn uniffi_mopro_bindings_checksum_func_generate_circom_proof() -> u16 {
    54365
}
#[no_mangle]
#[doc(hidden)]
pub extern "C" fn uniffi_mopro_bindings_checksum_func_generate_halo2_proof() -> u16 {
    3963
}
#[no_mangle]
#[doc(hidden)]
pub extern "C" fn uniffi_mopro_bindings_checksum_func_to_ethereum_inputs() -> u16 {
    64747
}
#[no_mangle]
#[doc(hidden)]
pub extern "C" fn uniffi_mopro_bindings_checksum_func_to_ethereum_proof() -> u16 {
    64531
}
#[no_mangle]
#[doc(hidden)]
pub extern "C" fn uniffi_mopro_bindings_checksum_func_verify_circom_proof() -> u16 {
    46591
}
#[no_mangle]
#[doc(hidden)]
pub extern "C" fn uniffi_mopro_bindings_checksum_func_verify_halo2_proof() -> u16 {
    56128
}
extern "C" {
    pub fn multiplier2Instantiate(
        i: *mut std::ffi::c_void,
        resolveImports: *mut std::ffi::c_void,
    );
    pub fn multiplier2FreeInstance(i: *mut std::ffi::c_void);
    pub fn multiplier2_getFieldNumLen32(i: *mut std::ffi::c_void) -> u32;
    pub fn multiplier2_getRawPrime(i: *mut std::ffi::c_void);
    pub fn multiplier2_getWitnessSize(i: *mut std::ffi::c_void) -> u32;
    pub fn multiplier2_readSharedRWMemory(i: *mut std::ffi::c_void, l0: u32) -> u32;
    pub fn multiplier2_writeSharedRWMemory(i: *mut std::ffi::c_void, l0: u32, l1: u32);
    pub fn multiplier2_setInputSignal(
        i: *mut std::ffi::c_void,
        l0: u32,
        l1: u32,
        l2: u32,
    );
    pub fn multiplier2_getWitness(i: *mut std::ffi::c_void, l0: u32);
    pub fn multiplier2_init(i: *mut std::ffi::c_void, l0: u32);
}
pub fn multiplier2_witness<I: IntoIterator<Item = (String, Vec<num_bigint::BigInt>)>>(
    inputs: I,
) -> Vec<num_bigint::BigInt> {
    unsafe {
        let instance = rust_witness::c_init();
        let resolver = rust_witness::c_resolver();
        multiplier2Instantiate(instance, resolver);
        let n32 = multiplier2_getFieldNumLen32(instance);
        multiplier2_getRawPrime(instance);
        let mut arr = ::alloc::vec::from_elem(0, n32 as usize);
        for x in 0..n32 {
            let res = multiplier2_readSharedRWMemory(instance, x);
            arr[(n32 as usize) - (x as usize) - 1] = res;
        }
        multiplier2_init(instance, 0);
        for (name, values) in inputs.into_iter() {
            let (msb, lsb) = rust_witness::fnv(&name);
            for (i, value) in values.into_iter().enumerate() {
                let f_arr = rust_witness::to_array32(&value, n32 as usize);
                for j in 0..n32 {
                    multiplier2_writeSharedRWMemory(
                        instance,
                        j,
                        f_arr[(n32 as usize) - 1 - (j as usize)],
                    );
                }
                multiplier2_setInputSignal(instance, msb, lsb, i as u32);
            }
        }
        let mut w = Vec::new();
        let witness_size = multiplier2_getWitnessSize(instance);
        for i in 0..witness_size {
            multiplier2_getWitness(instance, i);
            let mut arr = ::alloc::vec::from_elem(0, n32 as usize);
            for j in 0..n32 {
                arr[(n32 as usize) - 1
                    - (j as usize)] = multiplier2_readSharedRWMemory(instance, j);
            }
            w.push(rust_witness::from_array32(arr));
        }
        multiplier2FreeInstance(instance);
        rust_witness::c_cleanup(instance);
        w
    }
}
extern "C" {
    pub fn multiplier2blsInstantiate(
        i: *mut std::ffi::c_void,
        resolveImports: *mut std::ffi::c_void,
    );
    pub fn multiplier2blsFreeInstance(i: *mut std::ffi::c_void);
    pub fn multiplier2bls_getFieldNumLen32(i: *mut std::ffi::c_void) -> u32;
    pub fn multiplier2bls_getRawPrime(i: *mut std::ffi::c_void);
    pub fn multiplier2bls_getWitnessSize(i: *mut std::ffi::c_void) -> u32;
    pub fn multiplier2bls_readSharedRWMemory(i: *mut std::ffi::c_void, l0: u32) -> u32;
    pub fn multiplier2bls_writeSharedRWMemory(
        i: *mut std::ffi::c_void,
        l0: u32,
        l1: u32,
    );
    pub fn multiplier2bls_setInputSignal(
        i: *mut std::ffi::c_void,
        l0: u32,
        l1: u32,
        l2: u32,
    );
    pub fn multiplier2bls_getWitness(i: *mut std::ffi::c_void, l0: u32);
    pub fn multiplier2bls_init(i: *mut std::ffi::c_void, l0: u32);
}
pub fn multiplier2bls_witness<I: IntoIterator<Item = (String, Vec<num_bigint::BigInt>)>>(
    inputs: I,
) -> Vec<num_bigint::BigInt> {
    unsafe {
        let instance = rust_witness::c_init();
        let resolver = rust_witness::c_resolver();
        multiplier2blsInstantiate(instance, resolver);
        let n32 = multiplier2bls_getFieldNumLen32(instance);
        multiplier2bls_getRawPrime(instance);
        let mut arr = ::alloc::vec::from_elem(0, n32 as usize);
        for x in 0..n32 {
            let res = multiplier2bls_readSharedRWMemory(instance, x);
            arr[(n32 as usize) - (x as usize) - 1] = res;
        }
        multiplier2bls_init(instance, 0);
        for (name, values) in inputs.into_iter() {
            let (msb, lsb) = rust_witness::fnv(&name);
            for (i, value) in values.into_iter().enumerate() {
                let f_arr = rust_witness::to_array32(&value, n32 as usize);
                for j in 0..n32 {
                    multiplier2bls_writeSharedRWMemory(
                        instance,
                        j,
                        f_arr[(n32 as usize) - 1 - (j as usize)],
                    );
                }
                multiplier2bls_setInputSignal(instance, msb, lsb, i as u32);
            }
        }
        let mut w = Vec::new();
        let witness_size = multiplier2bls_getWitnessSize(instance);
        for i in 0..witness_size {
            multiplier2bls_getWitness(instance, i);
            let mut arr = ::alloc::vec::from_elem(0, n32 as usize);
            for j in 0..n32 {
                arr[(n32 as usize) - 1
                    - (j as usize)] = multiplier2bls_readSharedRWMemory(instance, j);
            }
            w.push(rust_witness::from_array32(arr));
        }
        multiplier2blsFreeInstance(instance);
        rust_witness::c_cleanup(instance);
        w
    }
}
extern "C" {
    pub fn keccak256256testInstantiate(
        i: *mut std::ffi::c_void,
        resolveImports: *mut std::ffi::c_void,
    );
    pub fn keccak256256testFreeInstance(i: *mut std::ffi::c_void);
    pub fn keccak256256test_getFieldNumLen32(i: *mut std::ffi::c_void) -> u32;
    pub fn keccak256256test_getRawPrime(i: *mut std::ffi::c_void);
    pub fn keccak256256test_getWitnessSize(i: *mut std::ffi::c_void) -> u32;
    pub fn keccak256256test_readSharedRWMemory(i: *mut std::ffi::c_void, l0: u32) -> u32;
    pub fn keccak256256test_writeSharedRWMemory(
        i: *mut std::ffi::c_void,
        l0: u32,
        l1: u32,
    );
    pub fn keccak256256test_setInputSignal(
        i: *mut std::ffi::c_void,
        l0: u32,
        l1: u32,
        l2: u32,
    );
    pub fn keccak256256test_getWitness(i: *mut std::ffi::c_void, l0: u32);
    pub fn keccak256256test_init(i: *mut std::ffi::c_void, l0: u32);
}
pub fn keccak256256test_witness<
    I: IntoIterator<Item = (String, Vec<num_bigint::BigInt>)>,
>(inputs: I) -> Vec<num_bigint::BigInt> {
    unsafe {
        let instance = rust_witness::c_init();
        let resolver = rust_witness::c_resolver();
        keccak256256testInstantiate(instance, resolver);
        let n32 = keccak256256test_getFieldNumLen32(instance);
        keccak256256test_getRawPrime(instance);
        let mut arr = ::alloc::vec::from_elem(0, n32 as usize);
        for x in 0..n32 {
            let res = keccak256256test_readSharedRWMemory(instance, x);
            arr[(n32 as usize) - (x as usize) - 1] = res;
        }
        keccak256256test_init(instance, 0);
        for (name, values) in inputs.into_iter() {
            let (msb, lsb) = rust_witness::fnv(&name);
            for (i, value) in values.into_iter().enumerate() {
                let f_arr = rust_witness::to_array32(&value, n32 as usize);
                for j in 0..n32 {
                    keccak256256test_writeSharedRWMemory(
                        instance,
                        j,
                        f_arr[(n32 as usize) - 1 - (j as usize)],
                    );
                }
                keccak256256test_setInputSignal(instance, msb, lsb, i as u32);
            }
        }
        let mut w = Vec::new();
        let witness_size = keccak256256test_getWitnessSize(instance);
        for i in 0..witness_size {
            keccak256256test_getWitness(instance, i);
            let mut arr = ::alloc::vec::from_elem(0, n32 as usize);
            for j in 0..n32 {
                arr[(n32 as usize) - 1
                    - (j as usize)] = keccak256256test_readSharedRWMemory(instance, j);
            }
            w.push(rust_witness::from_array32(arr));
        }
        keccak256256testFreeInstance(instance);
        rust_witness::c_cleanup(instance);
        w
    }
}
extern "C" {
    pub fn hashbenchblsInstantiate(
        i: *mut std::ffi::c_void,
        resolveImports: *mut std::ffi::c_void,
    );
    pub fn hashbenchblsFreeInstance(i: *mut std::ffi::c_void);
    pub fn hashbenchbls_getFieldNumLen32(i: *mut std::ffi::c_void) -> u32;
    pub fn hashbenchbls_getRawPrime(i: *mut std::ffi::c_void);
    pub fn hashbenchbls_getWitnessSize(i: *mut std::ffi::c_void) -> u32;
    pub fn hashbenchbls_readSharedRWMemory(i: *mut std::ffi::c_void, l0: u32) -> u32;
    pub fn hashbenchbls_writeSharedRWMemory(i: *mut std::ffi::c_void, l0: u32, l1: u32);
    pub fn hashbenchbls_setInputSignal(
        i: *mut std::ffi::c_void,
        l0: u32,
        l1: u32,
        l2: u32,
    );
    pub fn hashbenchbls_getWitness(i: *mut std::ffi::c_void, l0: u32);
    pub fn hashbenchbls_init(i: *mut std::ffi::c_void, l0: u32);
}
pub fn hashbenchbls_witness<I: IntoIterator<Item = (String, Vec<num_bigint::BigInt>)>>(
    inputs: I,
) -> Vec<num_bigint::BigInt> {
    unsafe {
        let instance = rust_witness::c_init();
        let resolver = rust_witness::c_resolver();
        hashbenchblsInstantiate(instance, resolver);
        let n32 = hashbenchbls_getFieldNumLen32(instance);
        hashbenchbls_getRawPrime(instance);
        let mut arr = ::alloc::vec::from_elem(0, n32 as usize);
        for x in 0..n32 {
            let res = hashbenchbls_readSharedRWMemory(instance, x);
            arr[(n32 as usize) - (x as usize) - 1] = res;
        }
        hashbenchbls_init(instance, 0);
        for (name, values) in inputs.into_iter() {
            let (msb, lsb) = rust_witness::fnv(&name);
            for (i, value) in values.into_iter().enumerate() {
                let f_arr = rust_witness::to_array32(&value, n32 as usize);
                for j in 0..n32 {
                    hashbenchbls_writeSharedRWMemory(
                        instance,
                        j,
                        f_arr[(n32 as usize) - 1 - (j as usize)],
                    );
                }
                hashbenchbls_setInputSignal(instance, msb, lsb, i as u32);
            }
        }
        let mut w = Vec::new();
        let witness_size = hashbenchbls_getWitnessSize(instance);
        for i in 0..witness_size {
            hashbenchbls_getWitness(instance, i);
            let mut arr = ::alloc::vec::from_elem(0, n32 as usize);
            for j in 0..n32 {
                arr[(n32 as usize) - 1
                    - (j as usize)] = hashbenchbls_readSharedRWMemory(instance, j);
            }
            w.push(rust_witness::from_array32(arr));
        }
        hashbenchblsFreeInstance(instance);
        rust_witness::c_cleanup(instance);
        w
    }
}
fn get_circom_wtns_fn(
    circuit: &str,
) -> Result<mopro_ffi::WtnsFn, mopro_ffi::MoproError> {
    match circuit {
        "multiplier2_final.zkey" => Ok(multiplier2_witness),
        "multiplier2_bls_final.zkey" => Ok(multiplier2bls_witness),
        "keccak256_256_test_final.zkey" => Ok(keccak256256test_witness),
        _ => {
            Err(
                mopro_ffi::MoproError::CircomError({
                    let res = ::alloc::fmt::format(
                        format_args!("Unknown ZKEY: {0}", circuit),
                    );
                    res
                }),
            )
        }
    }
}
fn get_halo2_proving_circuit(
    circuit_pk: &str,
) -> Result<mopro_ffi::Halo2ProveFn, mopro_ffi::MoproError> {
    match circuit_pk {
        "fibonacci_pk.bin" => Ok(halo2_fibonacci::prove),
        "hyperplonk_fibonacci_pk.bin" => Ok(hyperplonk_fibonacci::prove),
        "gemini_fibonacci_pk.bin" => Ok(gemini_fibonacci::prove),
        "keccak256_pk.bin" => Ok(halo2_keccak_256::prove),
        _ => {
            Err(
                mopro_ffi::MoproError::Halo2Error({
                    let res = ::alloc::fmt::format(
                        format_args!("Unknown proving key: {0}", circuit_pk),
                    );
                    res
                }),
            )
        }
    }
}
fn get_halo2_verifying_circuit(
    circuit_vk: &str,
) -> Result<mopro_ffi::Halo2VerifyFn, mopro_ffi::MoproError> {
    match circuit_vk {
        "fibonacci_vk.bin" => Ok(halo2_fibonacci::verify),
        "hyperplonk_fibonacci_vk.bin" => Ok(hyperplonk_fibonacci::verify),
        "gemini_fibonacci_vk.bin" => Ok(gemini_fibonacci::verify),
        "keccak256_vk.bin" => Ok(halo2_keccak_256::verify),
        _ => {
            Err(
                mopro_ffi::MoproError::Halo2Error({
                    let res = ::alloc::fmt::format(
                        format_args!("Unknown verifying key: {0}", circuit_vk),
                    );
                    res
                }),
            )
        }
    }
}
