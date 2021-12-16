mod init_structs;
mod connect_structs;
mod attest_structs;
mod error_struct;
mod getkey_structs;
mod reset_structs;

pub use init_structs::*;
pub use connect_structs::*;
pub use attest_structs::*;
pub use error_struct::*;
pub use getkey_structs::*;
pub use reset_structs::*;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use strum_macros::EnumString;

#[derive(IntoPrimitive, TryFromPrimitive, Clone, Debug, EnumString)]
#[repr(u8)]
pub enum Command {
    #[strum(serialize = "init")]
    Init,

    // Init commands for each attestation platform
    #[strum(serialize = "initsgx", serialize = "init_sgx", serialize = "init-sgx")]
    InitSGX,
    #[strum(serialize = "initsancus", serialize = "init_sancus", serialize = "init-sancus")]
    InitSancus,
    #[strum(serialize = "inittrustzone", serialize = "init_trustzone", serialize = "init-trustzone")]
    InitTrustZone,

    // operational commands
    #[strum(serialize = "attestnative", serialize = "attest_native", serialize = "attest-native")]
    AttestNative,
    #[strum(serialize = "attestsgx", serialize = "attest_sgx", serialize = "attest-sgx")]
    AttestSGX,
    #[strum(serialize = "attestsancus", serialize = "attest_sancus", serialize = "attest-sancus")]
    AttestSancus,
    #[strum(serialize = "attesttrustzone", serialize = "attest_trustzone", serialize = "attest-trustzone")]
    AttestTrustZone,

    #[strum(serialize = "connect")]
    Connect,

    #[strum(serialize = "getkey", serialize = "get_key", serialize = "get-key")]
    GetKey,

    #[strum(serialize = "getpubkey", serialize = "get_pub_key", serialize = "get-pub-key")]
    GetPubKey,

    #[strum(serialize = "reset")]
    Reset,
}

impl Command {
    pub fn from_u8(num : u8) -> Option<Command> {
        match Command::try_from(num) {
            Ok(c)   => Some(c),
            _       => None
        }
    }

    pub fn to_u8(&self) -> u8 {
        self.clone().into()
    }
}
