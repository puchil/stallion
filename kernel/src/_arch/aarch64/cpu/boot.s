// Load the address of a symbol into a register, PC-relative
// The Symbol must lie within +/- 4 GB of the Program Counter
// #Resources
// - https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_REL register, symbol
	adrp	\register, \symbol
	add		\register, \register, #:lo12:\symbol
.endm

.equ		_core_id_mask, 0b11

.section .text._start

//------------------------------------------------------------------------------
// fn _start()
//------------------------------------------------------------------------------
_start:
	mrs			x1, MPIDR_EL1
	and			x1, x1, _core_id_mask
	ldr 		x2, BOOT_CORE_ID			// provided by bsp/__board_name__/cpu.rs
	cmp 		x1, x2
	b.ne		.L_parking_loop
	
	//If execution reaches here, it is the boot core.
	
	//Initialize DRAM
	ADR_REL x0, __bss_start
	ADR_REL	x1, __bss_end_exclusive

.L_bss_init_loop:
	cmp			x0, x1
	b.eq		.L_prepare_rust
	stp			xzr, xzr, [x0], 16
	b				.L_bss_init_loop

.L_prepare_rust:
	ADR_REL	x0, __boot_core_stack_end_exclusive
	mov			sp, x0
	b				_start_rust //Jump to Rust Code

.L_parking_loop:	//Infinitely wait for events
	wfe
	b				.L_parking_loop

.size			_start, .-_start
.type			_start, function
.global		_start