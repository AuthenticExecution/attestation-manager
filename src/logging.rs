#[cfg(feature = "debug")]
#[macro_export]
macro_rules! debug {
    ($msg:expr) => {{
            println!("[attestation-manager] DEBUG: {}", $msg);
    }};
}
#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! debug {
    ($( $args:expr ),*) => {{}};
}
#[macro_export]
macro_rules! info {
    ($msg:expr) => {{
            println!("[attestation-manager] INFO: {}", $msg);
    }};
}
#[macro_export]
macro_rules! error {
    ($msg:expr) => {{
            eprintln!("[attestation-manager] ERROR: {}", $msg);
    }};
}
#[macro_export]
macro_rules! warning {
    ($msg:expr) => {{
            eprintln!("[attestation-manager] WARNING: {}", $msg);
    }};
}