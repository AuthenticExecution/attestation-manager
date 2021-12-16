use reactive_net::ResultCode;

#[derive(Debug)]
pub enum Error {
    EventLoopInitError,
    InvalidCommand,
    HandlerNotImplemented,
    InternalError(String),
    AlreadyInitialized,
    NotInitialized,
    NetworkError,
    ReactiveCommandFailed(ResultCode),
    ReactiveCommandPayloadError,
    AttestationFailed,
    TLSError,
    KeyConversionError
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
        -> Result<(), std::fmt::Error> {
            write!(f, "{:?}", self)
        }
}

impl std::error::Error for Error {}
