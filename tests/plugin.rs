use coreimage::prelude::*;

#[test]
fn plugin_registration_invokes_rust_callback() {
    CIPlugIn::load_non_executable_plugins();

    let registration = CIPlugInRegistration::new(<*mut core::ffi::c_void>::is_null);
    assert!(registration.load(None));
}

#[test]
fn plugin_callback_and_one_release_destructor_panic_are_contained() {
    struct CaptureDropBomb;

    impl Drop for CaptureDropBomb {
        fn drop(&mut self) {
            panic!("plug-in capture destructor");
        }
    }

    let bomb = CaptureDropBomb;
    let registration = CIPlugInRegistration::new(move |_| {
        std::hint::black_box(&bomb);
        panic!("plug-in callback");
    });

    assert!(!registration.load(None));
    drop(registration);
}
