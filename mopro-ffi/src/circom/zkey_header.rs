use ark_serialize::Read;
use ark_std::Zero;
use num_bigint::BigUint;
use std::fs::File;

pub struct ZkeyHeaderReader {
    zkey_path: std::path::PathBuf,
    offset: usize,
    data: Option<Vec<u8>>,
    pub n8q: u32,
    pub n8r: u32,
    n_public: u32,
    n_vars: u32,
    domain_size: u32,
    pub q: BigUint,
    pub r: BigUint,
}

impl ZkeyHeaderReader {
    pub fn new(zkey_path: &str) -> Self {
        ZkeyHeaderReader {
            zkey_path: std::path::PathBuf::from(zkey_path),
            offset: 0,
            data: None,
            n8q: 0,
            n8r: 0,
            n_public: 0,
            n_vars: 0,
            domain_size: 0,
            q: BigUint::zero(),
            r: BigUint::zero(),
        }
    }

    pub fn read(&mut self) {
        let mut file = File::open(self.zkey_path.clone()).unwrap();
        let mut zkey_bytes = vec![0; 512];
        // file.read_to_end(&mut zkey_bytes).unwrap();
        file.read_exact(&mut zkey_bytes).unwrap();
        self.data = Some(zkey_bytes);
        self.parse();
    }

    fn parse(&mut self) {
        let _magic = self.read_u32();
        let _version = self.read_u32();
        let num_sections = self.read_u32();
        for i in 0..num_sections {
            if i > 1 {
                return;
            }
            let section_id = self.read_u32();
            let section_len = self.read_u64();
            self.read_section(section_id, section_len);
        }
    }

    fn read_section(&mut self, section_n: u32, section_len: u64) {
        match section_n {
            1 => self.read_header(section_len),
            2 => self.read_groth16_header(section_len),
            // 3 => self.read_ic(section_len),
            // 4 => self.read_ccoefs(section_len),
            // 5 => self.read_a(section_len),
            // 6 => self.read_b1(section_len),
            // 7 => self.read_b2(section_len),
            // 8 => self.read_c(section_len),
            // 9 => self.read_h(section_len),
            // 10 => (|| {})(), // ignore reading the contributions
            _ => panic!("unknown section index"),
        }
    }

    fn read_u32(&mut self) -> u32 {
        let v = u32::from_le_bytes(
            self.data.as_ref().unwrap()[self.offset..(self.offset + 4)]
                .try_into()
                .unwrap(),
        );
        self.offset += 4;
        v
    }

    fn read_u64(&mut self) -> u64 {
        let v = u64::from_le_bytes(
            self.data.as_ref().unwrap()[self.offset..(self.offset + 8)]
                .try_into()
                .unwrap(),
        );
        self.offset += 8;
        v
    }

    fn read_bigint(&mut self, n8: u32) -> BigUint {
        let usize_n8 = usize::try_from(n8).unwrap();
        // convert an array of LE bytes to an array of LE 64 bit words
        let bytes = &self.data.as_ref().unwrap()[self.offset..(self.offset + usize_n8)];
        // let mut words_64 = [0_u64; 4];
        // for x in 0..4 {
        //     for y in 0..8 {
        //         words_64[x] += u64::from(bytes[x * 8 + y]) << (8 * y);
        //     }
        // }
        self.offset += usize_n8;
        BigUint::from_bytes_le(bytes)
        // BigInteger256::new(words_64)
    }

    // we start at the offset after the section length
    fn read_header(&mut self, _section_len: u64) {
        let key_type = self.read_u32();
        if key_type != 1 {
            panic!("non-groth16 zkey detected");
        }
    }

