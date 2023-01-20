use bs58;
use serde_json;
use tiny_keccak::{IntoXof, KangarooTwelve, Xof};

const KANGAROO_OUTPUT_SIZE: usize = 8; // Not equal to hash size in SerializedLaw

struct SerializedLaw {
    law: String,
    hash: String,
}

fn serialize(raw_law: String) -> SerializedLaw {
    let bytes = raw_law.into_bytes();
    let serialized_law = serde_json::to_string(&bytes).unwrap();  // String serialization.

    let serialized_law_clone = serialized_law.clone();
    let mut hasher = KangarooTwelve::new(serialized_law.into_bytes()).into_xof(); // Hash is calculated from a serialized string.
    let mut output = [0u8; KANGAROO_OUTPUT_SIZE];
    hasher.squeeze(&mut output[..]);

    let hash = bs58::encode(output).into_string(); // Base 58 for better visual representation.

    SerializedLaw { law: serialized_law_clone, hash }
}

fn main() {
    let law = String::from("Any strindsdg here.");
    let s = serialize(law);
    println!("{:?}", s.hash);
}
