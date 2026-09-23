mod common;

use std::error::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, Mutex, MutexGuard, OnceLock};
use std::thread;

use apple_cf::cg::CGContext;
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

fn red_image(width: f64, height: f64) -> CIImage {
    CIImage::from_color(&CIColor::rgb(1.0, 0.0, 0.0)).cropped_to(CGRect::new(0.0, 0.0, width, height))
}

fn assert_rect_eq(actual: CGRect, expected: CGRect) {
    assert!((actual.origin.x - expected.origin.x).abs() < 1e-9, "{actual:?} != {expected:?}");
    assert!((actual.origin.y - expected.origin.y).abs() < 1e-9, "{actual:?} != {expected:?}");
    assert!((actual.size.width - expected.size.width).abs() < 1e-9, "{actual:?} != {expected:?}");
    assert!((actual.size.height - expected.size.height).abs() < 1e-9, "{actual:?} != {expected:?}");
}

#[derive(Clone, Copy, Debug)]
struct Observed {
    format: CIFormat,
    input_width: usize,
    input_height: usize,
    output_width: usize,
    output_height: usize,
    input_len: usize,
    input_bytes_per_row: usize,
}

#[test]
fn closure_processor_rewrites_every_output_row() -> Result<(), Box<dyn Error>> {
    let observed = Arc::new(Mutex::new(Vec::new()));
    let log = Arc::clone(&observed);
    let kernel = CIImageProcessorKernel::new(move |inputs, output| {
        let input = inputs.first().ok_or("missing input")?;
        log.lock().map_err(|_| "poisoned log")?.push(Observed {
            format: input.format(),
            input_width: input.width(),
            input_height: input.height(),
            output_width: output.width(),
            output_height: output.height(),
            input_len: input.bytes().len(),
            input_bytes_per_row: input.bytes_per_row(),
        });
        if input.row(input.height()).is_some() || output.row(output.height()).is_some() {
            return Err("rows past the region must not exist".to_string());
        }
        for y in 0..output.height() {
            let source = input.row(y).ok_or("input row missing")?;
            let target = output.row_mut(y).ok_or("output row missing")?;
            for (pixel, from) in target.chunks_exact_mut(4).zip(source.chunks_exact(4)) {
                pixel.copy_from_slice(&[255 - from[0], 255 - from[1], 255 - from[2], from[3]]);
            }
        }
        Ok(())
    })
    .with_format(CIFormat::Bgra8)?;
    let image = red_image(7.0, 5.0);
    let output = kernel.apply(image.extent(), &[&image])?;

    for pixel in common::render_rgba8(&CIContext::new_default(), &output, 7, 5) {
        common::assert_pixel_near(pixel, [0, 255, 255, 255]);
    }
    let observed = observed.lock().expect("observation log").clone();
    assert!(!observed.is_empty());
    for call in observed {
        assert_eq!(call.format, CIFormat::Bgra8);
        assert_eq!((call.input_width, call.input_height), (call.output_width, call.output_height));
        assert!(call.input_bytes_per_row >= call.input_width * 4);
        assert_eq!(
            call.input_len,
            call.input_bytes_per_row * (call.input_height - 1) + call.input_width * 4
        );
    }
    Ok(())
}

fn drawn_rgba8(image: &CGImage, width: usize, height: usize, extent: f64) -> Vec<u8> {
    let canvas = CGContext::new_rgba8(width, height).expect("bitmap context");
    canvas.clear_rect(0.0, 0.0, extent, extent);
    canvas.draw_image(0.0, 0.0, extent, extent, image);
    unsafe { canvas.as_bytes() }.to_vec()
}

#[test]
fn closure_errors_and_panics_fail_the_render_and_clear_the_tile() -> Result<(), Box<dyn Error>> {
    let image = red_image(4.0, 4.0);
    let context = CIContext::new_default();
    let failing = [
        (
            CIImageProcessorKernel::new(|_, output| {
                output.bytes_mut().fill(0xff);
                Err("refused by the closure".to_string())
            }),
            "refused by the closure",
        ),
        (
            CIImageProcessorKernel::new(|_, output| {
                output.bytes_mut().fill(0xff);
                panic!("processor closure panic")
            }),
            "panicked",
        ),
    ];
    for (kernel, message) in failing {
        let output = kernel.with_format(CIFormat::Bgra8)?.apply(image.extent(), &[&image])?;
        let mut destination = CIRenderDestination::bitmap_rgba8(4, 4)?;
        let rendered = context
            .start_render_task(&output, &mut destination)?
            .wait_until_completed();
        assert!(
            matches!(rendered, Err(CIError::Framework(ref text)) if text.contains(message)),
            "{rendered:?}"
        );
        if let Ok(cg_image) = context.render_to_cg_image(&output) {
            assert!(drawn_rgba8(&cg_image, 4, 4, 4.0).iter().all(|byte| *byte == 0));
        }
    }
    Ok(())
}

#[test]
fn closure_processor_generates_without_inputs() -> Result<(), Box<dyn Error>> {
    let kernel = CIImageProcessorKernel::new(|inputs, output| {
        if !inputs.is_empty() {
            return Err("a generator has no inputs".to_string());
        }
        for y in 0..output.height() {
            output.row_mut(y).ok_or("output row missing")?.fill(255);
        }
        Ok(())
    })
    .with_format(CIFormat::Bgra8)?;
    let output = kernel.apply(CGRect::new(0.0, 0.0, 3.0, 2.0), &[])?;
    for pixel in common::render_rgba8(&CIContext::new_default(), &output, 3, 2) {
        common::assert_pixel_near(pixel, [255, 255, 255, 255]);
    }
    Ok(())
}

