use gloo_net::http::Request;
use serde::Deserialize;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let yamls = use_state(|| YamlLoader::load_from_str(" ").unwrap());
    {
        let yamls = yamls.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_yamls: String = Request::get(
                "http://127.0.0.1:8081/ipfs/QmfUwJRRDZxGo8jMvKVGxj6FDn8xsMXcyEbRrYaScCXhRv",
            )
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
            let fetched_yamls = YamlLoader::load_from_str(&fetched_yamls).unwrap();
            yamls.set(fetched_yamls);
        });
    }
    html! {
        <>
        {format!("{:?}", yamls)}
        </>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
