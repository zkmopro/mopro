use ark_bn254::{Bn254, Fq, Fq2, Fr, G1Affine, G2Affine};
use ark_ff::{BigInteger256, PrimeField};
use ark_groth16::{ProvingKey, VerifyingKey};
use ark_relations::r1cs::ConstraintMatrices;
use ark_serialize::Read;
use ark_std::Zero;
use std::fs::File;

pub struct ZkeyReader {
    zkey_path: std::path::PathBuf,
    offset: usize,
    data: Option<Vec<u8>>,
    n8q: u32,
    n8r: u32,
    n_public: u32,
    n_vars: u32,
    domain_size: u32,
    q: BigInteger256,
    r: BigInteger256,
    key: ProvingKey<Bn254>,
    matrices: Option<ConstraintMatrices<Fr>>,
}

fn default_key() -> ProvingKey<Bn254> {
    ProvingKey {
        vk: VerifyingKey::default(),
        beta_g1: G1Affine::default(),
        delta_g1: G1Affine::default(),
        a_query: Vec::new(),
        b_g1_query: Vec::new(),
        b_g2_query: Vec::new(),
        h_query: Vec::new(),
        l_query: Vec::new(),
    }
}

impl ZkeyReader {
    pub fn new(zkey_path: &str) -> Self {
        ZkeyReader {
            zkey_path: std::path::PathBuf::from(zkey_path),
            offset: 0,
            data: None,
            n8q: 0,
            n8r: 0,
            n_public: 0,
            n_vars: 0,
            domain_size: 0,
            q: BigInteger256::zero(),
            r: BigInteger256::zero(),
            key: default_key(),
            matrices: None,
        }
    }

    pub fn read(&mut self) -> (ProvingKey<Bn254>, ConstraintMatrices<Fr>) {
        let mut file = File::open(self.zkey_path.clone()).unwrap();
        let mut zkey_bytes = Vec::new();
        file.read_to_end(&mut zkey_bytes).unwrap();
        self.data = Some(zkey_bytes);
        self.parse();
        (self.key.clone(), self.matrices.as_ref().unwrap().clone())
    }

    fn parse(&mut self) {
        let _magic = self.read_u32();
        let _version = self.read_u32();
        let num_sections = self.read_u32();
        for _ in 0..num_sections {
            let section_id = self.read_u32();
            let section_len = self.read_u64();
            self.read_section(section_id, section_len);
        }
    }