#[test]
fn closure_processor_honors_each_supported_format() -> Result<(), Box<dyn Error>> {
    let image = red_image(4.0, 3.0);
    let context = CIContext::new_default();
    for format in [
        CIFormat::Bgra8,
        CIFormat::RgbaH,
        CIFormat::RgbaF,
        CIFormat::R8,
        CIFormat::RH,
        CIFormat::RF,
    ] {
        let observed = Arc::new(Mutex::new(Vec::new()));
        let log = Arc::clone(&observed);
        let kernel = CIImageProcessorKernel::new(move |inputs, output| {
            let input = inputs.first().ok_or("missing input")?;
            let fits = output.bytes_per_row() >= output.width() * output.format().bytes_per_pixel();
            log.lock()
                .map_err(|_| "poisoned log")?
                .push((input.format(), output.format(), fits));
            Ok(())
        })
        .with_format(format)?;
        assert_eq!(kernel.format(), Some(format));
        let output = kernel.apply(image.extent(), &[&image])?;
        let _ = common::render_rgba8(&context, &output, 4, 3);
        let observed = observed.lock().expect("observation log").clone();
        assert!(!observed.is_empty(), "{format:?} never ran");
        assert!(observed.iter().all(|call| *call == (format, format, true)), "{format:?}: {observed:?}");
    }

    for format in [CIFormat::Rgba8, CIFormat::Argb8, CIFormat::L8, CIFormat::Rgba16] {
        assert!(matches!(
            CIImageProcessorKernel::new(|_, _| Ok(())).with_format(format),
            Err(CIError::InvalidArgument(_))
        ));
    }
    let default_format = Arc::new(Mutex::new(None));
    let slot = Arc::clone(&default_format);
    let kernel = CIImageProcessorKernel::new(move |_, output| {
        *slot.lock().map_err(|_| "poisoned slot")? = Some(output.format());
        Ok(())
    });
    assert_eq!(kernel.format(), None);
    let output = kernel.apply(image.extent(), &[&image])?;
    let _ = common::render_rgba8(&context, &output, 4, 3);
    let chosen = default_format.lock().expect("format slot").take();
    assert!(chosen.is_some_and(|format| format.bytes_per_pixel() > 0));
    Ok(())
}

#[test]
fn closure_processor_region_of_interest_and_panic_fallback() -> Result<(), Box<dyn Error>> {
    let image = red_image(64.0, 64.0);
    let extent = CGRect::new(0.0, 0.0, 32.0, 32.0);
    let rect = CGRect::new(4.0, 4.0, 8.0, 8.0);

    let default = CIImageProcessorKernel::new(|_, _| Ok(())).apply(extent, &[&image])?;
    assert_rect_eq(default.region_of_interest_for_image(&image, rect), rect);

    let shifted = CIImageProcessorKernel::new(|_, _| Ok(()))
        .with_region_of_interest(|_, rect| {
            CGRect::new(rect.origin.x + 1.0, rect.origin.y + 2.0, rect.size.width, rect.size.height)
        })
        .apply(extent, &[&image])?;
    assert_rect_eq(
        shifted.region_of_interest_for_image(&image, rect),
        CGRect::new(5.0, 6.0, 8.0, 8.0),
    );

    let panicking = CIImageProcessorKernel::new(|_, _| Ok(()))
        .with_region_of_interest(|_, _| panic!("region callback panic"))
        .apply(extent, &[&image])?;
    assert_rect_eq(panicking.region_of_interest_for_image(&image, rect), image.extent());
    Ok(())
}

#[test]
fn closure_processor_state_is_released_with_its_images() {
    let calls = Arc::new(AtomicUsize::new(0));
    let counted = Arc::clone(&calls);
    let kernel = CIImageProcessorKernel::new(move |_, _| {
        counted.fetch_add(1, Ordering::SeqCst);
        Ok(())
    })
    .with_region_of_interest(|_, rect| rect);
    let image = red_image(4.0, 4.0);
    let worker = thread::spawn(move || {
        let output = kernel
            .apply(image.extent(), &[&image])
            .expect("processor should apply");
        let _ = common::render_rgba8(&CIContext::new_default(), &output, 4, 4);
        let empty = kernel
            .apply(CGRect::new(0.0, 0.0, 0.0, 0.0), &[&image])
            .expect("an empty extent yields an empty image");
        assert!(empty.extent().size.width.abs() < f64::EPSILON);
        let copies = (output.clone(), kernel.clone());
        drop((output, empty, kernel, copies));
    });
    worker.join().expect("processor worker should not panic");
    assert!(calls.load(Ordering::SeqCst) > 0);
    assert_eq!(Arc::strong_count(&calls), 1);
}

#[test]
fn closure_processor_images_render_on_other_threads() -> Result<(), Box<dyn Error>> {
    let kernel = CIImageProcessorKernel::new(|inputs, output| {
        let input = inputs.first().ok_or("missing input")?;
        for y in 0..output.height() {
            let source = input.row(y).ok_or("input row missing")?;
            output.row_mut(y).ok_or("output row missing")?.copy_from_slice(source);
        }
        Ok(())
    })
    .with_format(CIFormat::Bgra8)?;
    let image = red_image(6.0, 6.0);
    let output = Arc::new(kernel.apply(image.extent(), &[&image])?);
    let context = Arc::new(CIContext::new_default());
    let workers = (0..3)
        .map(|_| {
            let output = Arc::clone(&output);
            let context = Arc::clone(&context);
            thread::spawn(move || common::render_rgba8(&context, &output, 6, 6))
        })
        .collect::<Vec<_>>();
    for worker in workers {
        for pixel in worker.join().expect("render worker should not panic") {
            common::assert_pixel_near(pixel, [255, 0, 0, 255]);
        }
    }
    Ok(())
}
