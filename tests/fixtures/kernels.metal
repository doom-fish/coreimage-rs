#include <CoreImage/CoreImage.h>

using namespace metal;

extern "C" {
namespace coreimage {

float4 tintTowards(sample_t source, float amount, float4 offset, float4 color) {
    return float4(mix(source.rgb, color.rgb, amount) + offset.rgb, source.a);
}

float2 shiftRight(float amount, destination dest) {
    return dest.coord() - float2(amount, 0.0);
}

float4 mixTwo(sampler first, sampler second, float amount, destination dest) {
    float4 a = first.sample(first.transform(dest.coord()));
    float4 b = second.sample(second.transform(dest.coord()));
    return mix(a, b, amount);
}

float4 addBlend(sample_t foreground, sample_t background) {
    return foreground + background;
}

}
}
