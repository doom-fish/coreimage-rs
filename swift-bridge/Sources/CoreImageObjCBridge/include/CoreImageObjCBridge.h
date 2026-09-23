#import <CoreImage/CoreImage.h>
#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

BOOL CIXTrySetValueForKey(
    id object,
    id value,
    NSString *key,
    NSError * _Nullable * _Nullable error
);

CIImage * _Nullable CIXTryApplyKernel(
    CIKernel *kernel,
    CGRect extent,
    CIKernelROICallback callback,
    NSArray *arguments,
    NSError * _Nullable * _Nullable error
);

CIImage * _Nullable CIXTryApplyColorKernel(
    CIColorKernel *kernel,
    CGRect extent,
    NSArray *arguments,
    NSError * _Nullable * _Nullable error
);

CIImage * _Nullable CIXTryApplyWarpKernel(
    CIWarpKernel *kernel,
    CGRect extent,
    CIKernelROICallback callback,
    CIImage *image,
    NSArray *arguments,
    NSError * _Nullable * _Nullable error
);

NS_ASSUME_NONNULL_END
