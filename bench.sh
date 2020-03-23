#!/bin/sh
set -ex
cargo build --release
dd if=/dev/zero bs=1000000 count=10000 | target/release/jni_stream_bench native >/dev/null
dd if=/dev/zero bs=1000000 count=10000 | target/release/jni_stream_bench java-in >/dev/null
dd if=/dev/zero bs=1000000 count=10000 | target/release/jni_stream_bench java-out >/dev/null
 
