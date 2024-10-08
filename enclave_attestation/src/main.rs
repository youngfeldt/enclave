use std::error::Error; // Import the Error trait
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Set the output file path
    let file_path = "attestation_document.txt";

    // Retrieve the attestation document
    let attestation_doc = get_attestation_document("example_user_data")?;

    // Write the attestation document to a file
    let mut file = File::create(file_path)?;
    file.write_all(attestation_doc.as_bytes())?;

    println!("Attestation document written to file: {}", file_path);

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