# enclave

## Work in progress

# Idea: 
  Have a process in enclave that retrieves attestation document and sends it to the parent.

## To Make

1. cd enclave attestation
2. docker build -t enclave_attestation . 
3. nitro-cli build-enclave --docker-uri enclave_attestation --output-file enclave_attestation.eif
4. nitro-cli run-enclave --eif-path enclave_attestation.eif --cpu-count 2 --memory 256 --enclave-cid 10
