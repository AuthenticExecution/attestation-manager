//! # manager_net
//!
//! This library contains some I/O utilities to read/write serializable objects
//! It uses `serde` and `serde_json` external libraries
//! The goal is to provide an easy way to define a custom network protocol that can be used
//! by two remote components to communicate. The protocol is defined by the user, by simply
//! declaring Structs
//!
//! Custom structs have to be defined as in the following example:
//! ```
//! use rust_net::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Test {
//!     // your fields
//! }
//!
//! ```

#[cfg(feature = "manager_structs")]
mod structs;
mod enums;
mod net;

#[cfg(feature = "manager_structs")]
pub use structs::*;
pub use enums::*;
pub use net::*;

pub use serde;
pub use serde_json;
pub use serde::{Serialize, Deserialize};