use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InitData {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitSGXData {
    pub sp_privkey : Vec<u8>,
    pub sp_pubkey : Vec<u8>,
    pub ias_certificate : Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitSancusData {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitTrustZoneData {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitResponse {
}

impl InitData {
    pub fn new() -> InitData {
        InitData {
        }
    }
}

impl InitSGXData {
    pub fn new(sp_privkey : Vec<u8>, sp_pubkey : Vec<u8>, ias_certificate : Vec<u8>) -> InitSGXData {
        InitSGXData {
            sp_privkey,
            sp_pubkey,
            ias_certificate
        }
    }
}


impl InitResponse {
    pub fn new() -> InitResponse {
        InitResponse {
        }
    }
}
