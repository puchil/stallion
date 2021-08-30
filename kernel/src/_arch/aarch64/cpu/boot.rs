global_asm!(include_str!("boot.s"));

//
// The Rust entry of the 'kernel' binary.
// Called from: _start
#[no_mangle]
pub unsafe fn _start_rust() -> ! {
	crate::kernel_init()
}