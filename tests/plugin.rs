use coreimage::prelude::*;

#[test]
fn plugin_registration_invokes_rust_callback() {
    CIPlugIn::load_non_executable_plugins();

    let registration = CIPlugInRegistration::new(<*mut core::ffi::c_void>::is_null);
    assert!(registration.load(None));
}
