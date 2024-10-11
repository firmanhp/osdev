# OSdev

This is a toy operating system running on bare metal Raspberry Pi 3 Model B.
It could run on Raspberry Pi 4, but I don't have one to test.


## Building

This source builds on target `aarch64-unknown-none`. For more information, refer to `.cargo/config.toml`.

To add the target, run
```
rustup target add aarch64-unknown-none
```

## For QEMU

Run `cargo build_device`. ELF output will be located at
`target/aarch64-unknown-none/debug/osdev`.

Then, execute QEMU:
```
qemu-system-aarch64 \
  -nographic \
  -M raspi3b \
  -kernel target/aarch64-unknown-none/debug/osdev
```

## For bare metal

Install required dependencies
```
cargo install cargo-binutils
rustup component add llvm-tools
```

Build, and strip image for bare metal:
```
cargo build_device_img
```

The image will be on the root project.

## Testing

We have some on-host tests especially for data structures. Run test with

```
cargo test
```
