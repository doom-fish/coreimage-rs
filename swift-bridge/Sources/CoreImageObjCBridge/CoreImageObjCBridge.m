#import "CoreImageObjCBridge.h"

static void CIXStoreException(NSException *exception, NSError * _Nullable * _Nullable error) {
    if (error != NULL) {
        NSString *message = exception.reason ?: exception.name;
        *error = [NSError errorWithDomain:@"CoreImageObjCBridge"
                                     code:1
                                 userInfo:@{NSLocalizedDescriptionKey: message}];
    }
}

BOOL CIXTrySetValueForKey(
    id object,
    id value,
    NSString *key,
    NSError * _Nullable * _Nullable error
) {
    @try {
        [object setValue:value forKey:key];
        return YES;
    } @catch (NSException *exception) {
        CIXStoreException(exception, error);
        return NO;
    }
}

CIImage * _Nullable CIXTryApplyKernel(
    CIKernel *kernel,
    CGRect extent,
    CIKernelROICallback callback,
    NSArray *arguments,
    NSError * _Nullable * _Nullable error
) {
    @try {
        return [kernel applyWithExtent:extent roiCallback:callback arguments:arguments];
    } @catch (NSException *exception) {
        CIXStoreException(exception, error);
        return nil;
    }
}

CIImage * _Nullable CIXTryApplyColorKernel(
    CIColorKernel *kernel,
    CGRect extent,
    NSArray *arguments,
    NSError * _Nullable * _Nullable error
) {
    @try {
        return [kernel applyWithExtent:extent arguments:arguments];
    } @catch (NSException *exception) {
        CIXStoreException(exception, error);
        return nil;
    }
}

CIImage * _Nullable CIXTryApplyWarpKernel(
    CIWarpKernel *kernel,
    CGRect extent,
    CIKernelROICallback callback,
    CIImage *image,
    NSArray *arguments,
    NSError * _Nullable * _Nullable error
) {
    @try {
        return [kernel applyWithExtent:extent roiCallback:callback inputImage:image arguments:arguments];
    } @catch (NSException *exception) {
        CIXStoreException(exception, error);
        return nil;
    }
}
