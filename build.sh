# Add components for Rust.
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
rustup target add x86_64-pc-windows-gnu

# Build for each architecture.

echo "Compiling for ARM64 Apple."
cargo build --release --target=aarch64-apple-darwin

echo "Compiling for Intel x86-64 Apple."
cargo build --release --target=x86_64-apple-darwin

echo "Compiling for Intel x86-64 Windows."
cargo build --release --target=x86_64-pc-windows-gnu

# Move to "dist" folder and rename.

mkdir dist

mv target/aarch64-apple-darwin/release/cf dist
mv dist/cf dist/cflek-v.1.2.0-arm64-apple.bin

mv target/x86_64-apple-darwin/release/cf dist
mv dist/cf dist/cflek-v.1.2.0-x86_64-apple.bin

mv target/x86_64-pc-windows-gnu/release/cf.exe dist
mv dist/cf.exe dist/cflek-v.1.2.0-x86_64-windows.exe