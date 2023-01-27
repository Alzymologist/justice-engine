#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use blake2::{Blake2s256, Digest};
use linked_hash_map::LinkedHashMap;
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

fn sanitize_tree(hashmap_to_modify: &mut LinkedHashMap<Yaml, Yaml>)
 {
    let hashmap_to_traverse = hashmap_to_modify.to_owned();
    for (key, value) in hashmap_to_traverse.iter() {
        println!("initial:\n{:?}", hashmap_to_modify[&key]);
        match value {
            Yaml::Real(initial) => {
                let numeric: f64 = initial.parse().expect("Failed to parse Real.");
                let sanitized = numeric.to_string();
                hashmap_to_modify.insert(key.to_owned(), Yaml::Real(sanitized));
            }
            Yaml::Hash(deeper_hashmap_to_traverse) => {

                if let Yaml::Hash(mut hashmap2) = hashmap_to_modify[&key].to_owned(){
                sanitize_tree(&mut hashmap2);
                }

                // let mut deeper_hashmap_to_modify = &hashmap_to_modify[&key];
                // sanitize_tree(deeper_hashmap_to_modify);
            }
            _ => (),
        }
    }
}

fn main() {
    let path = Path::new("example2.yaml");
    let yaml_string = read_yaml(&path);
    let docs = YamlLoader::load_from_str(&yaml_string).unwrap();

    if let Yaml::Hash(hashmap) = &docs[0] {
        println!("initial_hashtable:\n{:?}", hashmap);
        let mut hashmap_to_mod = hashmap.to_owned();
        sanitize_tree(&mut hashmap_to_mod);
        //
        let s1 = Yaml::String("foo".to_string());
        let h1 = Yaml::String("test1".to_string());
        let h2 = &hashmap[&h1].as_hash();
        println!("access:\n{:?}", h2);

        {
            let hash_to_mod_clone = hashmap_to_mod.clone(); // For printing
            let mut s = String::new();
            let mut emitter = YamlEmitter::new(&mut s);
            emitter.dump(&Yaml::Hash(hashmap_to_mod)).unwrap(); // Dump the YAML to a String

            let mut hasher = Blake2s256::new();
            hasher.update(s);
            let hash = hasher.finalize();

            println!(
                "sanitized_hashtable:\n{:?} \nhash:\n{:x}",
                hash_to_mod_clone, hash
            );
        }
    }
}
// Dump the YAML object

// let mut hash_to_mod = hash.to_owned();
// println!("{:?}", hash_to_mod);

// for (key, value) in hash.iter() {
//     match value {
//         Yaml::Real(initial) => {
//             let numeric: f64 = initial.parse().expect("Failed to parse Real.");
//             let sanitized = numeric_representation.to_string();
//             hash_to_mod.insert(key.to_owned(), Yaml::Real(sanitized));
//         }
//         _ => (),
//     }

//     // if let Yaml::Real(_) = value {
//     //     hash_to_mod.insert(key.to_owned(), Yaml::Real("9.99999999".to_string()));
//     // }
// }
// println!("{:?}", hash_to_mod);
// }

// let path = Path::new("example2.yaml");
// let yaml_string = read_yaml(&path);
// let docs = YamlLoader::load_from_str(&yaml_string).unwrap();

// let doc = &docs[0];
// let doc_clone = doc.clone();

// let y = &Yaml::Real(String::from("9.9999"));
// for (key, mut item) in doc.as_hash().unwrap() {
//     item = match item {
//         Yaml::Real(initial_float_string) => y,
//         other => other,
//     }
// }

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
