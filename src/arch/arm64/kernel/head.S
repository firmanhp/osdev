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
// Assumes that this is running from EL3
_start:
    // https://forums.raspberrypi.com/viewtopic.php?t=273010
    // read cpu id, stop slave cores
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, 2f
    // cpu id > 0, stop
    b       _halt
    // cpu id == 0
2:  bl _init_kernel_el // Move to EL1

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

// SCTLR_ELn
// https://developer.arm.com/documentation/ddi0595/2020-12/AArch64-Registers/SCTLR-EL1--System-Control-Register--EL1-?lang=en
// SPSR_ELn
// https://developer.arm.com/documentation/ddi0595/2021-03/AArch64-Registers/SPSR-EL3--Saved-Program-Status-Register--EL3-
// ELR_ELn
// https://developer.arm.com/documentation/ddi0601/2024-09/AArch64-Registers/ELR-EL3--Exception-Link-Register--EL3-
// from
// https://github.com/s-matyukevich/raspberry-pi-os/blob/master/src/lesson03/include/arm/sysregs.h
// RES1 = reserved, set to 1

.equ SPSR_D,                (1 << 6)
.equ SPSR_A,                (1 << 7)
.equ SPSR_I,                (1 << 8)
.equ SPSR_F,                (1 << 9)
.equ SPSR_MODE_EL1h,        ((1 << 2) | 1)
.equ INIT_PSTATE_EL1,       (SPSR_D | SPSR_A | SPSR_I | SPSR_F | SPSR_MODE_EL1h)

.equ SCR_RESERVED,	        (3 << 4)
.equ SCR_EL3_RW,            (1 << 10)   // aarch64
.equ SCR_EL3_NS,            (1 << 0)    // non-secure
.equ INIT_SCR_EL3,          (SCR_RESERVED | SCR_EL3_RW | SCR_EL3_NS)

.equ HCR_RW,	    	    (1 << 31)   // <EL2 in aarch64
.equ INIT_HCR_EL2,		    HCR_RW

.equ SCTLR_RESERVED,                  (3 << 28) | (3 << 22) | (1 << 20) | (1 << 11)
.equ SCTLR_EE_LITTLE_ENDIAN,          (0 << 25)
.equ SCTLR_EOE_LITTLE_ENDIAN,         (0 << 24)
.equ SCTLR_I_CACHE_DISABLED,          (0 << 12)
.equ SCTLR_D_CACHE_DISABLED,          (0 << 2)
.equ SCTLR_MMU_DISABLED,              (0 << 0)
.equ SCTLR_MMU_ENABLED,               (1 << 0)
.equ INIT_SCTLR,                      (SCTLR_RESERVED | SCTLR_EE_LITTLE_ENDIAN | SCTLR_I_CACHE_DISABLED | SCTLR_D_CACHE_DISABLED | SCTLR_MMU_DISABLED)

// Initialize in EL1
// https://github.com/torvalds/linux/blob/master/arch/arm64/kernel/head.S#L275
// might need to set SCTLR_EL1 later on...
_init_kernel_el:
    // at some point maybe need to handle this in hypervisor mode.
    // mrs x1, CurrentEL
    // cmp x1, #CurrentEL_EL2
    // b.eq init_el2
    
	msr	sctlr_el1, x0		

    // this is apparently needed
	ldr	x0, =INIT_HCR_EL2
	msr	hcr_el2, x0

	ldr	x0, =INIT_SCR_EL3
	msr	scr_el3, x0

	ldr	x0, =INIT_PSTATE_EL1
	msr	spsr_el3, x0

	// Set the return pointer to after this subtroutine
    mov x0, lr		
	msr	elr_el3, x0

    // Maybe we might want to return the result later?
    // mov w0, #BOOT_CPU_MODE_EL1
    isb
    eret
