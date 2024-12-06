use crate::sgxlib::{
    sgx_tse::EnclaveKey,
    sgx_types::{error::SgxResult, types::KeyRequest},
};

pub fn get_key(key_request: &KeyRequest) -> SgxResult<[u8; 16]> {
    key_request.get_key()
}
