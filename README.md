# enclave

## To Make

1. cd enclave attestation
2. docker build enclave_attestation -t . 
3. nitro-cli build-enclave --docker-uri enclave_attestation --output-file enclave_attestation.eif
