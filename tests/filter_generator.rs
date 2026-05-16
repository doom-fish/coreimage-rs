use coreimage::prelude::*;

#[test]
fn filter_generator_has_empty_state_by_default() {
    let generator = CIFilterGenerator::new();
    let json = generator.exported_keys_json();

    assert!(json.contains('{'));
    assert!(generator.filter().is_some());
}
