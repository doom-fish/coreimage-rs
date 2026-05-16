use coreimage::prelude::*;

#[test]
fn vectors_round_trip_geometry() {
    let rect = CGRect::new(1.0, 2.0, 3.0, 4.0);
    let rect_vector = CIVector::from_rect(rect);
    let transform = CGAffineTransform::translation(5.0, 6.0);
    let transform_vector = CIVector::from_transform(transform);

    assert_eq!(rect_vector.count(), 4);
    assert!((rect_vector.rect_value().width - 3.0).abs() < f64::EPSILON);
    assert!((transform_vector.affine_transform_value().tx - 5.0).abs() < f64::EPSILON);
    assert!(!transform_vector.string_representation().is_empty());
}
