use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetRequest {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetResponse {
}

impl ResetRequest {
    pub fn new() -> ResetRequest {
        ResetRequest {}
    }
}

impl ResetResponse {
    pub fn new() -> ResetResponse {
        ResetResponse {}
    }
}
