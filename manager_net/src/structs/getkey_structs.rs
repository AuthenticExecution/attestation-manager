use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetKeyRequest {
    pub name : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetKeyResponse {
    pub module_key : Vec<u8>
}

impl GetKeyRequest {
    pub fn new(name : String) -> GetKeyRequest {
        GetKeyRequest {
            name
        }
    }
}

impl GetKeyResponse {
    pub fn new(module_key : Vec<u8>) -> GetKeyResponse {
        GetKeyResponse {
            module_key
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPubKeyRequest {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPubKeyResponse {
    pub sp_pubkey : Vec<u8>
}

impl GetPubKeyRequest {
    pub fn new() -> GetPubKeyRequest {
        GetPubKeyRequest {
        }
    }
}

impl GetPubKeyResponse {
    pub fn new(sp_pubkey : Vec<u8>) -> GetPubKeyResponse {
        GetPubKeyResponse {
            sp_pubkey
        }
    }
}
