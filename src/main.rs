use base64::{Engine, engine::general_purpose};
use reqwasm::http::{Request, Method};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use log::{info, error};

const PROJECT_ID: &str = "2NdtdwZgrdj6fwMubLQMiMTs5nH";
const PROJECT_SECRET: &str = "78c624acbfe219c5d0b4a8566c867ab0";
const ENDPOINT: &str = "https://ipfs.infura.io:5001";

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).expect("Error initializing console_log");

    spawn_local(async {
        let auth_header = format!(
            "Basic {}",
            general_purpose::STANDARD.encode(&format!("{}:{}", PROJECT_ID, PROJECT_SECRET))
        );

        // Define the hash value here
        let hash = "QmfUwJRRDZxGo8jMvKVGxj6FDn8xsMXcyEbRrYaScCXhRv";

        // READ FILE WITH HASH
        let response = Request::new(&format!("{}/api/v0/cat?arg={}", ENDPOINT, hash))
            .method(Method::POST)
            .header("Authorization", &auth_header)
            .send()
            .await;

        match response {
            Ok(response) => {
                if response.status() == 200 {
                    let text = response.text().await.unwrap();
                    info!("File content: {}", text);
                } else {
                    error!("Error: {:?}", response.status());
                }
            }
            Err(err) => {
                error!("Error: {:?}", err);
            }
        }
    });

    Ok(())
}
