use clap::{Arg, App};
use reqwest::multipart;
use std::fs::File;
use std::io::Read;
use serde_json::Value;

const API_URL: &str = "https://api.openxpress.cloud";

#[tokio::main]
async fn main() {
    let matches = App::new("Docker Tool")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Uploads a ZIP file and configures Docker containers")
        .arg(Arg::new("username")
            .short('u')
            .long("username")
            .value_name("USERNAME")
            .help("Username for authentication")
            .required(true)
            .takes_value(true))
        .arg(Arg::new("password")
            .short('p')
            .long("password")
            .value_name("PASSWORD")
            .help("Password for authentication")
            .required(true)
            .takes_value(true))
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .value_name("FILE")
            .help("Sets the ZIP file to upload")
            .required(true)
            .takes_value(true))
        .arg(Arg::new("ports")
            .short('P')
            .long("ports")
            .value_name("PORTS")
            .help("Sets the ports for the Docker container")
            .required(true)
            .takes_value(true))
        .get_matches();

    let username = matches.value_of("username").unwrap();
    let password = matches.value_of("password").unwrap();
    let file = matches.value_of("file").unwrap();
    let ports = matches.value_of("ports").unwrap();

    match login(username, password).await {
        Ok(token) => {
            match upload_zip(file, &token).await {
                Ok(response) => {
                    // Extract the file_url from the response
                    let file_url: String = extract_file_url(&response).expect("Failed to extract file URL");

                    // Configure the Docker container
                    match configure_docker(&file_url, ports, &token).await {
                        Ok(response) => println!("Docker configured: {}", response),
                        Err(e) => eprintln!("Error configuring Docker: {}", e),
                    }
                },
                Err(e) => eprintln!("Error uploading file: {}", e),
            }
        },
        Err(e) => eprintln!("Error logging in: {}", e),
    }
}

async fn login(username: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("{}/login", API_URL);

    // Create a hashmap with the form data
    let mut form_data = std::collections::HashMap::new();
    form_data.insert("username", username);
    form_data.insert("password", password);

    let response = client.post(&url)
        .form(&form_data)  // Use .form for form data
        .send()
        .await?
        .text()
        .await?;

    // Extract the token from the response
    let token = extract_token(&response)?;
    // println!("Token: {}", token);
    Ok(token)
}

async fn upload_zip(file_path: &str, token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut file = File::open(file_path)?;
    let mut file_contents = Vec::new();
    file.read_to_end(&mut file_contents)?;
    let file_name = file_path.to_string();
    let form = multipart::Form::new()
        .part("file", multipart::Part::stream_with_length(file_contents.clone(), file_contents.len() as u64).file_name(file_name));
    let url = format!("{}/upload_zip", API_URL);
    let response = client.post(&url)
        .bearer_auth(token)
        .multipart(form)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

async fn configure_docker(image_url: &str, ports: &str, token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("{}/upload_docker_image", API_URL);
    let response = client.post(&url)
        .bearer_auth(token)
        .form(&[("image_url", image_url), ("ports", ports)])
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

fn extract_file_url(response: &str) -> Option<String> {
    // Parse the response to extract the file URL (assuming it's a JSON response)
    let json_response: Value = serde_json::from_str(response).ok()?;
    json_response["file_url"].as_str().map(|s| s.to_string())
}

fn extract_token(response: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Parse the response to extract the token (assuming it's a JSON response)
    let json_response: Value = serde_json::from_str(response)?;
    if let Some(token) = json_response["access_token"].as_str() {
        Ok(token.to_string())
    } else {
        Err("Token not found in response".into())
    }
}
