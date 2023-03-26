use gloo_net::http::Request;
use yaml_rust::{Yaml, YamlLoader};
use yew::prelude::*;
use tokio::runtime::Builder;

fn get_yamls() -> Vec<Yaml> {

    let rt = Builder::new_current_thread()
        .build()
        .unwrap();

    let request = Request::get("http://127.0.0.1:8081/ipfs/QmfUwJRRDZxGo8jMvKVGxj6FDn8xsMXcyEbRrYaScCXhRv");

    let fetched_yamls = rt.block_on(async {
        let response_text = request
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        YamlLoader::load_from_str(&response_text).unwrap()
    });
    fetched_yamls
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        {format!("{:?}", get_yamls())}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
