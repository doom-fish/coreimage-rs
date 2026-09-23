mod common;

use std::error::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use coreimage::prelude::*;

const LIBRARY: &[u8] = include_bytes!("fixtures/kernels.metallib");

fn solid(red: f64, green: f64, blue: f64, width: f64, height: f64) -> CIImage {
    CIImage::from_color(&CIColor::rgb(red, green, blue)).cropped_to(CGRect::new(0.0, 0.0, width, height))
}

fn assert_rect_eq(actual: CGRect, expected: CGRect) {
    assert!((actual.origin.x - expected.origin.x).abs() < 1e-9, "{actual:?} != {expected:?}");
    assert!((actual.origin.y - expected.origin.y).abs() < 1e-9, "{actual:?} != {expected:?}");
    assert!((actual.size.width - expected.size.width).abs() < 1e-9, "{actual:?} != {expected:?}");
    assert!((actual.size.height - expected.size.height).abs() < 1e-9, "{actual:?} != {expected:?}");
}

#[test]
fn metal_library_lists_every_kernel_function() {
    let mut names = CIKernel::kernel_names_from_metal_library_data(LIBRARY);
    names.sort();
    assert_eq!(names, ["addBlend", "mixTwo", "shiftRight", "tintTowards"]);
    assert!(CIKernel::kernel_names_from_metal_library_data(&[]).is_empty());
    assert!(CIKernel::kernel_names_from_metal_library_data(&[1, 2, 3]).is_empty());
}

#[test]
fn metal_kernels_load_as_their_declared_class() -> Result<(), Box<dyn Error>> {
    let color = CIColorKernel::from_metal_library_data("tintTowards", LIBRARY, None)?;
    let warp = CIWarpKernel::from_metal_library_data("shiftRight", LIBRARY, None)?;
    let general = CIKernel::from_metal_library_data("mixTwo", LIBRARY, None)?;
    let blend = CIBlendKernel::from_metal_library_data("addBlend", LIBRARY, None)?;
    assert_eq!(color.name(), "tintTowards");
    assert_eq!(warp.name(), "shiftRight");
    assert_eq!(general.name(), "mixTwo");
    assert_eq!(blend.name(), "addBlend");

    let formatted =
        CIColorKernel::from_metal_library_data("tintTowards", LIBRARY, Some(CIFormat::RgbaH))?;
    assert_eq!(formatted.name(), "tintTowards");

    assert!(general.as_color_kernel().is_none());
    assert!(general.as_warp_kernel().is_none());
    let loaded_warp = CIKernel::from_metal_library_data("shiftRight", LIBRARY, None)?;
    assert_eq!(loaded_warp.as_warp_kernel().map(|kernel| kernel.name()).as_deref(), Some("shiftRight"));
    assert!(loaded_warp.as_color_kernel().is_none());
    let loaded_blend = blend.as_kernel();
    assert!(loaded_blend.as_blend_kernel().is_some());
    assert!(loaded_blend.as_color_kernel().is_some());
    assert!(loaded_blend.as_warp_kernel().is_none());
    Ok(())
}

#[test]
fn metal_kernel_loading_reports_errors() {
    let missing = CIKernel::from_metal_library_data("noSuchKernel", LIBRARY, None)
        .expect_err("an unknown function must not load");
    assert!(matches!(missing, CIError::Framework(ref message) if message.contains("does not exist")));

    let wrong_class = CIWarpKernel::from_metal_library_data("tintTowards", LIBRARY, None)
        .expect_err("a color kernel is not a warp kernel");
    assert!(matches!(wrong_class, CIError::Framework(ref message) if message.contains("CIWarpKernel")));

    assert!(matches!(
        CIColorKernel::from_metal_library_data("mixTwo", LIBRARY, None),
        Err(CIError::Framework(_))
    ));
    assert!(matches!(
        CIKernel::from_metal_library_data("mixTwo", &[0, 1, 2, 3], None),
        Err(CIError::Framework(_))
    ));
    assert!(matches!(
        CIKernel::from_metal_library_data("mixTwo", &[], None),
        Err(CIError::InvalidArgument(_))
    ));
    assert!(matches!(
        CIKernel::from_metal_library_data("mix\0Two", LIBRARY, None),
        Err(CIError::InvalidArgument(_))
    ));
}

