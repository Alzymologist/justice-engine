#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use blake2::{Blake2s256, Digest};
use std::collections::BTreeMap;
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

fn main() {
    let path = Path::new("example2.yaml");
    let yaml_string = read_yaml(&path);
    let docs = YamlLoader::load_from_str(&yaml_string).unwrap();

    let doc = &docs[0];
    let doc_clone = doc.clone();

    let y = &Yaml::Real(String::from("9.9999"));
    for (key, mut item) in doc.as_hash().unwrap() {
        item = match item {
            Yaml::Real(initial_float_string) => y,
            other => other,
        }
    }

    // Dump the YAML object
    let mut processed_yaml_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut processed_yaml_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }

    let mut hasher = Blake2s256::new();
    hasher.update(processed_yaml_str);
    let hash = hasher.finalize();

    println!("item:\n{:?} \nhash:\n{:x}", doc_clone, hash);
}

//  let y = &Yaml::Real(String::from("9.9999"));
// for (key, mut item) in doc.as_hash().unwrap() {
//     item = match item {
//         Yaml::Real(initial_float_string) => y,
//         other => other,
//     }
// }

// match &doc {
//     Yaml::

// }
// for (key, item) in doc_clone2.as_hash().unwrap() {
//     match item {
//     Array => println!("key:{:?}, Array", key),
//     _ => println!("key:{:?}, itemi:{:?}", key, item),
//     }
// }
//     for (key, value) in &doc_clone2 {
// }
