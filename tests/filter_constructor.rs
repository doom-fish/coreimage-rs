use std::error::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

use coreimage::prelude::*;

#[test]
fn rust_filter_constructor_registers_custom_filters() -> Result<(), Box<dyn Error>> {
    let constructor = CIFilterConstructor::new(|name| {
        let mut filter = CIFilter::new("CIColorInvert")?;
        filter.set_name(name);
        Some(filter)
    });

    let direct = constructor
        .filter_with_name("CoreImageRsTestInvert")
        .expect("constructor should create a filter");
    assert_eq!(direct.name(), "CoreImageRsTestInvert");

    let custom_name = "CoreImageRsRegisteredInvert";
    CIFilter::register_filter_name(custom_name, &constructor, Some("Registered invert"))?;
    let registered = CIFilter::new(custom_name).expect("registered filter should exist");
    assert_eq!(registered.name(), custom_name);
    Ok(())
}

#[test]
fn registered_constructor_handles_concurrent_framework_callbacks() -> Result<(), Box<dyn Error>> {
    let calls = Arc::new(AtomicUsize::new(0));
    let callback_calls = Arc::clone(&calls);
    let constructor = CIFilterConstructor::new(move |name| {
        callback_calls.fetch_add(1, Ordering::Relaxed);
        let mut filter = CIFilter::new("CIColorInvert")?;
        filter.set_name(name);
        Some(filter)
    });
    let name = "CoreImageRsConcurrentRegisteredInvert";
    CIFilter::register_filter_name(name, &constructor, None)?;

    let threads = (0..8)
        .map(|_| {
            thread::spawn(move || {
                for _ in 0..32 {
                    let filter = CIFilter::new(name).expect("registered filter should exist");
                    assert_eq!(filter.name(), name);
                }
            })
        })
        .collect::<Vec<_>>();
    for thread in threads {
        thread.join().expect("constructor worker should not panic");
    }

    assert!(calls.load(Ordering::Relaxed) >= 8 * 32);
    Ok(())
}

struct PanicOnDrop;

impl Drop for PanicOnDrop {
    fn drop(&mut self) {
        panic!("panic payload destructor");
    }
}

#[test]
fn constructor_callback_contains_hostile_panic_payloads() {
    let constructor = CIFilterConstructor::new(|_| {
        std::panic::panic_any(PanicOnDrop);
    });

    assert!(constructor.filter_with_name("CoreImageRsPanic").is_none());
}

#[test]
fn constructor_release_contains_one_capture_destructor_panic() {
    struct CaptureDropBomb;

    impl Drop for CaptureDropBomb {
        fn drop(&mut self) {
            panic!("capture destructor");
        }
    }

    let bomb = CaptureDropBomb;
    let constructor = CIFilterConstructor::new(move |_| {
        std::hint::black_box(&bomb);
        None
    });
    drop(constructor);
}
