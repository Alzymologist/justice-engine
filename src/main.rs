use bs58;
use serde_json;
use tiny_keccak::{IntoXof, KangarooTwelve, Xof};

const KANGAROO_OUTPUT_SIZE: usize = 8; // Not equal to .hash size in SerializedLaw

struct SerializedLaw {
    law: String,
    hash: String,
}

fn serialize(raw_law: String) -> SerializedLaw {
    let bytes = &raw_law.into_bytes();
    let mut hasher = KangarooTwelve::new(bytes).into_xof();
    let mut output = [0u8; KANGAROO_OUTPUT_SIZE];
    hasher.squeeze(&mut output[..]);

    let hash = bs58::encode(output).into_string(); // Base 58 for better visual representation.
    let law = serde_json::to_string(bytes).unwrap();

    SerializedLaw { law, hash }
}

fn main() {
    let law = String::from("Any strindsdg here.");
    let s = serialize(law);
    println!("{:?}", s.hash);
}
