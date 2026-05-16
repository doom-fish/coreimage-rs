use coreimage::prelude::*;

fn main() {
    let vector = CIVector::from_transform(CGAffineTransform::translation(4.0, 6.0));
    let transform = vector.affine_transform_value();
    println!("transform translation = {}, {}", transform.tx, transform.ty);
}
