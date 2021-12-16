use crate::{info,debug, ATTESTATION_KEY};

#[cfg(platform = "sgx")]
use sgx_attestation::do_attestation;
#[cfg(platform = "sgx")]
use crate::{error, PORT};

pub fn init_manager() {
    // ID and EM port not used here
    info!("Waiting for attestation");
    attest();
    info!("Attestation succeeded");
}

#[cfg(platform = "native")]
fn attest() {
    debug!("Native attestation");

    //replace the key here with the one really used
    let key = vec!(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
    let mut attestation_key = ATTESTATION_KEY.lock().unwrap();
    attestation_key.replace(key);
}

#[cfg(platform = "sgx")]
fn attest() {
    debug!("SGX attestation");
    // Hardcode your SP public key here.
    let pubkey = "\
-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAqv0UL87sGYb9jDoga0TI
LjI+U49Nn2j/9QXTIvmMg4K96dVJEkmxI6ZbtFCRWMxGbnpdPCQOJ5oZ3s/86G+R
BxcPBYWLkVQ5wXbwvQic6Dui79Jqj251vij2VPI2ifqoTveZkYuMaAD9VaobgD0A
SAs6Do7Z3Pskiq20AKbnGYyAOAt3hK5+3QlazdeF2py2UMNl7Z6Bvj1oZPzxDxZ/
I65MHURA5Q8Gl1X458qAQlCunt9OeGtmizKovBTBNsVSUbgOv1OnJgfscU/s1Dy6
lzrNRaPNxdVP7AcrRz6uLVw3ZEbuCc4B5O7QDrEWDp3WXB7rUypmAVtdIj7JW5YU
WQIDAQAB
-----END PUBLIC KEY-----
\0";
    let listen_port = PORT;

    match do_attestation(listen_port, pubkey) {
        Ok(key)     => {
            let mut attestation_key = ATTESTATION_KEY.lock().unwrap();
            attestation_key.replace(key);
        },
        Err(e)      => {
            error!(e);
            panic!("Attestation failed");
        }
    }
}

#[cfg(platform = "trustzone")]
fn attest() {
    panic!("TrustZone attestation not implemented");
}

#[cfg(not(any(platform = "sgx", platform = "native", platform = "trustzone")))]
fn attest() {
    panic!("Unknown platform");
}
