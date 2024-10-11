#!/bin/bash

set -x

cargo rustc --target=aarch64-unknown-none -- -C link-arg=-Tsrc/aarch64.ld
aarch64-elf-objcopy target/aarch64-unknown-none/debug/osdev -O binary osdev.img
cp osdev.img /Volumes/bootfs/
