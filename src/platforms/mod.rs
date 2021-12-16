pub mod sgx;
pub mod sancus;
pub mod trustzone;
mod net;

pub use sgx::*;
pub use sancus::*;
pub use trustzone::*;

const CHALLENGE_LENGTH : u16 = 16;
