use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub description : String
}

impl ErrorResponse {
    pub fn new(description : String) -> ErrorResponse {
        ErrorResponse {
            description
        }
    }
}
