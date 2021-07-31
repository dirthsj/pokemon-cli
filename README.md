# System requirements
- x86_64 Ubuntu (other debian variants may work, but are not guaranteed)
# Setup
- Install [rustup](https://rustup.rs)
- Install cargo-deb `cargo install cargo-deb`
- Install arm-unknown-linux-gnueabihf toolchain `rustup target add arm-unknown-linux-gnueabihf`
- Install gcc-arm-linux-gnueabihf `sudo apt-get install gcc-arm-linux-gnueabihf`

# Building
## Native
`cargo build`
## Cross
`cargo build --target=arm-unknown-linux-gnueabihf`