# enclave

## Work in progress

# Idea: 
  Have a process in enclave that retrieves attestation document and sends it to the parent.

## To Make

1. cd enclave attestation
2. docker build enclave_attestation -t . 
3. nitro-cli build-enclave --docker-uri enclave_attestation --output-file enclave_attestation.eif
