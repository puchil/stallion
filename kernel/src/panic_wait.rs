use crate::{cpu, println};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	if let Some(args) = info.message() {
		println!("\nKernel Panic: {}", args);
	} else {
		println!("\nKernel Panic!");
	}
	cpu::wait_forever()
}

