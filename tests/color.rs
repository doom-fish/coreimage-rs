use coreimage::prelude::*;

#[test]
fn colors_expose_components() {
    let color = CIColor::rgba(0.1, 0.2, 0.3, 0.4);
    let named = CIColor::named(CIColorName::Red);

    assert_eq!(color.number_of_components(), 4);
    assert!((color.alpha() - 0.4).abs() < 1e-6);
    assert!(named.red() > 0.9);
    assert!(!named.string_representation().is_empty());
}
