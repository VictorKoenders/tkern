// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2021-2022 Andre Richter <andre.o.richter@gmail.com>

//--------------------------------------------------------------------------------------------------
// Definitions
//--------------------------------------------------------------------------------------------------

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
.org 0x80000
.section .text._start

//------------------------------------------------------------------------------
// fn _start()
//------------------------------------------------------------------------------
_start:
	// Initialize DRAM.
	ldr x6, =__bss_size
	// early on in development we might have a kernel with no BSS, so we check for that here
	cbz x6, .boot_prepare_rust

	ldr	x5, =__bss_start
.boot_bss_init_loop:
	sub x6, x6, #1
	str XZR, [x5, x6]

	cbnz x6, .boot_bss_init_loop

	// Prepare the jump to Rust code.
.boot_prepare_rust:
	// Set the stack pointer.
	adrp x5, _start
	add x5, x5, #:lo12:_start
	mov	 sp, x5

	ldr x1, __kernel_end
	// Jump to Rust code.
	b	_start_rust

	// Infinitely wait for events (aka "park the core").
.boot_parking_loop:
	wfe
	b	.boot_parking_loop

.size	_start, . - _start
.type	_start, function
