use crate::{Error, Response};

use serde::{Serialize, Deserialize};
use std::io::{Read, Write};
use std::convert::TryFrom;

/// This function reads a struct that implements the `Deserialize` trait from an input stream
/// The input stream must implement the `std::io::Read` trait
///
/// Returns a `Result` wrapping the struct read from the stream
///
/// # Arguments
///
/// * `stream` - The input stream
pub fn read_from_stream<R : Read, T: for<'de> Deserialize<'de>>
    (stream : &mut R) -> Result<T, Error> {

    let mut length = [0u8; 2];
    if let Err(_) = stream.read_exact(&mut length) {
        return Err(Error::NetworkReadError);
    }

    let l = u16::from_be_bytes(length);
    if l == 0 {
        return Err(Error::PayloadSizeZero);
    }

    let mut buf = vec!(0u8; l as usize);

    if let Err(_) = stream.read_exact(&mut buf) {
        return Err(Error::NetworkReadError);
    }

    match serde_json::from_slice(&buf) {
        Ok(s)   => Ok(s),
        Err(_)  => Err(Error::DeserializationError)
    }
}

/// This function writes a struct that implements the `Serialize` trait to an output stream
/// The output stream must implement the `std::io::Write` trait
///
/// Returns a `Result` to simply indicate if the write succeeded
///
/// # Arguments
///
/// * `stream` - The input stream
/// * `obj` - The instance of the struct to write
pub fn write_to_stream<W: Write, T: Serialize>
    (stream : &mut W, obj : &T) -> Result<(), Error> {
    let buf = match serde_json::to_vec(obj) {
        Ok(b)   => b,
        Err(_)  => return Err(Error::SerializationError)
    };

    let len = buf.len();

    let len_arr = match u16::try_from(len) {
        Ok(l)   => l.to_be_bytes(),
        Err(_)  => return Err(Error::PayloadTooBig)
    };

    if let Err(_) = stream.write(&len_arr) {
        return Err(Error::NetworkWriteError)
    }
    if let Err(_) = stream.write(&buf) {
        return Err(Error::NetworkWriteError)
    }

    Ok(())
}

/// This function reads a command from input stream
/// Similar to read_from_stream, but before the JSON object it attempts to read a byte
/// indicating the command code, which has to be managed outside of this library
///
/// Returns a `Result` wrapping a tuple containing the command code and the `T` instance
///
/// # Arguments
///
/// * `stream` - The input stream
pub fn read_command<R : Read, T: for<'de> Deserialize<'de>>
    (stream : &mut R) -> Result<(u8, T), Error> {
    let mut code = [0u8; 1];
    if let Err(_) = stream.read_exact(&mut code) {
        return Err(Error::NetworkReadError);
    }

    match read_from_stream::<R, T>(stream) {
            Ok(r)   => Ok((code[0], r)),
            Err(e)  => Err(e)
    }
}

/// This function should be used to write a command (i.e., send a request)
/// It is similar to `write_to_stream`, however this function writes a byte before the JSON object
/// to indicate the command code, which has to be managed outside of this library
///
/// Returns a `Result` to simply indicate if the write succeeded
///
/// # Arguments
///
/// * `stream` - The output stream
/// * `code` - The command code
/// * `obj` - The instance of the struct to write
pub fn write_command<W: Write, T: Serialize>
    (stream : &mut W, code: u8, obj : &T) -> Result<(), Error> {

    if let Err(_) = stream.write(&[code]) {
        return Err(Error::NetworkWriteError)
    }

    write_to_stream::<W, T>(stream, obj)
}

/// This function reads a response from input stream
/// It performs a check on the first byte received (`Response` enum code)
/// If the response code is `Response::Ok`, it tries to read the `T` struct
/// Otherwise, it tries to read the `E` struct
///
/// Returns a `Result` wrapping another `Result` with the two struct options (either `T` or `E`)
///
/// # Arguments
///
/// * `stream` - The input stream
///
/// # Examples
///
/// Read a `ResultData` object from a `TcpStream` if a previous request succeeded (`Result::Ok`)
/// If the request failed (`Result::Error`), read a `ErrorData` object
///
/// ```no_run
/// use std::net::TcpStream;
/// use rust_net::{read_response, Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct ResultData{ /* ... */ }
///
/// #[derive(Serialize, Deserialize)]
/// struct ErrorData{ /* ... */ }
///
/// let mut stream = TcpStream::connect("1.2.3.4:4444").unwrap();
///
/// // ...
///
/// match read_response::<TcpStream, ResultData, ErrorData>(&mut stream).unwrap() {
///     Ok(obj)         => (), // do something with the ResultData object
///     Err(obj)        => (), // do something with the ErrorData object
/// }
/// ```
pub fn read_response<R : Read, T: for<'de> Deserialize<'de>,
    E: for<'de> Deserialize<'de>>
    (stream : &mut R) -> Result<Result<T, E>, Error> {
    let mut code = [0u8; 1];
    if let Err(_) = stream.read_exact(&mut code) {
        return Err(Error::NetworkReadError);
    }

    let code = Response::from_u8(code[0])?;

    match code {
        Response::Ok    => match read_from_stream::<R, T>(stream) {
            Ok(r)   => Ok(Ok(r)),
            Err(e)  => Err(e)
        },
        _               => match read_from_stream::<R, E>(stream) {
            Ok(r)   => Ok(Err(r)),
            Err(e)  => Err(e)
        }
    }
}

/// This function should be used to write a response of a previous request
/// It is similar to `write_to_stream`, however this function also writes a `Response` byte
/// to indicate whether the request succeeded or not
///
/// Returns a `Result` to simply indicate if the write succeeded
///
/// # Arguments
///
/// * `stream` - The output stream
/// * `resp` - The response code, instance of the enum `Response`
/// * `obj` - The instance of the struct to write
pub fn write_response<W: Write, T: Serialize>
    (stream : &mut W, resp : Response, obj : &T) -> Result<(), Error> {
    if let Err(_) = stream.write(&[resp.to_u8()]) {
        return Err(Error::NetworkWriteError)
    }

    write_to_stream::<W, T>(stream, obj)
}
