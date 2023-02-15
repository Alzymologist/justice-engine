use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yaml_rust::{YamlEmitter, YamlLoader};
use yew::prelude::*;

use crate::yaml; // Homemade crate.

use crate::storage::Storage;

use libp2p::{futures::StreamExt, kad::{Quorum, Record, record::Key}, PeerId};

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

    let testshit = use_state(|| 0);
    let listener = use_state(|| "not initialized".to_owned());
    let my_id = use_state(|| "not initialized".to_owned());

    //let storage = use_state(|| Storage::new());
    
    
    let cloned_yaml_state = yaml_state.clone();
    let callback = Callback::from(move |s: String| {
        cloned_yaml_state.set(s);
    });

    let read_string = &*yaml_state;
    let read_yaml_vec = YamlLoader::load_from_str(read_string).unwrap();
    let yaml_tree = read_yaml_vec[0].clone();
    let read_tree_for_printing = {
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&yaml_tree).unwrap(); // dump the YAML object to a String
        }
        out_str
    };

    let sanitized_yaml_tree = yaml::sanitize_tree(yaml_tree.clone());
    let sanitized_tree_for_printing = {
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&sanitized_yaml_tree).unwrap(); // dump the YAML object to a String
        }
        out_str
    };

    let hash = yaml::yaml_to_hash(sanitized_yaml_tree);


    let testshit1 = testshit.clone();
    let listen_handle = listener.clone();
    let my_id_handle = my_id.clone();
    let hashcopy = hash.clone();
    use_state(move || {
    wasm_bindgen_futures::spawn_local(async move {
        testshit1.set(*testshit1 + 1);
        let mut storage = Storage::new();
        my_id_handle.set(storage.local_peer_id.to_string());
        testshit1.set(*testshit1 + 10);
        //storage.swarm.behaviour_mut().get_closest_peers("QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN".parse().transpose().unwrap());
        //storage.swarm.dial(PeerId::random());
        //storage.swarm.behaviour_mut().get_closest_peers(storage.local_peer_id);
        //let key = Key::new(&hashcopy);
        /*storage.swarm.behaviour_mut().put_record(Record{
            key: key.clone(),
            value: vec![1, 2, 3],
            publisher: None,
            expires: None,
        }, Quorum::One);
        storage.swarm.behaviour_mut().get_record(key);*/
        loop {
            let event = storage.swarm.select_next_some().await;
            let message = format!("{:?}", event);
            listen_handle.set(message);
        };
    });
    });
    let testvalue = &*testshit;

    html! {
        <form>
            <YamlInput
            name="yaml_form" handle_onchange={callback} value={read_string.clone()} // Properties
            />

        <p>{"client id: "}{&*my_id}</p>

        <p>{"testcounter:"}{testvalue}</p>

        <p>{"Listening on "}{&*listener}</p>

        <hr/>
        <div style="white-space:pre">{"Read YAML:"}<br/>{read_tree_for_printing}</div>
        <hr/>
        <div style="white-space:pre">{"Sanitized YAML:"}<br/>{sanitized_tree_for_printing}</div>
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

