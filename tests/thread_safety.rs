mod common;

use std::sync::Arc;
use std::thread;

use coreimage::prelude::*;

const fn assert_send_sync<T: Send + Sync>() {}

struct Invalid;

trait AmbiguousIfSend<A> {
    fn check() {}
}
impl<T: ?Sized> AmbiguousIfSend<()> for T {}
impl<T: ?Sized + Send> AmbiguousIfSend<Invalid> for T {}

trait AmbiguousIfSync<A> {
    fn check() {}
}
impl<T: ?Sized> AmbiguousIfSync<()> for T {}
impl<T: ?Sized + Sync> AmbiguousIfSync<Invalid> for T {}

#[test]
fn documented_thread_safe_types_are_send_and_sync() {
    assert_send_sync::<CIImage>();
    assert_send_sync::<CIContext>();
    assert_send_sync::<CIColor>();
    assert_send_sync::<CIVector>();
    assert_send_sync::<CIKernel>();
    assert_send_sync::<CIColorKernel>();
    assert_send_sync::<CIWarpKernel>();
    assert_send_sync::<CIBlendKernel>();
}

#[test]
fn mutable_types_stay_on_their_thread() {
    <CIFilter as AmbiguousIfSend<_>>::check();
    <CIFilter as AmbiguousIfSync<_>>::check();
    <CIImageAccumulator as AmbiguousIfSend<_>>::check();
    <CIImageAccumulator as AmbiguousIfSync<_>>::check();
    <CIRenderDestination as AmbiguousIfSend<_>>::check();
    <CIRenderDestination as AmbiguousIfSync<_>>::check();
    <CIRAWFilter as AmbiguousIfSend<_>>::check();
    <CIRAWFilter as AmbiguousIfSync<_>>::check();
    <CIFilterGenerator as AmbiguousIfSend<_>>::check();
    <CIFilterGenerator as AmbiguousIfSync<_>>::check();
}

#[test]
fn one_context_renders_one_image_on_several_threads() {
    let context = Arc::new(CIContext::new_default());
    let blend = Arc::new(CIBlendKernel::built_in(CIBlendKernelKind::SourceOver));
    let foreground = CIImage::from_color(&CIColor::rgba(1.0, 0.0, 0.0, 0.5))
        .cropped_to(CGRect::new(0.0, 0.0, 8.0, 8.0));
    let image = Arc::new(
        blend
            .apply(&foreground, &common::solid_image())
            .expect("source-over blend should apply"),
    );
    let expected = common::render_rgba8(&context, &image, 8, 8);
    assert!(expected.iter().all(|pixel| pixel[3] == 255));

    let workers = (0..4)
        .map(|_| {
            let context = Arc::clone(&context);
            let image = Arc::clone(&image);
            let blend = Arc::clone(&blend);
            thread::spawn(move || {
                let again = blend
                    .apply(&image, &CIImage::from_color(&CIColor::clear()))
                    .expect("blending over clear should apply");
                (
                    common::render_rgba8(&context, &image, 8, 8),
                    common::render_rgba8(&context, &again, 8, 8),
                )
            })
        })
        .collect::<Vec<_>>();

    for worker in workers {
        let (direct, reblended) = worker.join().expect("render worker should not panic");
        assert_eq!(direct, expected);
        assert_eq!(reblended, expected);
    }
}
