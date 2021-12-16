use attestation_manager::{init, event_loop};

fn main() {
    // init enclave. Includes attestation
    init::init_manager();

    // start event loop
    event_loop::start().unwrap();
}
