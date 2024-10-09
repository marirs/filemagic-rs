# Linux aarch64 Build
PKG_CONFIG_SYSROOT_DIR=/ cargo build --release --target aarch64-unknown-linux-gnu --features=vendored

# Linux x86_64 Build
PKG_CONFIG_SYSROOT_DIR=/ cargo build --release --target x86_64-unknown-linux-gnu --features=vendored

# Darwin aarch64 Build
PKG_CONFIG_SYSROOT_DIR=/ cargo build --release --target aarch64-apple-darwin --features=vendored

# Darwin x86_64 Build
PKG_CONFIG_SYSROOT_DIR=/ cargo build --release --target x86_64-apple-darwin --features=vendored

# Windows x86_64 Build
PKG_CONFIG_SYSROOT_DIR=/ cargo build --release --target x86_64-pc-windows-gnu

# Windows i686 (32bit) Build
PKG_CONFIG_SYSROOT_DIR=/ cargo build --release --target i686-pc-windows-gnu