#import "CoreImageObjCBridge.h"

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
        if (error != NULL) {
            NSString *message = exception.reason ?: exception.name;
            *error = [NSError errorWithDomain:@"CoreImageObjCBridge"
                                         code:1
                                     userInfo:@{NSLocalizedDescriptionKey: message}];
        }
        return NO;
    }
}