#[test]
fn color_kernel_binds_scalars_vectors_and_colors() -> Result<(), Box<dyn Error>> {
    let kernel = CIColorKernel::from_metal_library_data("tintTowards", LIBRARY, None)?;
    let context = CIContext::new_default();
    let source = solid(0.0, 1.0, 0.0, 4.0, 4.0);
    let extent = source.extent();
    let red = CIColor::rgb(1.0, 0.0, 0.0);
    let no_offset = CIVector::new4(0.0, 0.0, 0.0, 0.0);
    let blue_offset = CIVector::new4(0.0, 0.0, 1.0, 0.0);

    let untouched = kernel.apply(
        extent,
        &[(&source).into(), 0.0.into(), (&no_offset).into(), (&red).into()],
    )?;
    let tinted = kernel.apply(
        extent,
        &[(&source).into(), 1.0.into(), (&blue_offset).into(), (&red).into()],
    )?;

    for pixel in common::render_rgba8(&context, &untouched, 4, 4) {
        common::assert_pixel_near(pixel, [0, 255, 0, 255]);
    }
    for pixel in common::render_rgba8(&context, &tinted, 4, 4) {
        common::assert_pixel_near(pixel, [255, 0, 255, 255]);
    }
    Ok(())
}

#[test]
fn color_kernel_rejects_arguments_that_do_not_match_its_signature() -> Result<(), Box<dyn Error>> {
    let kernel = CIColorKernel::from_metal_library_data("tintTowards", LIBRARY, None)?;
    let source = solid(0.0, 1.0, 0.0, 4.0, 4.0);
    let red = CIColor::rgb(1.0, 0.0, 0.0);
    let short = CIVector::new(0.0, 0.0);
    let extent = source.extent();

    let cases: [Vec<CIKernelArgument<'_>>; 4] = [
        vec![(&source).into(), 1.0.into()],
        vec![1.0.into(), (&source).into(), (&short).into(), (&red).into()],
        vec![(&source).into(), 1.0.into(), (&short).into(), (&red).into()],
        vec![],
    ];
    for arguments in cases {
        assert!(matches!(
            kernel.apply(extent, &arguments),
            Err(CIError::NullResult(_))
        ));
    }
    Ok(())
}

#[test]
fn general_kernel_samples_every_image_through_its_region_callback() -> Result<(), Box<dyn Error>> {
    let kernel = CIKernel::from_metal_library_data("mixTwo", LIBRARY, None)?;
    let context = CIContext::new_default();
    let red = solid(1.0, 0.0, 0.0, 4.0, 4.0);
    let blue = solid(0.0, 0.0, 1.0, 4.0, 4.0);
    let extent = red.extent();
    let queried = Arc::new(Mutex::new(Vec::new()));

    let mut outputs = Vec::new();
    for amount in [0.0, 1.0] {
        let queried = Arc::clone(&queried);
        outputs.push(kernel.apply(
            extent,
            &[(&red).into(), (&blue).into(), amount.into()],
            move |index, rect| {
                queried.lock().expect("query log").push(index);
                rect
            },
        )?);
    }

    for pixel in common::render_rgba8(&context, &outputs[0], 4, 4) {
        common::assert_pixel_near(pixel, [255, 0, 0, 255]);
    }
    for pixel in common::render_rgba8(&context, &outputs[1], 4, 4) {
        common::assert_pixel_near(pixel, [0, 0, 255, 255]);
    }
    let queried = queried.lock().expect("query log").clone();
    assert!(queried.contains(&0));
    assert!(queried.contains(&1));
    assert!(queried.iter().all(|index| *index < 2));

    let large_red = solid(1.0, 0.0, 0.0, 64.0, 64.0);
    let large_blue = solid(0.0, 0.0, 1.0, 64.0, 64.0);
    let shifted = kernel.apply(
        extent,
        &[(&large_red).into(), (&large_blue).into(), 0.5.into()],
        |index, rect| {
            let offset = if index == 0 { 10.0 } else { 20.0 };
            CGRect::new(rect.origin.x + offset, rect.origin.y, rect.size.width, rect.size.height)
        },
    )?;
    let rect = CGRect::new(1.0, 1.0, 2.0, 2.0);
    assert_rect_eq(
        shifted.region_of_interest_for_image(&large_red, rect),
        CGRect::new(11.0, 1.0, 2.0, 2.0),
    );
    assert_rect_eq(
        shifted.region_of_interest_for_image(&large_blue, rect),
        CGRect::new(21.0, 1.0, 2.0, 2.0),
    );
    Ok(())
}

