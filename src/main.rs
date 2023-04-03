use base64::{Engine, engine::general_purpose};
use reqwest::Client;
use std::error::Error;
use tokio;

const PROJECT_ID: &str = "2NdtdwZgrdj6fwMubLQMiMTs5nH";
const PROJECT_SECRET: &str = "78c624acbfe219c5d0b4a8566c867ab0";
const ENDPOINT: &str = "https://ipfs.infura.io:5001";

fn main() -> Result<(), Box<dyn Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let auth_header = format!(
            "Basic {}",
            general_purpose::STANDARD.encode(&format!("{}:{}", PROJECT_ID, PROJECT_SECRET))
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
            .post(&format!("{}/api/v0/cat", ENDPOINT))
            .query(&[("arg", hash)])
            .send()
            .await?;
        println!("{:?}", response2);
        let response2_text = response2.text().await?;
        println!("{}", response2_text);

        Result::<_, Box<dyn Error>>::Ok(())
    })
}