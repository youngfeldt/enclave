use std::os::unix::net::{UnixListener, UnixStream};
use std::io::prelude::*;
use serde_json::Value;
use std::fs;
use std::path::Path;

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
    let socket_path = "/vsock/vm.sock";

    // Ensure the directory exists
    if let Some(parent) = Path::new(socket_path).parent() {
        fs::create_dir_all(parent)?;
    }

    // Attempt to remove existing socket file
    if Path::new(socket_path).exists() {
        if let Err(e) = fs::remove_file(socket_path) {
            eprintln!("Failed to remove existing socket file: {}", e);
        }
    }

    let listener = UnixListener::bind(socket_path)?;
    println!("VSOCK listener waiting for connections...");

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

