// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(dead_code)]

use yew::prelude::*;
use blake2::{Blake2s256, Digest};
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_yaml(path: &Path) -> String {
    let display = path.display();
    let mut file = match File::open(path) {
        Err(err) => panic!("couldn't open {}: {}", display, err),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(err) => panic!("couldn't read {}: {}", display, err),
        Ok(_) => s,
    }
}

fn sanitize_tree(mut yaml_to_sanitize: Yaml) -> Yaml {
    match yaml_to_sanitize {
        Yaml::Real(initial) => {
            let num: f64 = initial.parse().expect("Failed to parse Real.");
            if num.fract() == 0.0  // Precision up to 15 decimal places.
            {
                yaml_to_sanitize = Yaml::Integer(num as i64)
            } else {
                let sanitized = num.to_string();
                yaml_to_sanitize = Yaml::Real(sanitized);
            }
        }
        Yaml::Hash(ref mut hashmap_to_modify) => {
            let hashmap_to_traverse = hashmap_to_modify.clone();
            for (key, value) in hashmap_to_traverse.iter() {
                hashmap_to_modify.insert(key.to_owned(), sanitize_tree(value.clone()));
            }
        }
        Yaml::Array(ref mut vec) => {
            let vec_to_traverse = vec.clone();
            for (i, element) in vec_to_traverse.iter().enumerate() {
                vec[i] = sanitize_tree(element.clone());
            }
        }
        _ => (),
    }
    yaml_to_sanitize
}


#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();

    let path = Path::new("example2.yaml");
    let yaml_string = read_yaml(&path);
    let docs = YamlLoader::load_from_str(&yaml_string).unwrap();

    let yaml_tree = docs[0].clone();
    println!("initial_yaml:\n{:?}", yaml_tree);
    let sanitized_yaml_tree = sanitize_tree(yaml_tree);

    {
        let hash_to_mod_clone = sanitized_yaml_tree.clone(); // For printing
        let mut s = String::new();
        let mut emitter = YamlEmitter::new(&mut s);
        emitter.dump(&sanitized_yaml_tree).unwrap();

        let mut hasher = Blake2s256::new();
        hasher.update(s);
        let hash = hasher.finalize();

        println!(
            "sanitized_yaml:\n{:?} \nhash:\n{:x}",
            hash_to_mod_clone, hash
        );
    }
}
