use std::net::{TcpListener, TcpStream};
use sgx_attestation::sgx_crypto::tls_psk::*;
use sgx_attestation::sgx_crypto::Context;

use crate::{debug, info, warning, PORT, ATTESTATION_KEY};
use crate::error::Error;
use crate::processor::process_message;

use anyhow::Result;
use std::convert::TryInto;

pub fn start() -> Result<()> {
    let listener = match TcpListener::bind(("0.0.0.0", PORT)) {
        Ok(l)       => l,
        Err(_)      => return Err(Error::EventLoopInitError.into())
    };

    info!(&format!("Listening on {}", PORT));

    for stream in listener.incoming() {
        match stream {
            Ok(s)   => handle_client(s),
            Err(_)  => warning!("Connection with client failed")
        }
    }

    Ok(())
}

fn handle_client(stream : TcpStream) {
    // establish TLS
    let session = match establish_tls_server(stream) {
        Ok(s)   => s,
        Err(_)  => {
            warning!("Failed to establish TLS session");
            return;
        }
    };

    debug!("Established new TLS connection");
    if let Err(e) = process_message(session) {
        warning!(&format!("Error: {}", e));
    }
    debug!("Connection closed");
}

pub fn establish_tls_server(stream : TcpStream) -> Result<Context<TcpStream>> {
    let attestation_key = ATTESTATION_KEY.lock().unwrap();
    
    let key : &[u8] = match attestation_key.as_ref() {
        Some(k)     => k,
        None        => return Err(Error::NotInitialized.into())
    };

    let key : &[u8; 16] = match key.try_into() {
        Ok(k)   => k,
        Err(_)  => return Err(Error::KeyConversionError.into())
    };

    let ctx = match new_server_context(key, stream) {
        Ok(c)   => c,
        Err(_)  => return Err(Error::TLSError.into())
    };

    Ok(ctx)
}