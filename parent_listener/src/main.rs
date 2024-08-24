use std::io::prelude::*;
use serde_json::Value;
use vsock::VsockListener;

fn handle_client(mut stream: vsock::VsockStream) -> std::io::Result<()> {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;

    let attestation_doc: Value = serde_json::from_str(&buffer).expect("Invalid JSON");

    if let Some(pcrs) = attestation_doc.get("pcrs") {
        println!("Received PCR values:");
        for (key, value) in pcrs.as_object().unwrap() {
            println!("PCR {}: {}", key, value);
        }
    } else {
        println!("No PCR values found in the attestation document.");
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    // Define the VSOCK port on which the parent instance will listen for connections
    let vsock_port = 5005; // Use the same port number that the enclave will connect to

    // Create a VSOCK listener
    let listener = VsockListener::bind(vsock_port)?;
    println!("VSOCK listener waiting for connections on port {}...", vsock_port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(err) = handle_client(stream) {
                    eprintln!("Failed to handle client: {}", err);
                }
            }
            Err(err) => {
                eprintln!("Connection failed: {}", err);
            }
        }
    }

    Ok(())
}
