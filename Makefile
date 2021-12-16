REPO             ?= authexec/attestation-manager
TAG              ?= sgx
TAG_NATIVE       ?= native
VOLUME           ?= $(shell pwd)
VENDOR_KEY       ?= keys/vendor_key.pem
SP_PUBKEY        ?= keys/sp_pubkey.pem

PORT             ?= 1234
HOST_NETWORK     ?= 1
SGX_DEVICE       ?= /dev/isgx

ifeq ($(HOST_NETWORK), 1)
	CNTR_NET	= --network=host
else
	CNTR_NET	= -p $(PORT):1234
endif

build:
	docker build -t $(REPO):$(TAG) --build-arg VENDOR_KEY=$(VENDOR_KEY) .

build_native:
	docker build -t $(REPO):$(TAG_NATIVE) -f Dockerfile_native .

push: login
	docker push $(REPO):$(TAG)

pull:
	docker pull $(REPO):$(TAG)

run:
	docker run --rm $(CNTR_NET) -v /var/run/aesmd/:/var/run/aesmd --device $(SGX_DEVICE) --name attestation-manager $(REPO):$(TAG)

run_native:
	docker run --rm $(CNTR_NET) --name attestation-manager $(REPO):$(TAG_NATIVE)

get_sig:
	mkdir -p enclave
	docker run --rm -it --detach --name tmp_container $(REPO):$(TAG) bash
	docker cp tmp_container:/home/enclave/enclave.sig enclave/enclave.sig
	docker stop tmp_container

login:
	docker login

check:
	cargo check

check_sgx:
	MANAGER_PLATFORM=sgx cargo check --features=sgx --target=x86_64-fortanix-unknown-sgx

# Generate fresh keys for signing the enclave and for the AM
keys:
	mkdir -p keys
	openssl genrsa -3 3072 > keys/vendor_key.pem
	openssl genrsa -f4 -out keys/sp_privkey.pem 2048
	openssl rsa -in keys/sp_privkey.pem -outform PEM -pubout -out keys/sp_pubkey.pem

replace_key:
		python3 replace_key.py $(SP_PUBKEY)

.PHONY: keys
