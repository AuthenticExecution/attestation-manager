# AM keys

These are of course toy keys, and should never be used in production.

## Explanation

- `vendor_key.pem`: key used in the Dockerfile to sign the AM enclave
- `sp_{privkey,pubkey}.pem`: keypair used during attestation, used by the AM to authenticate the challenger
    - the AM embeds the public key in the code. During attestation, this key is used to "attest" the challenger, who uses the private key