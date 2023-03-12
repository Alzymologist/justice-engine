use blake2::{Blake2s256, Digest};
use hex_literal::hex;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;
use yaml_rust::{Yaml, YamlEmitter};
use hex::encode;

pub fn sanitize_tree(mut yaml_to_sanitize: Yaml) -> Yaml {
    match yaml_to_sanitize {
        Yaml::Real(initial) => {
            let num: f64 = initial.parse().expect("Failed to parse Real.");
            if num.fract() == 0.0
            // Precision up to 15 decimal places.
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

pub fn yaml_to_hash(sanitized_yaml: Yaml) -> String {
    let mut s = String::new();
    let mut emitter = YamlEmitter::new(&mut s);
    emitter.dump(&sanitized_yaml).unwrap();

    let mut hasher = Blake2s256::new();
    hasher.update(s);
    let hash = hasher.finalize();
    hex::encode(hash)
}
