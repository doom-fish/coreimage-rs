use coreimage::prelude::*;

#[test]
fn filter_shapes_transform_and_combine() {
    let shape = CIFilterShape::new(CGRect::new(0.0, 0.0, 10.0, 10.0));
    let moved = shape.transform(CGAffineTransform::translation(5.0, 3.0), false);
    let inset = moved.inset(1, 2);
    let union = inset.union_rect(CGRect::new(0.0, 0.0, 4.0, 4.0));
    let intersection = union.intersection(&shape);

    assert!((moved.extent().x - 5.0).abs() < f64::EPSILON);
    assert!(inset.extent().width <= moved.extent().width);
    assert!(union.extent().width >= intersection.extent().width);
    assert!(intersection.extent().height <= shape.extent().height);
}
