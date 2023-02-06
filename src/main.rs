// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(dead_code)]
//
use blake2::{Blake2s256, Digest};
use yaml_rust::{YamlEmitter, YamlLoader};

mod web;
mod yaml;

const EXAMPLE_YAML: &str = include_str!("example.yaml");

fn main() {
    yew::Renderer::<web::App>::new().render();
    let docs = YamlLoader::load_from_str(&EXAMPLE_YAML).unwrap();

    let yaml_tree = docs[0].clone();
    println!("initial_yaml:\n{:?}", yaml_tree);
    let sanitized_yaml_tree = yaml::sanitize_tree(yaml_tree);

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
