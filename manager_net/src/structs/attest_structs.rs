use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AttestationRequestNative {
    pub id : u16,
    pub name : String,
    pub host : String,
    pub port : u16,
    pub em_port: u16,
    pub key : Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttestationRequestSGX {
    pub id : u16,
    pub name : String,
    pub host : String,
    pub port : u16,
    pub em_port: u16,
    pub aesm_client_host : String,
    pub aesm_client_port : u16,
    pub sigstruct : Vec<u8>,
    pub config : Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttestationRequestSancus {
    pub id : u16,
    pub name : String,
    pub host : String,
    pub port : u16,
    pub em_port: u16,
    pub key : Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttestationRequestTrustZone {
    pub id : u16,
    pub name : String,
    pub host : String,
    pub port : u16,
    pub em_port: u16,
    pub key : Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttestationResponse {
    pub module_key : Vec<u8>
}

impl AttestationRequestNative {
    pub fn new(id : u16, name : String, host : String, port : u16, em_port : u16,
            key : Vec<u8>) -> AttestationRequestNative {
        AttestationRequestNative {
            id,
            name,
            host,
            port,
            em_port,
            key
        }
    }
}

impl AttestationRequestSancus {
    pub fn new(id : u16, name : String, host : String, port : u16, em_port : u16,
            key : Vec<u8>) -> AttestationRequestSancus {
        AttestationRequestSancus {
            id,
            name,
            host,
            port,
            em_port,
            key
        }
    }
}

impl AttestationRequestSGX {
    pub fn new(id : u16, name : String, host : String, port : u16, em_port : u16, aesm_client_host : String,
        aesm_client_port : u16, sigstruct : Vec<u8>, config : Vec<u8>) -> AttestationRequestSGX {
        AttestationRequestSGX {
            id,
            name,
            host,
            port,
            em_port,
            aesm_client_host,
            aesm_client_port,
            sigstruct,
            config
        }
    }
}

impl AttestationResponse {
    pub fn new(module_key : Vec<u8>) -> AttestationResponse {
        AttestationResponse {
            module_key
        }
    }
}
