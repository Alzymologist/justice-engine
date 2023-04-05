use blake2::{Blake2s256, Digest};
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

pub fn sanitize_yaml(mut yaml_to_sanitize: Yaml) -> Yaml {
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
                hashmap_to_modify.insert(key.to_owned(), sanitize_yaml(value.clone()));
            }
        }
        Yaml::Array(ref mut vec) => {
            let vec_to_traverse = vec.clone();
            for (i, element) in vec_to_traverse.iter().enumerate() {
                vec[i] = sanitize_yaml(element.clone());
            }
        }
        _ => (),
    }
    yaml_to_sanitize
}

pub fn hash_yaml(sanitized_yaml: Yaml) -> String {
    let mut s = String::new();
    let mut emitter = YamlEmitter::new(&mut s);
    emitter.dump(&sanitized_yaml).unwrap();

    let mut hasher = Blake2s256::new();
    hasher.update(s);
    let hash = hasher.finalize();
    hex::encode(hash)
}

pub fn load_yaml(read_string: &String) -> Result<Yaml, String> {
    match YamlLoader::load_from_str(read_string) {
        Ok(t) => Ok(t[0].clone()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn print_yaml(yaml_to_print: Yaml) -> String {
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&yaml_to_print).unwrap(); // dump the YAML object to a String
    }
    out_str
}
