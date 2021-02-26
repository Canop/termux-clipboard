# compile the examples for android using cargo cross

cross build --example print-clipboard --target aarch64-linux-android --release
cross build --example fill-clipboard --target aarch64-linux-android --release

# copy them to a build directory for easier discovery

mkdir build
cp target/aarch64-linux-android/release/examples/print-clipboard build/
cp target/aarch64-linux-android/release/examples/fill-clipboard build/
