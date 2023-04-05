use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use base64::{Engine, engine::general_purpose};
use gloo::console::log;
use reqwasm::http::{Request, Method};
use wasm_bindgen_futures::spawn_local;

mod yaml;

const PROJECT_ID: &str = "2NdtdwZgrdj6fwMubLQMiMTs5nH";
const PROJECT_SECRET: &str = "78c624acbfe219c5d0b4a8566c867ab0";
const ENDPOINT: &str = "https://ipfs.infura.io:5001";
const HASH: &str = "QmfUwJRRDZxGo8jMvKVGxj6FDn8xsMXcyEbRrYaScCXhRv";

async fn request_tree() -> String {
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
// const RAW_YAML_EXAMPLE: &str = include_str!("example.yaml"); // Will be used for the initial state. //***


#[derive(Properties, PartialEq)]
pub struct Properties {
    pub name: String,
    pub value: String,
}

#[function_component(YamlForm)]
pub fn yaml_form(properties: &Properties) -> Html {
    // Loading of the initial state, it latter be used to write the initial value for the textarea:
    let yaml_state = use_state(|| properties.value.clone());

    let yaml_state_for_callback = yaml_state.clone();
    let oninput = Callback::from(move |event: InputEvent| {
        let new_value: String = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        yaml_state_for_callback.set(new_value);
    });

    let input = html! {
        <form>
            <textarea rows="20" cols="50" oninput = {oninput} value = {(&*yaml_state).to_string()} />
            <br/>
        </form>
    };
    let output = match yaml::load_yaml(&*yaml_state) {
        Ok(yaml_tree) => {
            let sanitized_yaml = yaml::sanitize_yaml(yaml_tree);
            let yaml_hash = yaml::hash_yaml(sanitized_yaml.clone());
            let printed_yaml = yaml::print_yaml(sanitized_yaml);

            html! {
                <form>
                    <div style="white-space:pre">{"Yaml is correct 🌟"}</div>
                    <p>{"Hash: "}<br/>{yaml_hash}</p>
                    <div style="white-space:pre">{"Sanitized YAML:"}<br/>{printed_yaml}</div>
                </form>
            }
        }
        Err(error) => {
            html! {
                <form>
                <div style="white-space:pre">{"Yaml is broken ❗"}</div>
                <p>{"Error: "}<br/>{error}</p>
                </form>
            }
        }
    };

    html! { <div> {input} {output} </div> }
}

// #[function_component(App)]
// pub fn app() -> Html {
//     let raw_yaml_string = String::from(RAW_YAML_EXAMPLE); //***

//     html! {
//         <div>
//         <YamlForm name="yaml_form" value={raw_yaml_string} />
//         </div>
//     }
// }

#[function_component(App)]
fn app() -> Html {
    let response_state = use_state(|| 
        String::from("Loading..."));
    {
        let response_state = response_state.clone();
            spawn_local(async move {
                let result = request_tree().await; 
                response_state.set(result);
            });
    }
    html! {
        <div>
        <YamlForm name="yaml_form" value={format!("{:?}", response_state)} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
