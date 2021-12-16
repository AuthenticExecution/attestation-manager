use std::sync::Mutex;
#[macro_use] extern crate lazy_static;

pub mod init;
pub mod event_loop;
pub mod error;
mod platforms;
mod processor;
mod handlers;
mod data;
mod module;
mod logging;

use sgx_attestation::sgx_crypto::Context as TLSSession;
pub const PORT : u16 = 1234;

lazy_static! {
    pub static ref ATTESTATION_KEY: Mutex<Option<Vec<u8>>> = {
        Mutex::new(None)
    };
}