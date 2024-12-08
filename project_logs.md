# 30/11/2024

- move from el3 -> el1
- had to refer to [link](https://s-matyukevich.github.io/raspberry-pi-os/docs/lesson02/rpi-os.html)
- must set up HCR_EL2 (qemu only?)
- memory access issues
- RES1 = reserved, set to 1 to prevent UB


# 01/12/2024
- memory access issue on EL1
- executing mmio_read -> PC went wild to 0xd503201f140003e0
```
(gdb) ni
core::num::{impl#11}::count_ones () at /rustc/eeb90cda1969383f56a2637cbd3037bdf598841c/library/core/src/num/uint_macros.rs:79
0xd503201f140003e0 in ?? ()

0000000000082e84 <_ZN4core3ptr13read_volatile18precondition_check17h3ecfd8b8af5fa566E>:
   ...
   82eb0: 14000001     	b	0x82eb4 <_ZN4core3ptr13read_volatile18precondition_check17h3ecfd8b8af5fa566E+0x30>
   82eb4: 1400000c     	b	0x82ee4 <_ZN4core3ptr13read_volatile18precondition_check17h3ecfd8b8af5fa566E+0x60>
   82eb8: f94007e8     	ldr	x8, [sp, #0x8]
   82ebc: 9e670100     	fmov	d0, x8
   82ec0: 0e205800     	cnt	v0.8b, v0.8b
   82ec4: 2e303800     	uaddlv	h0, v0.8b
   ...
```
- SIMD operations detected on last 2 op...
  - this is probably the reason we had this weird bug
  - EL1 may not be properly configured to use SIMD
- check for simd register usage using `v[0-9]\.`
- Disable SIMD on compilation: [link](https://os.phil-opp.com/disable-simd/) (this is for x86)
- bruh, libcore is already prebuilt? have to create custom target?
  - we have `aarch64-unknown-none-softfloat` to use software floating points
  - `"features": "+v8a,+strict-align,-neon,-fp-armv8",`

# 08/12/2024
- Interrupt setting has an issue in VBAR_EL1 assignment
- I accidentally passed a dereferenced the address
- Timer works now, although there is no clear mapping to real time calc for now.

# 09/12/2024
- UART interrupt implemented, added callbacks (rx interrupt only)
  - test on `diagnostic/uart_interrupt.rs`
- On device execution is broken. Probably starting from EL3 -> EL1 logic
- Debugging without the EL switch shows that it is running on "Hypervisor" mode
  - This means EL2. We should properly adjust our code to also handle starting
    from EL2.
