// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(dead_code)]

use blake2::{Blake2s256, Digest};
use std::collections::BTreeMap;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn read_yaml (path: &Path) -> String {
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

    let deserialized: BTreeMap<String, f64> = serde_yaml::from_str(&yaml_string).unwrap();  // Deserialize and
    let serialized = serde_yaml::to_string(&deserialized).unwrap();  // serialize back, to ensure constent representaion.
    let serialized_copy = serialized.clone();

    let mut hasher = Blake2s256::new();
    hasher.update(serialized);
    let hash = hasher.finalize();

    println!("{}\nhash: {:x}", serialized_copy, hash);
}
