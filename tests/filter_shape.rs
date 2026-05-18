use coreimage::prelude::*;

#[test]
fn filter_shapes_transform_and_combine() {
    let shape = CIFilterShape::new(CGRect::new(0.0, 0.0, 10.0, 10.0));
    let moved = shape.transform(CGAffineTransform::translation(5.0, 3.0), false);
    let inset = moved.inset(1, 2);
    let union = inset.union_rect(CGRect::new(0.0, 0.0, 4.0, 4.0));
    let intersection = union.intersection(&shape);

    assert!((moved.extent().origin.x - 5.0).abs() < f64::EPSILON);
    assert!(inset.extent().size.width <= moved.extent().size.width);
    assert!(union.extent().size.width >= intersection.extent().size.width);
    assert!(intersection.extent().size.height <= shape.extent().size.height);
}
