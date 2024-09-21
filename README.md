# Setup

## Probe-rs
```html
https://probe.rs/docs/getting-started/installation/
```

## Packages
```shell
sudo apt-get install   gdb-multiarch   minicom   openocd

rustup target add thumbv7em-none-eabi

rustup component add llvm-tools

cargo install itm
cargo install cargo-binutils
```

# Build & Flash
```shell
cargo build --release
```

```shell
cargo flash --chip stm32f411vetx --release
```