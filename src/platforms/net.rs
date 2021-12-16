use std::net::TcpStream;
use anyhow::Result;

use crate::error::Error;
use reactive_net::{CommandMessage, ResultMessage};

pub fn send_reactive_command(host : &str, port : u16, cmd : &CommandMessage) -> Result<ResultMessage> {
    let addr = format!("{}:{}", host, port);

    let mut stream = match TcpStream::connect(addr) {
        Ok(s) => s,
        Err(_) => return Err(Error::NetworkError.into())
    };

    reactive_net::write_command(&mut stream, cmd)?;
    let result = reactive_net::read_result(&mut stream)?;

    Ok(result)
}
