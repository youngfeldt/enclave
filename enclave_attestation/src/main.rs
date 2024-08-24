use std::fs::File;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::process::Command;

fn get_attestation_document(user_data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("nsm-cli")
        .arg("request-attestation")
        .arg("--user-data")
        .arg(user_data)
        .output()?;

    if !output.status.success() {
        return Err(format!("Failed to get attestation document: {:?}", output.stderr).into());
    }

    let attestation_doc = String::from_utf8(output.stdout)?;
    Ok(attestation_doc)
}

fn send_to_vsock_server(attestation_doc: &str) -> Result<(), Box<dyn std::error::Error>> {
    let vsock_path = "/vsock/vm.sock"; // Path to the VSOCK socket file
    let mut stream = UnixStream::connect(vsock_path)?;

    stream.write_all(attestation_doc.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_data = "example_user_data";
    let attestation_doc = get_attestation_document(user_data)?;

    send_to_vsock_server(&attestation_doc)?;

    println!("Sent attestation document. Waiting to stay alive...");

    // Keep the enclave alive by sleeping indefinitely
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
