use yew::prelude::*;
use base64::{Engine, engine::general_purpose};
use surf::Client;
use std::error::Error;

// use http_client::isahc::IsahcClient as DefaultClient;

const PROJECT_ID: &str = "2NdtdwZgrdj6fwMubLQMiMTs5nH";
const PROJECT_SECRET: &str = "78c624acbfe219c5d0b4a8566c867ab0";
const ENDPOINT: &str = "https://ipfs.infura.io:5001";

async fn request_tree() -> Result<String, Box<dyn Error>> {
    let auth_header = format!(
        "Basic {}",
        general_purpose::STANDARD.encode(&format!("{}:{}", PROJECT_ID, PROJECT_SECRET))
    );
    let client = Client::new();

    // Define the hash value
    let hash = "QmfUwJRRDZxGo8jMvKVGxj6FDn8xsMXcyEbRrYaScCXhRv";

    // READ FILE WITH HASH
    let mut request = client
        .post(&format!("{}/api/v0/cat", ENDPOINT))
        .header("Authorization", &auth_header);

    request = request.query(&[("arg", hash)])?;

    let mut response = request.send().await?;
    let response_text = response.body_string().await?;
    Ok(response_text)
}

#[function_component(App)]
fn app() -> Html {
    let videos = use_state(|| String::from(" "));
    {
        let videos = videos.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let answer = request_tree();
            let fetched_videos = match answer.await {
                Ok(r) => {r},
                Err(e) => e.to_string(),
            };
            videos.set(fetched_videos);
        });
    }
    html! {
        <>
        {format!("{:?}", videos)}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
