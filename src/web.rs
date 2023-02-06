use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const EXAMPLE_YAML: &str = include_str!("example.yaml");

#[derive(Properties, PartialEq)]
pub struct Properties {
    name: String,
}

#[function_component(YamlForm)]
pub fn text_input(p: &Properties) -> Html {
    let onchange = Callback::from(|event: Event| {
        let value: String = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        log!(value);
    });

    html! {
        <
            textarea
            rows="20"
            cols="50"
            onchange = {onchange}
            value={EXAMPLE_YAML.clone()}
        />
    }
}

#[function_component(App)]
pub fn app() -> Html {
    // let counter = use_state(|| "h");
    // let onclick = {
    //     let counter = counter.clone();
    //     move |_| {
    //         let value = *counter;
    //         counter.set(value);
    //     }
    // };
    //
    //
    let load = Callback::from(|message: String| log!(message));
    html! {
        <div>
            <YamlForm name="yamlform" />
            // <button {onclick}>{ "Parse Yaml" }</button>
            // <p>{ *counter }</p>
        </div>
    }
}
