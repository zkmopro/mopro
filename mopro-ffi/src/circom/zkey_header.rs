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

// This implementation loads only the first few bytes
// of the zkey file to get the groth16 header.
//
// This header tells us what curve the zkey was built for.
// This is difficult to do in zkey.rs because we define the
// size if integers based on the type at the rust level, while
// zkeys specify their integer sizes in the file.
//
// e.g. we need to use the integer size specified in the zkey to
// determine what type to use in rust
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

#[cfg(test)]
mod tests {
    use crate::circom::zkey_header::ZkeyHeaderReader;
    use ark_ff::PrimeField;
    use num_bigint::BigUint;

    #[test]
    fn test_zkey_parse_bn() {
        let zkey_path = "../test-vectors/circom/keccak256_256_test_final.zkey".to_string();

        let mut zkey_reader = ZkeyHeaderReader::new(&zkey_path);
        zkey_reader.read();

        assert_eq!(zkey_reader.n8q, 32);
        assert_eq!(zkey_reader.n8r, 32);
        assert_eq!(zkey_reader.q, BigUint::from(ark_bn254::Fq::MODULUS));
        assert_eq!(zkey_reader.r, BigUint::from(ark_bn254::Fr::MODULUS));
    }

    #[test]
    fn test_zkey_parse_bls() {
        let zkey_path = "../test-vectors/circom/multiplier2_bls_final.zkey".to_string();

        let mut zkey_reader = ZkeyHeaderReader::new(&zkey_path);
        zkey_reader.read();

        assert_eq!(zkey_reader.n8q, 48);
        assert_eq!(zkey_reader.n8r, 32);
        assert_eq!(zkey_reader.q, BigUint::from(ark_bls12_381::Fq::MODULUS));
        assert_eq!(zkey_reader.r, BigUint::from(ark_bls12_381::Fr::MODULUS));
    }
}
