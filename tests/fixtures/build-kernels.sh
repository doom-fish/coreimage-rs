#!/bin/sh
set -eu
cd "$(dirname "$0")"
xcrun metal -fcikernel -mmacosx-version-min=11.0 -c kernels.metal -o kernels.air
xcrun metallib -cikernel kernels.air -o kernels.metallib
rm kernels.air
