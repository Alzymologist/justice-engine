use base64::{Engine, engine::general_purpose};
use log::{info, error};
use reqwasm::http::{Request, Method};
use wasm_bindgen_futures::spawn_local;
use yew::functional::{use_effect, use_state};
use yew::prelude::*;

const PROJECT_ID: &str = "2NdtdwZgrdj6fwMubLQMiMTs5nH";
const PROJECT_SECRET: &str = "78c624acbfe219c5d0b4a8566c867ab0";
const ENDPOINT: &str = "https://ipfs.infura.io:5001";
const HASH: &str = "QmfUwJRRDZxGo8jMvKVGxj6FDn8xsMXcyEbRrYaScCXhRv";

async fn request_tree() -> String {
    console_log::init_with_level(log::Level::Debug).expect("Error initializing console_log");

        let auth_header = format!(
            "Basic {}",
            general_purpose::STANDARD.encode(&format!("{}:{}", PROJECT_ID, PROJECT_SECRET))
        );

        let result = Request::new(&format!("{}/api/v0/cat?arg={}", ENDPOINT, HASH))
            .method(Method::POST)
            .header("Authorization", &auth_header)
            .send()
            .await;

        match result {
            Ok(res) => res.text().await.unwrap(),
            Err(err) => err.to_string(),
        }
}


#[function_component(App)]
fn app() -> Html {
    let response_state = use_state(|| String::from("Loading..."));
    {
        let response_state = response_state.clone();
            spawn_local(async move {
                let result = request_tree().await; 
                response_state.set(result);
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