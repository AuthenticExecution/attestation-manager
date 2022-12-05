use crate::handlers::*;
use crate::error::Error;
use crate::TLSSession;
use std::net::TcpStream;
use manager_net::Command;

use std::io::Read;
use anyhow::Result;

pub fn process_message(mut session : TLSSession<TcpStream>) -> Result<()> {
    let mut arr : [u8; 1] = [0];
    session.read_exact(&mut arr)?;

    let cmd = match Command::from_u8(arr[0]) {
        Some(c)     => c,
        None        => return Err(Error::InvalidCommand.into())
    };

    match cmd {
        Command::Init               => handler_init(&mut session),
        Command::InitSGX            => handler_init_sgx(&mut session),
        Command::AttestNative       => handler_attest_native(&mut session),
        Command::AttestSGX          => handler_attest_sgx(&mut session),
        Command::AttestSancus       => handler_attest_sancus(&mut session),
        Command::AttestTrustZone    => handler_attest_trustzone(&mut session),
        Command::GetKey             => handler_get_key(&mut session),
        Command::GetPubKey          => handler_get_pub_key(&mut session),
        Command::Reset              => handler_reset(&mut session),
        _                           => {
            return Err(Error::HandlerNotImplemented.into());
        }
    }
}