    fn read_section(&mut self, section_n: u32, section_len: u64) {
        match section_n {
            1 => self.read_header(section_len),
            2 => self.read_groth16_header(section_len),
            3 => self.read_ic(section_len),
            4 => self.read_ccoefs(section_len),
            5 => self.read_a(section_len),
            6 => self.read_b1(section_len),
            7 => self.read_b2(section_len),
            8 => self.read_c(section_len),
            9 => self.read_h(section_len),
            10 => (|| {})(), // ignore reading the contributions
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

    fn read_bigint(&mut self, n8: u32) -> BigInteger256 {
        let usize_n8 = usize::try_from(n8).unwrap();
        // convert an array of LE bytes to an array of LE 64 bit words
        let bytes = &self.data.as_ref().unwrap()[self.offset..(self.offset + usize_n8)];
        let mut words_64 = [0_u64; 4];
        for x in 0..4 {
            for y in 0..8 {
                words_64[x] += u64::from(bytes[x * 8 + y]) << (8 * y);
            }
        }
        self.offset += usize_n8;
        BigInteger256::new(words_64)
    }

    fn read_fr(&mut self, n8: u32) -> Fr {
        // Double unwrap to divide by R?
        Fr::new_unchecked(Fr::new_unchecked(self.read_bigint(n8)).into_bigint())
    }

    fn read_g1(&mut self, n8: u32) -> G1Affine {
        let x = Fq::new_unchecked(self.read_bigint(n8));
        let y = Fq::new_unchecked(self.read_bigint(n8));
        let infinity = x.is_zero() && y.is_zero();
        if infinity {
            G1Affine::identity()
        } else {
            G1Affine::new_unchecked(x, y)
        }
    }

    fn read_g2(&mut self, n8: u32) -> G2Affine {
        let f1_x = Fq::new_unchecked(self.read_bigint(n8));
        let f1_y = Fq::new_unchecked(self.read_bigint(n8));
        let f1 = Fq2::new(f1_x, f1_y);
        let f2_x = Fq::new_unchecked(self.read_bigint(n8));
        let f2_y = Fq::new_unchecked(self.read_bigint(n8));
        let f2 = Fq2::new(f2_x, f2_y);
        let infinity = f1.is_zero() && f2.is_zero();
        if infinity {
            G2Affine::identity()
        } else {
            G2Affine::new_unchecked(f1, f2)
        }
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
        self.key.vk.alpha_g1 = self.read_g1(self.n8q);
        self.key.beta_g1 = self.read_g1(self.n8q);
        self.key.vk.beta_g2 = self.read_g2(self.n8q);
        self.key.vk.gamma_g2 = self.read_g2(self.n8q);
        self.key.delta_g1 = self.read_g1(self.n8q);
        self.key.vk.delta_g2 = self.read_g2(self.n8q);
    }

    fn read_ic(&mut self, _section_len: u64) {
        let mut ic = Vec::new();
        for _ in 0..(self.n_public + 1) {
            ic.push(self.read_g1(self.n8q));
        }
        self.key.vk.gamma_abc_g1 = ic;
    }

    fn read_ccoefs(&mut self, _section_len: u64) {
        let ccoef_len = self.read_u32();
        let mut max_constraint_index = 0_u32;
        let mut matrices = vec![vec![vec![]; self.domain_size as usize]; 2];
        for _ in 0..ccoef_len {
            let matrix = self.read_u32();
            let constraint = self.read_u32();
            let signal = self.read_u32();
            let val = self.read_fr(self.n8r);
            if constraint > max_constraint_index {
                max_constraint_index = constraint;
            }
            matrices[matrix as usize][constraint as usize].push((val, signal as usize));
        }
        let num_constraints = (max_constraint_index - self.n_public) as usize;
        matrices.iter_mut().for_each(|m| {
            m.truncate(num_constraints);
        });
        // This is taken from Arkworks' to_matrices() function
        let a = matrices[0].clone();
        let b = matrices[1].clone();
        let a_num_non_zero: usize = a.iter().map(|lc| lc.len()).sum();
        let b_num_non_zero: usize = b.iter().map(|lc| lc.len()).sum();
        self.matrices = Some(ConstraintMatrices {
            num_instance_variables: (self.n_public + 1) as usize,
            num_witness_variables: (self.n_vars - self.n_public) as usize,
            num_constraints,
            a_num_non_zero,
            b_num_non_zero,
            c_num_non_zero: 0,
            a,
            b,
            c: vec![],
        });
    }

    fn read_a(&mut self, _section_len: u64) {
        let mut a = Vec::new();
        for _ in 0..self.n_vars {
            a.push(self.read_g1(self.n8q));
        }
        self.key.a_query = a;
    }

    fn read_b1(&mut self, _section_len: u64) {
        let mut b1 = Vec::new();
        for _ in 0..self.n_vars {
            b1.push(self.read_g1(self.n8q));
        }
        self.key.b_g1_query = b1;
    }

    fn read_b2(&mut self, _section_len: u64) {
        let mut b2 = Vec::new();
        for _ in 0..self.n_vars {
            b2.push(self.read_g2(self.n8q));
        }
        self.key.b_g2_query = b2;
    }

    fn read_c(&mut self, _section_len: u64) {
        let mut c = Vec::new();
        for _ in (self.n_public + 1)..self.n_vars {
            c.push(self.read_g1(self.n8q));
        }
        self.key.l_query = c;
    }

    fn read_h(&mut self, _section_len: u64) {
        let mut h = Vec::new();
        for _ in 0..self.domain_size {
            h.push(self.read_g1(self.n8q));
        }
        self.key.h_query = h;
    }
}

#[cfg(test)]
mod tests {
    use crate::circom::zkey::{Fr, ZkeyReader};
    use ark_bn254::{G1Affine, G2Affine};
    use ark_circom::read_zkey;
    use std::fs::File;

    fn compare_g1_vecs(v1: Vec<G1Affine>, v2: Vec<G1Affine>) {
        assert!(v1.len() == v2.len());
        for x in 0..v1.len() {
            assert!(v1[x] == v2[x]);
        }
    }

    fn compare_g2_vecs(v1: Vec<G2Affine>, v2: Vec<G2Affine>) {
        assert!(v1.len() == v2.len());
        for x in 0..v1.len() {
            assert!(v1[x] == v2[x]);
        }
    }

    fn compare_fr_matrix(v1: Vec<Vec<(Fr, usize)>>, v2: Vec<Vec<(Fr, usize)>>) {
        assert!(v1.len() == v2.len());
        for x in 0..v1.len() {
            assert!(v1[x].len() == v2[x].len());
            for y in 0..v1[x].len() {
                assert!(v1[x][y].0 == v2[x][y].0);
                assert!(v1[x][y].1 == v2[x][y].1);
            }
        }
    }

    #[test]
    fn test_zkey_parse() {
        let zkey_path = "../test-vectors/circom/keccak256_256_test_final.zkey".to_string();
        use std::time::Instant;
        let now = Instant::now();

        let mut file = File::open(&zkey_path).unwrap();
        let c_zkey = read_zkey(&mut file).unwrap();
        let elapsed = now.elapsed();
        println!("orig Elapsed: {:.2?}", elapsed);

        let now = Instant::now();
        let mut zkey_reader = ZkeyReader::new(&zkey_path);
        let zkey = zkey_reader.read();
        let elapsed = now.elapsed();
        println!("new Elapsed: {:.2?}", elapsed);

        // Compare the parsed ProvingKey
        assert!(c_zkey.0.vk.alpha_g1 == zkey.0.vk.alpha_g1);
        assert!(c_zkey.0.vk.beta_g2 == zkey.0.vk.beta_g2);
        assert!(c_zkey.0.vk.gamma_g2 == zkey.0.vk.gamma_g2);
        assert!(c_zkey.0.vk.delta_g2 == zkey.0.vk.delta_g2);
        compare_g1_vecs(c_zkey.0.vk.gamma_abc_g1, zkey.0.vk.gamma_abc_g1);
        assert!(c_zkey.0.beta_g1 == zkey.0.beta_g1);
        assert!(c_zkey.0.delta_g1 == zkey.0.delta_g1);
        compare_g1_vecs(c_zkey.0.a_query, zkey.0.a_query);
        compare_g1_vecs(c_zkey.0.b_g1_query, zkey.0.b_g1_query);
        compare_g2_vecs(c_zkey.0.b_g2_query, zkey.0.b_g2_query);
        compare_g1_vecs(c_zkey.0.h_query, zkey.0.h_query);
        compare_g1_vecs(c_zkey.0.l_query, zkey.0.l_query);

        // Compare the parsed ConstraintMatrices
        assert!(c_zkey.1.num_instance_variables == zkey.1.num_instance_variables);
        assert!(c_zkey.1.num_witness_variables == zkey.1.num_witness_variables);
        assert!(c_zkey.1.num_constraints == zkey.1.num_constraints);
        assert!(c_zkey.1.a_num_non_zero == zkey.1.a_num_non_zero);
        assert!(c_zkey.1.b_num_non_zero == zkey.1.b_num_non_zero);
        assert!(c_zkey.1.c_num_non_zero == zkey.1.c_num_non_zero);
        compare_fr_matrix(c_zkey.1.a, zkey.1.a);
        compare_fr_matrix(c_zkey.1.b, zkey.1.b);
        compare_fr_matrix(c_zkey.1.c, zkey.1.c);
    }
}
