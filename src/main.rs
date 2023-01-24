#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use blake2::{Blake2s256, Digest};
use std::collections::BTreeMap;
use yaml_rust::{YamlEmitter, YamlLoader};

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
    // println!("{:?}", docs);
    let doc = &docs[0];

    let doc_clone = doc.clone();
    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }
    // println!("{:?}", out_str);

    // let deserialized: BTreeMap<String, f64> = serde_yaml::from_str(&yaml_string).unwrap(); // Deserialize and
    // let serialized = serde_yaml::to_string(&yaml).unwrap(); // serialize back, to ensure constent representaion.
    // let serialized_copy = serialized.clone();

    let mut hasher = Blake2s256::new();
    hasher.update(out_str);
    let hash = hasher.finalize();

    println!("{:?}\nhash: {:x}", doc_clone, hash);
}
