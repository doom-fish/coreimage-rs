mod common;

use std::error::Error;

use coreimage::prelude::*;

#[test]
fn context_renders_and_writes_png() -> Result<(), Box<dyn Error>> {
    let context = CIContext::new_default();
    let image = common::solid_image();
    let rendered = context.render_to_cg_image(&image)?;
    let output = common::target_path("context-test.png");

    context.write_png(&image, &output)?;

    assert_eq!(rendered.width(), 64);
    assert_eq!(rendered.height(), 64);
    assert!(output.exists());
    Ok(())
}

#[test]
fn maximum_image_size_queries_report_platform_unavailability() {
    let context = CIContext::new_default();

    assert!(matches!(
        context.input_image_maximum_size(),
        Err(CIError::Unsupported(_))
    ));
    assert!(matches!(
        context.output_image_maximum_size(),
        Err(CIError::Unsupported(_))
    ));
}