#[test]
fn general_kernel_region_panics_fall_back_to_each_input_extent() -> Result<(), Box<dyn Error>> {
    let kernel = CIKernel::from_metal_library_data("mixTwo", LIBRARY, None)?;
    let small = solid(1.0, 0.0, 0.0, 4.0, 4.0);
    let large = solid(0.0, 0.0, 1.0, 16.0, 8.0);
    let output = kernel.apply(
        small.extent(),
        &[(&small).into(), (&large).into(), 0.5.into()],
        |_, _| panic!("region callback panic"),
    )?;
    let rect = CGRect::new(1.0, 1.0, 1.0, 1.0);
    assert_rect_eq(output.region_of_interest_for_image(&small, rect), small.extent());
    assert_rect_eq(output.region_of_interest_for_image(&large, rect), large.extent());
    Ok(())
}

#[test]
fn general_kernel_errors_instead_of_crashing_on_bad_arguments() -> Result<(), Box<dyn Error>> {
    let kernel = CIKernel::from_metal_library_data("mixTwo", LIBRARY, None)?;
    let red = solid(1.0, 0.0, 0.0, 4.0, 4.0);
    let extent = red.extent();
    assert!(matches!(
        kernel.apply(extent, &[(&red).into(), 0.5.into()], |_, rect| rect),
        Err(CIError::NullResult(_))
    ));
    assert!(matches!(
        kernel.apply(extent, &[], |_, rect| rect),
        Err(CIError::NullResult(_))
    ));

    let warp_as_general = CIKernel::from_metal_library_data("shiftRight", LIBRARY, None)?;
    assert!(matches!(
        warp_as_general.apply(extent, &[2.0.into()], |_, rect| rect),
        Err(CIError::NullResult(_))
    ));
    Ok(())
}

#[test]
fn warp_kernel_moves_pixels_and_reports_its_region() -> Result<(), Box<dyn Error>> {
    let kernel = CIWarpKernel::from_metal_library_data("shiftRight", LIBRARY, None)?;
    let context = CIContext::new_default();
    let pixels = [255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255];
    let input = CIImage::from_bitmap(&pixels, 4, 1, 16, CIFormat::Rgba8, Some(CIColorSpace::Srgb))?;
    let calls = Arc::new(AtomicUsize::new(0));
    let counted = Arc::clone(&calls);
    let output = kernel.apply(input.extent(), &input, &[1.0.into()], move |_, rect| {
        counted.fetch_add(1, Ordering::SeqCst);
        CGRect::new(rect.origin.x - 1.0, rect.origin.y, rect.size.width, rect.size.height)
    })?;

    let rendered = common::render_rgba8(&context, &output, 4, 1);
    common::assert_pixel_near(rendered[0], [0, 0, 0, 0]);
    common::assert_pixel_near(rendered[1], [255, 0, 0, 255]);
    common::assert_pixel_near(rendered[2], [0, 255, 0, 255]);
    common::assert_pixel_near(rendered[3], [0, 0, 255, 255]);
    assert!(calls.load(Ordering::SeqCst) > 0);
    assert_rect_eq(
        output.region_of_interest_for_image(&input, CGRect::new(2.0, 0.0, 2.0, 1.0)),
        CGRect::new(1.0, 0.0, 2.0, 1.0),
    );
    assert!(matches!(
        kernel.apply(input.extent(), &input, &[], |_, rect| rect),
        Err(CIError::NullResult(_))
    ));
    Ok(())
}

