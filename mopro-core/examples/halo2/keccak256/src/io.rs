use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use halo2_proofs::halo2curves::bn256::{Bn256, G1Affine};
use halo2_proofs::plonk::ProvingKey;
use halo2_proofs::poly::commitment::Params;
use halo2_proofs::poly::kzg::commitment::ParamsKZG;
use halo2_proofs::SerdeFormat::RawBytes;

fn with_writer<E>(path: &Path, f: impl FnOnce(&mut BufWriter<File>) -> Result<(), E>)
where
    E: fmt::Debug,
{
    let file = File::create(path).expect("Unable to create file");
    let mut writer = BufWriter::new(file);
    f(&mut writer).expect("Unable to write to file");
    writer.flush().expect("Unable to flush file");
}

/// Write SRS to file.
pub fn write_srs(srs: &ParamsKZG<Bn256>, path: &Path) {
    with_writer(path, |writer| srs.write(writer));
}

/// Write proving key and verification key to file.
pub fn write_keys(pk: &ProvingKey<G1Affine>, pk_path: &Path, vk_path: &Path) {
    with_writer(pk_path, |writer| pk.write(writer, RawBytes));
    with_writer(vk_path, |writer| pk.get_vk().write(writer, RawBytes));
}
