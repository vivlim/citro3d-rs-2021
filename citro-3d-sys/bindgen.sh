#!/usr/bin/env bash

clang_version=$1

if [ -z "$clang_version" ]; then
    echo "  usage: ./bindgen.sh <clang_version>"
    echo "example: ./bindgen.sh 5.0.0"
    echo "Check your current version with \`clang -v\`."
    exit 1
fi

set -euxo pipefail

bindgen "$DEVKITPRO/libctru/include/citro2d.h" \
    --rust-target nightly \
    --use-core \
    --distrust-clang-mangling \
    --no-layout-tests \
    --ctypes-prefix "::libc" \
    --no-prepend-enum-name \
    --whitelist-type "C2D_.*" \
    --whitelist-function "C2D_.*" \
    --whitelist-var "C2D_.*" \
    --whitelist-type "C3D_.*" \
    --whitelist-function "C3D_.*" \
    --whitelist-var "C3D_.*" \
    --whitelist-type "Mtx_.*" \
    --whitelist-function "Mtx_.*" \
    --whitelist-var "Mtx_.*" \
    --whitelist-type "GX_.*" \
    --whitelist-function "GX_.*" \
    --whitelist-var "GX_.*" \
    --whitelist-type "DVLB_.*" \
    --whitelist-function "DVLB_.*" \
    --whitelist-var "DVLB_.*" \
    --blacklist-type "u(8|16|32|64)" \
    --blacklist-type "__builtin_va_list" \
    --blacklist-type "__va_list" \
    --with-derive-default \
    -- \
    --target=arm-none-eabi \
    --sysroot=$DEVKITARM/arm-none-eabi \
    -isystem$DEVKITARM/arm-none-eabi/include \
    -isystem/usr/lib/clang/$clang_version/include \
    -I$DEVKITPRO/libctru/include \
    -mfloat-abi=hard \
    -march=armv6k \
    -mtune=mpcore \
    -mfpu=vfp \
    -DARM11 \
    -D_3DS \
> src/bindings.rs