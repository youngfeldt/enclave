use std::os::unix::net::{UnixListener, UnixStream};
use std::io::prelude::*;
use serde_json::Value;

fn handle_client(mut stream: UnixStream) -> std::io::Result<()> {
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
    let listener = UnixListener::bind("/vsock/vm.sock")?;

    println!("VSOCK listener waiting for connections...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream)?;
            }
            Err(err) => {
                println!("Connection failed: {}", err);
            }
        }
    }

    Ok(())
}