    fn read_groth16_header(&mut self, _section_len: u64) {
        self.n8q = self.read_u32();
        // read the q
        self.q = self.read_bigint(self.n8q);

        self.n8r = self.read_u32();
        // read the r
        self.r = self.read_bigint(self.n8r);

        self.n_vars = self.read_u32();
        self.n_public = self.read_u32();
        self.domain_size = self.read_u32();
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::circom::zkey::{Fr, ZkeyReader};
//     use ark_bn254::{G1Affine, G2Affine};
//     use ark_circom::read_zkey;
//     use std::fs::File;

//     fn compare_g1_vecs(v1: Vec<G1Affine>, v2: Vec<G1Affine>) {
//         assert!(v1.len() == v2.len());
//         for x in 0..v1.len() {
//             assert!(v1[x] == v2[x]);
//         }
//     }

//     fn compare_g2_vecs(v1: Vec<G2Affine>, v2: Vec<G2Affine>) {
//         assert!(v1.len() == v2.len());
//         for x in 0..v1.len() {
//             assert!(v1[x] == v2[x]);
//         }
//     }

//     fn compare_fr_matrix(v1: Vec<Vec<(Fr, usize)>>, v2: Vec<Vec<(Fr, usize)>>) {
//         assert!(v1.len() == v2.len());
//         for x in 0..v1.len() {
//             assert!(v1[x].len() == v2[x].len());
//             for y in 0..v1[x].len() {
//                 assert!(v1[x][y].0 == v2[x][y].0);
//                 assert!(v1[x][y].1 == v2[x][y].1);
//             }
//         }
//     }

//     #[test]
//     fn test_zkey_parse() {
//         let zkey_path = "../test-vectors/circom/keccak256_256_test_final.zkey".to_string();
//         use std::time::Instant;
//         let now = Instant::now();

//         let mut file = File::open(&zkey_path).unwrap();
//         let c_zkey = read_zkey(&mut file).unwrap();
//         let elapsed = now.elapsed();
//         println!("orig Elapsed: {:.2?}", elapsed);

//         let now = Instant::now();
//         let mut zkey_reader = ZkeyReader::new(&zkey_path);
//         let zkey = zkey_reader.read();
//         let elapsed = now.elapsed();
//         println!("new Elapsed: {:.2?}", elapsed);

//         // Compare the parsed ProvingKey
//         assert!(c_zkey.0.vk.alpha_g1 == zkey.0.vk.alpha_g1);
//         assert!(c_zkey.0.vk.beta_g2 == zkey.0.vk.beta_g2);
//         assert!(c_zkey.0.vk.gamma_g2 == zkey.0.vk.gamma_g2);
//         assert!(c_zkey.0.vk.delta_g2 == zkey.0.vk.delta_g2);
//         compare_g1_vecs(c_zkey.0.vk.gamma_abc_g1, zkey.0.vk.gamma_abc_g1);
//         assert!(c_zkey.0.beta_g1 == zkey.0.beta_g1);
//         assert!(c_zkey.0.delta_g1 == zkey.0.delta_g1);
//         compare_g1_vecs(c_zkey.0.a_query, zkey.0.a_query);
//         compare_g1_vecs(c_zkey.0.b_g1_query, zkey.0.b_g1_query);
//         compare_g2_vecs(c_zkey.0.b_g2_query, zkey.0.b_g2_query);
//         compare_g1_vecs(c_zkey.0.h_query, zkey.0.h_query);
//         compare_g1_vecs(c_zkey.0.l_query, zkey.0.l_query);

//         // Compare the parsed ConstraintMatrices
//         assert!(c_zkey.1.num_instance_variables == zkey.1.num_instance_variables);
//         assert!(c_zkey.1.num_witness_variables == zkey.1.num_witness_variables);
//         assert!(c_zkey.1.num_constraints == zkey.1.num_constraints);
//         assert!(c_zkey.1.a_num_non_zero == zkey.1.a_num_non_zero);
//         assert!(c_zkey.1.b_num_non_zero == zkey.1.b_num_non_zero);
//         assert!(c_zkey.1.c_num_non_zero == zkey.1.c_num_non_zero);
//         compare_fr_matrix(c_zkey.1.a, zkey.1.a);
//         compare_fr_matrix(c_zkey.1.b, zkey.1.b);
//         compare_fr_matrix(c_zkey.1.c, zkey.1.c);
//     }
// }
