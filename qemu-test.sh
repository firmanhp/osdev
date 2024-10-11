#!/bin/bash

qemu-system-aarch64 \
  -nographic \
  -M raspi3b \
  -kernel target/aarch64-unknown-none/debug/osdev
