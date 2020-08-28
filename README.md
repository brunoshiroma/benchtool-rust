# Benchtool Rust
[![Build Status](https://travis-ci.com/brunoshiroma/benchtool-rust.svg?branch=master)](https://travis-ci.com/brunoshiroma/benchtool-rust)

Simple benchmark tool written in Rust

### Compile for android
https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html
https://medium.com/visly/rust-on-android-19f34a2fb43
https://github.com/mozilla/rust-android-gradle

On linux create file ~/.cargo/config
```
[target.aarch64-linux-android]
linker = "[NDK]/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang"
ar = "[NDK]/toolchains/llvm/prebuilt/linux-x86_64/aarch64-linux-android/bin/ar"

[target.i686-linux-android]
linker = "[NDK]/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android21-clang"
ar = "[NDK]/toolchains/llvm/prebuilt/linux-x86_64/i686-linux-android/bin/ar"

[target.armv7-linux-androideabi]
.
.

[target.x86_64-linux-android]
.
.
```

### Example for building for android
```sh
cargo build --target aarch64-linux-android --release
cargo build --target i686-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target x86_64-linux-android --release
```