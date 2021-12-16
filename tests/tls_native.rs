use std::net::TcpListener;
use std::io::Read;

use attestation_manager::init::init_manager;
use attestation_manager::event_loop::establish_tls_server;
use attestation_manager::{PORT, info};

#[test]
#[ignore]
fn test_tls_native() {
    init_manager();

    let listener = TcpListener::bind(("0.0.0.0", PORT)).expect("Failed to bind port");

    info!(&format!("Listening on {}", PORT));

    let stream = listener.accept().expect("Failed to accept stream").0;

    info!("Received new connection");

    let mut session = establish_tls_server(stream).expect("Failed to establish TLS connection");

    info!("Established TLS connection");

    let mut msg = vec![0u8; 5];
    session.read_exact(&mut msg[..]).expect("Failed to read bytes");

    assert_eq!(msg, b"hello");

    info!("Completed");
}
