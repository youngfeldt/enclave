use std::io::prelude::*;
use serde_json::Value;
use vsock::{VsockListener, VsockAddr};
use libc::VMADDR_CID_ANY; // Import libc directly for VMADDR_CID_ANY

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
    let vsock_port = 5005; // The same port number the enclave will connect to
    let addr = VsockAddr::new(VMADDR_CID_ANY, vsock_port); // Bind to any CID for listening

    // Create a VSOCK listener using the VsockAddr
    let listener = VsockListener::bind(&addr)?;
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
