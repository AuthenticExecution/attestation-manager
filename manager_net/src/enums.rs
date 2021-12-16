/// This enum is used to identify a response message
/// A positive response typically contains some custom data, which is wrapped into a struct
/// A negative response should not contain any functional data, but only some information about the error (therefore, it is wrapped on a different struct)
/// Therefore, this value is used to deserialize the correct struct when a response is received
#[derive(Clone, Debug)]
pub enum Response {
    Ok      = 0,
    Error   = 1
}

impl Response {
    pub fn from_u8(num : u8) -> Result<Response, Error> {
        match num {
            0       => Ok(Response::Ok),
            1       => Ok(Response::Error),
            _       => Err(Error::InvalidResponseCode)
        }
    }

    pub fn to_u8(&self) -> u8 {
        self.clone() as u8
    }
}

/// This is a simple enum used when something goes wrong.
#[derive(Debug)]
pub enum Error {
    DeserializationError,
    SerializationError,
    NetworkReadError,
    NetworkWriteError,
    PayloadSizeZero,
    PayloadTooBig,
    InvalidResponseCode
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
        -> Result<(), std::fmt::Error> {
            write!(f, "{:?}", self)
        }
}

impl std::error::Error for Error {}
