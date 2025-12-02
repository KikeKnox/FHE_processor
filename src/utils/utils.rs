use tfhe::prelude::*;

pub fn generate_keys() -> (tfhe::ClientKey, tfhe::ServerKey) {
    let client_key = tfhe::ClientKey::generate_with_seed(config, seed)
}