use anyhow::Result;
use rand::Rng;

use reactive_crypto::{Encryption, encrypt};
use reactive_net::{EntrypointID, CommandCode, CommandMessage, ResultCode};

use super::net::send_reactive_command;
use super::CHALLENGE_LENGTH;
use manager_net::{AttestationRequestTrustZone};
use crate::error::Error;

pub fn trustzone_attest(data : &AttestationRequestTrustZone) -> Result<()> {
    // here I use rand crate to generate a random challenge.
    // Are there security concerns here? Probably it is okay for generating a challenge.
    let challenge = rand::thread_rng().gen::<[u8; CHALLENGE_LENGTH as usize]>();

    // create the payload
    let mut payload = Vec::<u8>::with_capacity(6 + (CHALLENGE_LENGTH as usize));
    payload.extend_from_slice(&data.id.to_be_bytes());
    payload.extend_from_slice(&(EntrypointID::Attest as u16).to_be_bytes());
    payload.extend_from_slice(&CHALLENGE_LENGTH.to_be_bytes());
    payload.extend_from_slice(&challenge);

    // create the command and send it
    let cmd = CommandMessage::new(CommandCode::CallEntrypoint, Some(payload));
    let res = send_reactive_command(&data.host, data.em_port, &cmd)?;

    // verify the response
    match res.get_code() {
        ResultCode::Ok  => (),
        c               => return Err(Error::ReactiveCommandFailed(*c).into())
    }

    let res_mac = res.get_payload().ok_or(Error::ReactiveCommandPayloadError)?;

    // calculate the MAC of the challenge and compare it to the received one
    let mac = encrypt(&[], &data.key, &challenge, &Encryption::Aes)?;

    if mac != res_mac {
        return Err(Error::AttestationFailed.into());
    }

    Ok(())
}
