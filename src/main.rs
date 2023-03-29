use base64::{engine::general_purpose, Engine};
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use wasm_bindgen_futures::spawn_local;
use yew::functional::{use_effect, use_state};
use yew::prelude::*;

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

async fn request_tree() -> Result<String, Box<dyn Error>> {
    let config = read_config()?;

    let auth_header = format!(
        "Basic {}",
        general_purpose::STANDARD
            .encode(&format!("{}:{}", config.PROJECT_ID, config.PROJECT_SECRET))
    );
    let client = Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Authorization", auth_header.parse()?);
            headers
        })
        .build()?;

    let hash = "QmfUwJRRDZxGo8jMvKVGxj6FDn8xsMXcyEbRrYaScCXhRv";

    // READ FILE WITH HASH
    let response2 = client
        .post(&format!("{}/api/v0/cat", config.ENDPOINT))
        .query(&[("arg", hash)])
        .send()
        .await?;
    println!("{:?}", response2);
    let response2_text = response2.text().await?;
    Ok(response2_text)
}

#[function_component(App)]
fn app() -> Html {
    let response_state = use_state(|| String::from("Loading..."));
    {
        let response_state = response_state.clone();
            spawn_local(async move {
                let result = request_tree().await;
                let message = match result {
                    Ok(s) => s,
                    Err(e) => e.to_string(),
                };
                response_state.set(message);
            });
    }
    html! {
        <>
        {format!("{:?}", response_state)}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}