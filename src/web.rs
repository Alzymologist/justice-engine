use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yaml_rust::{YamlEmitter, YamlLoader, Yaml};
use yew::prelude::*;

use crate::yaml; // Homemade crate.

const RAW_YAML_EXAMPLE: &str = include_str!("example.yaml"); // Will be used for the initial state.

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub name: String,
    pub handle_onchange: Callback<String>,
    pub value: String,
}

#[function_component(YamlForm)]
pub fn yaml_form() -> Html {
    // Loading the initial state for "Read Yaml:" field:
    let yaml_state = use_state(|| RAW_YAML_EXAMPLE.to_owned());
    // This state will latter be used to write the initial value for the textarea.

    let cloned_yaml_state = yaml_state.clone();
    let callback = Callback::from(move |s: String| {
        cloned_yaml_state.set(s);
    });

    let status_indicator: &str;
    let read_string = &*yaml_state;

    let yaml_tree = match YamlLoader::load_from_str(read_string){
        Ok(t) => {
            status_indicator = "Yaml is correct 👌";
            t[0].clone()
        }
        Err(e) => {
            status_indicator = "Yaml is broken ❗";
            Yaml::from_str(" ").clone()
        }
    };

    let sanitized_yaml_tree = yaml::sanitize_tree(yaml_tree);
    let sanitized_tree_for_printing = {
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&sanitized_yaml_tree).unwrap(); // dump the YAML object to a String
        }
        out_str
    };

    let hash = yaml::yaml_to_hash(sanitized_yaml_tree);

    html! {
        <form>
            <YamlInput
            name="yaml_form" handle_onchange={callback} value={read_string.clone()} // Properties
            />
        <div style="white-space:pre">{status_indicator}</div>
        <br/>
        <hr/>
        <div style="white-space:pre">{"Sanitized YAML:"}<br/>{sanitized_tree_for_printing}</div>
        <hr/>
        <p>{"Hash: "}<br/>{hash}</p>
        </form>
    }
}

#[function_component(YamlInput)]
pub fn yaml_input(p: &Properties) -> Html {
    let handle_onchange = p.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let new_value: String = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        handle_onchange.emit(new_value);
    });

    html! {
        <
            textarea  // Render textarea
            rows="20"
            cols="50"
            onchange = {onchange}
            value = {p.value.clone()} // This line is included to support both the initial state and updates on change.
        />
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <YamlForm/>
        </div>
    }
}