#[test]
fn blend_kernel_from_a_metal_library_blends() -> Result<(), Box<dyn Error>> {
    let kernel = CIBlendKernel::from_metal_library_data("addBlend", LIBRARY, None)?;
    let output = kernel.apply(&solid(1.0, 0.0, 0.0, 4.0, 4.0), &solid(0.0, 0.0, 1.0, 4.0, 4.0))?;
    for pixel in common::render_rgba8(&CIContext::new_default(), &output, 4, 4) {
        common::assert_pixel_near(pixel, [255, 0, 255, 255]);
    }
    Ok(())
}

#[test]
fn region_closures_are_released_with_their_images() -> Result<(), Box<dyn Error>> {
    let general = CIKernel::from_metal_library_data("mixTwo", LIBRARY, None)?;
    let warp = CIWarpKernel::from_metal_library_data("shiftRight", LIBRARY, None)?;
    let image = solid(1.0, 0.0, 0.0, 4.0, 4.0);
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = |calls: &Arc<AtomicUsize>| {
        let calls = Arc::clone(calls);
        move |_: usize, rect: CGRect| {
            calls.fetch_add(1, Ordering::SeqCst);
            rect
        }
    };
    let (mixed_roi, warped_roi) = (counter(&calls), counter(&calls));
    let (rejected_roi, rejected_warp_roi) = (counter(&calls), counter(&calls));
    assert_eq!(Arc::strong_count(&calls), 5);

    let worker = thread::spawn(move || {
        let rect = CGRect::new(0.0, 0.0, 2.0, 2.0);
        let mixed = general
            .apply(image.extent(), &[(&image).into(), (&image).into(), 0.5.into()], mixed_roi)
            .expect("mixTwo should apply");
        let _ = mixed.region_of_interest_for_image(&image, rect);
        let warped = warp
            .apply(image.extent(), &image, &[1.0.into()], warped_roi)
            .expect("shiftRight should apply");
        let _ = warped.region_of_interest_for_image(&image, rect);
        assert!(general.apply(image.extent(), &[], rejected_roi).is_err());
        assert!(warp.apply(image.extent(), &image, &[], rejected_warp_roi).is_err());
        let copy = mixed.clone();
        drop(mixed);
        assert!((copy.extent().size.width - 4.0).abs() < f64::EPSILON);
        drop((copy, warped));
    });
    worker.join().expect("kernel worker should not panic");
    assert!(calls.load(Ordering::SeqCst) >= 2);
    assert_eq!(Arc::strong_count(&calls), 1);
    Ok(())
}

#[test]
fn kernels_compile_from_metal_source_where_supported() {
    let source = "#include <CoreImage/CoreImage.h>\n\
        using namespace metal;\n\
        [[stitchable]] float4 doubled(coreimage::sample_t s) { return s * 2.0; }\n\
        [[stitchable]] float2 moved(float a, coreimage::destination d) { return d.coord() + float2(a, 0); }\n";
    let kernels = match CIKernel::kernels_from_metal_source(source) {
        Ok(kernels) => kernels,
        Err(CIError::Unsupported(message)) => {
            eprintln!("skipping: {message}");
            return;
        }
        Err(error) => panic!("Metal source should compile: {error}"),
    };
    let mut names = kernels.iter().map(CIKernel::name).collect::<Vec<_>>();
    names.sort();
    assert_eq!(names, ["doubled", "moved"]);
    for kernel in &kernels {
        match kernel.name().as_str() {
            "doubled" => assert!(kernel.as_color_kernel().is_some()),
            _ => assert!(kernel.as_warp_kernel().is_some()),
        }
    }
    assert!(matches!(
        CIKernel::kernels_from_metal_source("not metal {"),
        Err(CIError::Framework(_) | CIError::Unsupported(_))
    ));
}
