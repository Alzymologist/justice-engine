use base64::{Engine, engine::general_purpose};
use reqwasm::http::{Request, Method};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use log::{info, error};
use yew::functional::{use_effect, use_state};
use yew::prelude::*;
use std::error::Error;

const PROJECT_ID: &str = "2NdtdwZgrdj6fwMubLQMiMTs5nH";
const PROJECT_SECRET: &str = "78c624acbfe219c5d0b4a8566c867ab0";
const ENDPOINT: &str = "https://ipfs.infura.io:5001";

// #[wasm_bindgen(start)]
async fn request_tree() -> Result<String, Box<dyn Error>> {
    console_log::init_with_level(log::Level::Debug).expect("Error initializing console_log");

    // spawn_local(async {
        let auth_header = format!(
            "Basic {}",
            general_purpose::STANDARD.encode(&format!("{}:{}", PROJECT_ID, PROJECT_SECRET))
        );

        let hash = "QmfUwJRRDZxGo8jMvKVGxj6FDn8xsMXcyEbRrYaScCXhRv";

        // READ FILE WITH HASH
        let response = Request::new(&format!("{}/api/v0/cat?arg={}", ENDPOINT, hash))
            .method(Method::POST)
            .header("Authorization", &auth_header)
            .send()
            .await;

        Ok(String::from("Hello"))
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