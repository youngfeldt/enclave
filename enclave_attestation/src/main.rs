use std::io::prelude::*;
use vsock::VsockStream;

fn main() -> std::io::Result<()> {
    let cid = 3; // CID for the parent instance
    let port = 5005; // The same port number used by the parent listener

    // Connect to the parent instance VSOCK listener
    let mut stream = VsockStream::connect(cid, port)?;

    let attestation_doc = get_attestation_document("example_user_data")?;
    stream.write_all(attestation_doc.as_bytes())?;

    println!("Sent attestation document to parent. Staying alive...");

    // Keep the enclave alive
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

fn get_attestation_document(user_data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("nsm-cli")
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
