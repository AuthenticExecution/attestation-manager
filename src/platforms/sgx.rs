use std::sync::Mutex;
use anyhow::Result;

use manager_net::{InitSGXData, AttestationRequestSGX};
use sgx_attestation::attest_enclave;
use crate::error::Error;

lazy_static! {
    static ref SGX_DATA: Mutex<Option<InitSGXData>> = {
        Mutex::new(None)
    };
}

fn is_sgx_initialized() -> bool {
    let sgx_data = SGX_DATA.lock().unwrap();
    sgx_data.is_some()
}

fn set_sgx_data(data : InitSGXData) {
    let mut sgx_data = SGX_DATA.lock().unwrap();
    sgx_data.replace(data);
}

pub fn sgx_attest(data : &AttestationRequestSGX) -> Result<Vec<u8>> {
    if !is_sgx_initialized() {
        return Err(Error::NotInitialized.into());
    }

    let sgx_data = SGX_DATA.lock().unwrap();
    let sgx_data = sgx_data.as_ref().unwrap(); //safe

    attest_enclave(&data.host, data.port, &data.aesm_client_host, data.aesm_client_port,
        &sgx_data.sp_privkey, &sgx_data.ias_certificate, &data.config, &data.sigstruct)
}

pub fn sgx_init(data : InitSGXData) -> Result<()> {
    match is_sgx_initialized() {
        true    => Err(Error::AlreadyInitialized.into()),
        false   => {
            set_sgx_data(data);
            Ok(())
        }
    }
}

pub fn get_sp_pubkey() -> Option<Vec<u8>> {
    let sgx_data = SGX_DATA.lock().unwrap();

    sgx_data.as_ref().map(|d| d.sp_pubkey.clone())
}
