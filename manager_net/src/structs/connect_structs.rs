use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectRequest {
    pub test_1 : String,
    pub test_2 : u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectResponse {
    pub test_1 : String,
    pub test_2 : u32
}
