use coreimage::prelude::*;

fn main() {
    let generator = CIFilterGenerator::new();
    println!(
        "generator exported keys: {}",
        generator.exported_keys_json()
    );
    println!("generator has filter: {}", generator.filter().is_some());
}
