#!/bin/bash

qemu-system-aarch64 \
  -nographic \
  -M raspi3b \
  -device loader,file=osdev.img,addr=0x80000,cpu-num=0
