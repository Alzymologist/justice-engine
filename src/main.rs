use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod yaml;

const RAW_YAML_EXAMPLE: &str = include_str!("example.yaml"); // Will be used for the initial state. //***

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

#[function_component(App)]
pub fn app() -> Html {
    let raw_yaml_string = String::from(RAW_YAML_EXAMPLE); //***

    html! {
        <div>
        <YamlForm name="yaml_form" value={raw_yaml_string} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
