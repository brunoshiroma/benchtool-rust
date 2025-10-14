# Compiling to android +35 api

Android api +35 requires 16kb page files

```bash
# download last ndk https://developer.android.com/ndk/downloads

wget https://dl.google.com/android/repository/android-ndk-r29-linux.zip
unzip android-ndk-r29-linux.zip
export ANDROID_NDK_HOME=/workspaces/benchtool-rust/android-ndk-r29

cargo ndk build --target aarch64-linux-android --release

rustup target add x86_64-linux-android
rustup target add aarch64-linux-android

cargo install cargo-ndk

cargo ndk build --target aarch64-linux-android --release
cargo ndk build --target x86_64-linux-android --release

```