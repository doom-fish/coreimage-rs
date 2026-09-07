mod common;

use std::error::Error;
use std::sync::{Arc, Barrier, Mutex, MutexGuard, OnceLock};
use std::thread;

use coreimage::prelude::*;

fn processor_test_guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .expect("processor test lock should not be poisoned")
}

#[test]
fn passthrough_processor_preserves_extent() -> Result<(), Box<dyn Error>> {
    let _guard = processor_test_guard();
    let image = common::solid_image();
    let output = CIImageProcessor::apply_passthrough(&image)?;
    let invocation = CIImageProcessor::last_invocation();
    let image_extent = image.extent();

    assert!((output.extent().size.width - image_extent.size.width).abs() < f64::EPSILON);
    assert!(CIImageProcessor::last_invocation_json().starts_with('{'));
    assert_eq!(invocation.input_count(), 1);
    let input = invocation
        .input()
        .expect("expected image processor input snapshot");
    assert!(input.bytes_per_row() > 0);
    assert!((input.region().size.width - image_extent.size.width).abs() < f64::EPSILON);
    assert!(
        (invocation.output().region().size.height - image_extent.size.height).abs() < f64::EPSILON
    );
    Ok(())
}

#[test]
fn concurrent_processor_snapshots_never_mix_invocations() {
    let _guard = processor_test_guard();
    let dimensions = [(17.0, 29.0), (31.0, 13.0), (47.0, 19.0), (23.0, 41.0)];
    let barrier = Arc::new(Barrier::new(dimensions.len()));
    let threads = dimensions
        .into_iter()
        .map(|(width, height)| {
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                let image = CIImage::from_color(&CIColor::rgb(0.2, 0.4, 0.8))
                    .cropped_to(CGRect::new(0.0, 0.0, width, height));
                barrier.wait();
                for _ in 0..24 {
                    CIImageProcessor::apply_passthrough(&image)
                        .expect("passthrough processing should succeed");
                    let invocation = CIImageProcessor::last_invocation();
                    let input = invocation
                        .input()
                        .expect("every recorded invocation should have an input");
                    let output = invocation.output();

                    assert_eq!(invocation.input_count(), 1);
                    assert!(
                        (input.region().size.width - output.region().size.width).abs()
                            < f64::EPSILON
                    );
                    assert!(
                        (input.region().size.height - output.region().size.height).abs()
                            < f64::EPSILON
                    );
                    assert_eq!(input.format_raw(), output.format_raw());
                }
            })
        })
        .collect::<Vec<_>>();

    for thread in threads {
        thread.join().expect("processor worker should not panic");
    }
}
