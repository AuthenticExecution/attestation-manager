FROM authexec/docker-sgx-tools:latest

ARG APP=attestation-manager
ARG VENDOR_KEY
ARG ELF2SGX_ARGS="--heap-size 0x2000000 --stack-size 0x200000 --threads 8"
ARG SGXSIGN_ARGS="-d --xfrm 7/0 --isvprodid 0 --isvsvn 0"

ENV MANAGER_PLATFORM=sgx

COPY Cargo.toml build.rs ./
COPY manager_net manager_net
COPY src src
COPY $VENDOR_KEY vendor/vendor_key.pem

RUN cargo build --target x86_64-fortanix-unknown-sgx --features sgx \
  && mkdir -p enclave \
  && ftxsgx-elf2sgxs target/x86_64-fortanix-unknown-sgx/debug/$APP --output enclave/enclave.sgxs  $ELF2SGX_ARGS  \
  && sgxs-sign --key vendor/vendor_key.pem enclave/enclave.sgxs enclave/enclave.sig $SGXSIGN_ARGS


FROM authexec/docker-sgx-base:latest
COPY --from=0 /usr/src/app/enclave /home/enclave

CMD ["ftxsgx-runner", "/home/enclave/enclave.sgxs"]
