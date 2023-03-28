use base64::{Engine, engine::general_purpose};
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use tokio;

#[derive(Deserialize)]
struct Config {
    PROJECT_ID: String,
    PROJECT_SECRET: String,
    ENDPOINT: String,
}

fn read_config() -> Result<Config, Box<dyn Error>> {
    let mut file = File::open("keys.json")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let config: Config = serde_json::from_str(&data)?;
    Ok(config)
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = read_config()?;

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let auth_header = format!(
            "Basic {}",
            general_purpose::STANDARD.encode(&format!("{}:{}", config.PROJECT_ID, config.PROJECT_SECRET))
        );
        let client = Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert("Authorization", auth_header.parse()?);
                headers
            })
            .build()?;

        // Define the hash value here
        let hash = "QmfUwJRRDZxGo8jMvKVGxj6FDn8xsMXcyEbRrYaScCXhRv";

        // READ FILE WITH HASH
        let response2 = client
            .post(&format!("{}/api/v0/cat", config.ENDPOINT))
            .query(&[("arg", hash)])
            .send()
            .await?;
        println!("{:?}", response2);
        let response2_text = response2.text().await?;
        println!("{}", response2_text);

        Result::<_, Box<dyn Error>>::Ok(())
    })
}