use crate::sgxlib::sgx_types::{
    error::{SgxResult, SgxStatus},
    types::KeyRequest,
};

pub fn get_key(key_request: &KeyRequest) -> SgxResult<[u8; 16]> {
    Err(SgxStatus::UnsupportedFeature)
}
