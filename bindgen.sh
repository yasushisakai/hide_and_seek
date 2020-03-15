# compile with target architecture
RUSTFLAGS="-Z embed-bitcode" cargo +ios-arm64 build --target aarch64-apple-ios --release --lib

# make header files
cbindgen src/lib.rs -l c > hide_and_seek.h

# move to location
