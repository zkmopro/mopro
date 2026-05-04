use noir::barretenberg::{
    srs::{localsrs::LocalSrs, netsrs::NetSrs, setup_srs_from_bytecode, Srs},
    utils::{compute_subgroup_size, get_circuit_size},
    verify::{get_ultra_honk_keccak_verification_key, get_ultra_honk_verification_key},
};
use std::{env, fs};

const ULTRA_HONK_SRS_MULTIPLIER: u32 = 8;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!(
            "usage: {} <circuit.json> <out.srs> <out_keccak.vk> <out_poseidon.vk>",
            args[0]
        );
        std::process::exit(2);
    }
    let circuit_path = &args[1];
    let srs_out = &args[2];
    let vk_keccak_out = &args[3];
    let vk_poseidon_out = &args[4];

    let circuit_txt = fs::read_to_string(circuit_path).expect("read circuit json");
    let circuit: serde_json::Value =
        serde_json::from_str(&circuit_txt).expect("parse circuit json");
    let bytecode = circuit["bytecode"]
        .as_str()
        .expect("bytecode field missing");

    // Ultra-Honk needs ~8x the dyadic gate count in SRS points for
    // witness/permutation/lookup polynomials. Mirror `setup_srs_from_bytecode`'s
    // sizing or the prove path will fail with an out-of-range slice on load.
    let circuit_size = get_circuit_size(bytecode, false);
    let subgroup_size = compute_subgroup_size(circuit_size * ULTRA_HONK_SRS_MULTIPLIER);
    let num_points = subgroup_size + 1;
    println!(
        "circuit_size={} subgroup_size={} num_points={}",
        circuit_size, subgroup_size, num_points
    );

    let net_srs: Srs = NetSrs::new(num_points).to_srs();
    LocalSrs(net_srs).save(Some(srs_out.as_str()));
    println!("wrote SRS to {}", srs_out);

    setup_srs_from_bytecode(bytecode, Some(srs_out.as_str()), false).expect("setup srs");

    let vk_keccak = get_ultra_honk_keccak_verification_key(bytecode, false, false)
        .expect("compute keccak vk");
    fs::write(vk_keccak_out, &vk_keccak).expect("write keccak vk");
    println!(
        "wrote Keccak VK ({} bytes) to {}",
        vk_keccak.len(),
        vk_keccak_out
    );

    let vk_poseidon =
        get_ultra_honk_verification_key(bytecode, false).expect("compute poseidon vk");
    fs::write(vk_poseidon_out, &vk_poseidon).expect("write poseidon vk");
    println!(
        "wrote Poseidon VK ({} bytes) to {}",
        vk_poseidon.len(),
        vk_poseidon_out
    );
}
