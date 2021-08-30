__rpi_load_addr = 0x80000;

ENTRY(__rpi_load_addr)

PHDRS
{
	segment_rx PT_LOAD FLAGS(5); /*5 == RX*/
	segment_rx PT_LOAD FLAGS(6); /*6 == RX*/
}

SECTIONS
{
	. = __rpi_load_addr;
																			 /*   ^                 */
																			 /*   ^ Stack						*/
																			 /*   ^ Growth					*/
																			 /*   ^ Direction				*/
	__boot_core_stack_end_exclusive = .; /*   |                 */
	.text :
	{
		KEEP(*(.text._start))
		*(.text._start_arguments) /* Constants (or statics in RUST) read by _start().*/
		*(.text._start_rust)			 /* The Rust entry point */
		*(.text*)								 /* Everything Elese */
	} :segment_rx
	
	.rodata	: ALIGN(8) { *(.rodata*) } : segment_rx
	.got		: ALIGN(8) { *(.got)  	 } : segment_rx
	.data		: 				 { *(.data*)   } : segment_rx
	.bss		: ALIGN(16)
	{
		__bss_start = .;
			*(.bss*);
			. = ALIGN(16);
			__bss_end_exclusive = .;
		} :NONE
}