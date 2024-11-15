// Boot entry point for ARM64.
// Assumes MMU off.
// BSS must be zeroed out.
//
// AArch64 mode

// To keep this in the first portion of the binary.
.section ".text.boot"

// Make _start global.
.globl _start
.globl _halt

// Entry point for the kernel. Registers:
// x0 -> 32 bit pointer to DTB in memory (primary core only) / 0 (secondary cores)
// x1 -> 0
// x2 -> 0
// x3 -> 0
// x4 -> 32 bit kernel entry point, _start location
_start:
    // https://forums.raspberrypi.com/viewtopic.php?t=273010
    // read cpu id, stop slave cores
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, 2f
    // cpu id > 0, stop
    b       _halt
2:  // cpu id == 0

    // set stack before our code
    ldr     x5, =_start
    mov     sp, x5

    // clear bss
    ldr     x5, =__bss_start
    ldr     w6, =__bss_size
1:  cbz     w6, 2f
    str     xzr, [x5], #8
    sub     w6, w6, #1
    cbnz    w6, 1b

    // jump to Rust code
2:  bl      arch_setup
    bl      board_setup
    // should not return
    bl      kernel_main
    // for failsafe, halt this core
_halt:
    wfe
    b _halt