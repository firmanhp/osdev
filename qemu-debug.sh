#!/bin/bash

# Connect GDB to localhost:1234
# (gdb) target remote localhost:1234

qemu-system-aarch64 \
  -s -S \
  -nographic \
  -M raspi3b \
  -kernel target/aarch64-unknown-none/debug/osdev
  # -device loader,file=target/aarch64-unknown-none/debug/osdev,addr=0x80000,cpu-num=0
  # -kernel out/img/kernel7.img
  # -kernel out/img/myos.elf
