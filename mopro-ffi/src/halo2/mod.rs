use crate::{GenerateProofResult, MoproError};
use std::collections::HashMap;

pub type Halo2ProveFn =
    fn(&str, &str, HashMap<String, Vec<String>>) -> Result<GenerateProofResult, MoproError>;

pub type Halo2VerifyFn = fn(&str, &str, Vec<u8>, Vec<u8>) -> Result<bool, MoproError>;
