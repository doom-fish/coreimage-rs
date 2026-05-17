use std::error::Error;

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
