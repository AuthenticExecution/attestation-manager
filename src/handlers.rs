use crate::{TLSSession, debug, info};
use manager_net::*;
use anyhow::Result;
use std::net::TcpStream;

use crate::data::{add_module, get_module_key, clear_map};
use crate::module::Module;
use crate::platforms::*;

pub fn handler_init(session : &mut TLSSession<TcpStream>) -> Result<()> {
    debug!("handler_init");

    // Not used for now
    let _data : InitData = read_from_stream(session)?;
    write_response(session, Response::Ok, &InitResponse::new())?;
    Ok(())
}

pub fn handler_init_sgx(session : &mut TLSSession<TcpStream>) -> Result<()> {
    debug!("handler_init_sgx");
    let data : InitSGXData = read_from_stream(session)?;

    match sgx_init(data) {
        Ok(_)   =>  {
            write_response(session, Response::Ok, &InitResponse::new())?;
            info!("SGX initialization succeeded");
        }
        Err(e)  => write_response(session, Response::Error, &ErrorResponse::new(e.to_string()))?
    }

    Ok(())
}

pub fn handler_attest_native(session : &mut TLSSession<TcpStream>) -> Result<()> {
    debug!("handler_attest_native");

    let data : AttestationRequestNative = read_from_stream(session)?;

    add_module(data.name.clone(), Module::new(data.id, data.host, data.port, data.em_port, data.key.clone()))?;
    write_response(session, Response::Ok, &AttestationResponse::new(data.key))?;
    info!(&format!("Native attestation of {} succeeded", data.name));

    Ok(())
}

pub fn handler_attest_sgx(session : &mut TLSSession<TcpStream>) -> Result<()> {
    debug!("handler_attest_sgx");

    let data : AttestationRequestSGX = read_from_stream(session)?;

    match sgx_attest(&data) {
        Ok(v)   =>  {
            add_module(data.name.clone(), Module::new(data.id, data.host, data.port, data.em_port, v.clone()))?;
            write_response(session, Response::Ok, &AttestationResponse::new(v))?;
            info!(&format!("SGX attestation of {} succeeded", data.name));
        }
        Err(e)  => write_response(session, Response::Error, &ErrorResponse::new(e.to_string()))?
    }

    Ok(())
}

pub fn handler_attest_sancus(session : &mut TLSSession<TcpStream>) -> Result<()> {
    debug!("handler_attest_sancus");

    let data : AttestationRequestSancus = read_from_stream(session)?;

    match sancus_attest(&data) {
        Ok(_)   =>  {
            add_module(data.name.clone(), Module::new(data.id, data.host, data.port, data.em_port, data.key.clone()))?;
            write_response(session, Response::Ok, &AttestationResponse::new(data.key))?;
            info!(&format!("Sancus attestation of {} succeeded", data.name));
        }
        Err(e)  => write_response(session, Response::Error, &ErrorResponse::new(e.to_string()))?
    }

    Ok(())
}

pub fn handler_attest_trustzone(session : &mut TLSSession<TcpStream>) -> Result<()> {
    debug!("handler_attest_trustzone");

    let data : AttestationRequestTrustZone = read_from_stream(session)?;

    match trustzone_attest(&data) {
        Ok(_)   =>  {
            add_module(data.name.clone(), Module::new(data.id, data.host, data.port, data.em_port, data.key.clone()))?;
            write_response(session, Response::Ok, &AttestationResponse::new(data.key))?;
            info!(&format!("TrustZone attestation of {} succeeded", data.name));
        }
        Err(e)  => write_response(session, Response::Error, &ErrorResponse::new(e.to_string()))?
    }

    Ok(())
}

pub fn handler_get_key(session : &mut TLSSession<TcpStream>) -> Result<()> {
    debug!("handler_get_key");

    let data : GetKeyRequest = read_from_stream(session)?;

    match get_module_key(&data.name) {
        Some(k) => write_response(session, Response::Ok, &GetKeyResponse::new(k))?,
        None    => write_response(session, Response::Error, &ErrorResponse::new(String::from("module not yet attested")))?
    }

    Ok(())
}

pub fn handler_get_pub_key(session : &mut TLSSession<TcpStream>) -> Result<()> {
    debug!("handler_get_pub_key");

    let _data : GetPubKeyRequest = read_from_stream(session)?;

    match get_sp_pubkey() {
        Some(k) => write_response(session, Response::Ok, &GetPubKeyResponse::new(k))?,
        None    => write_response(session, Response::Error, &ErrorResponse::new(String::from("SGX not initialized yet")))?
    }

    Ok(())
}

pub fn handler_reset(session : &mut TLSSession<TcpStream>) -> Result<()> {
    debug!("handler_reset");

    let _data : ResetRequest = read_from_stream(session)?;
    clear_map();
    info!("Reset");

    write_response(session, Response::Ok, &ResetResponse::new())?;
    Ok(())
}
