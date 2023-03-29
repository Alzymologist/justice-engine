use yew::prelude::*;
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

fn request_tree() -> Result<String, Box<dyn Error>> {
    let config = read_config()?;

    let rt = tokio::runtime::Runtime::new()?;
    let response2_text = rt.block_on(async {
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

        Result::<String, Box<dyn Error>>::Ok(response2_text)
    })?;

    Ok(response2_text)
}

    //////


// use gloo_net::http::Request;
// use yaml_rust::{Yaml, YamlLoader};
// use tokio::runtime::Builder;

// fn get_yamls() -> Vec<Yaml> {

//     let rt = Builder::new_current_thread()
//         .build()
//         .unwrap();

//     let request = Request::get("http://127.0.0.1:8081/ipfs/QmfUwJRRDZxGo8jMvKVGxj6FDn8xsMXcyEbRrYaScCXhRv");

//     let fetched_yamls = rt.block_on(async {
//         let response_text = request
//             .send()
//             .await
//             .unwrap()
//             .text()
//             .await
//             .unwrap();
//         YamlLoader::load_from_str(&response_text).unwrap()
//     });
//     fetched_yamls
// }


#[function_component(App)]
fn app() -> Html {
    let string_to_render = match request_tree() {
        Ok(s) => {s},
        Err(e) => {e.to_string() }};

    html! {
        <>
        {format!("{:?}", string_to_render)}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}