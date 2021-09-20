//! - Name: kernel
//! - Type: Nano Kernel
//! - Copied from: https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials
//! 
//! ## Code Organization
//! The code is divided into different *modules*, each representing a typical
//! **subsystem** of the 'kernel'. Top level module files of the subsystems
//! reside directly in the 'src' folder.
//! ### Subsystems
//! - CPU
//! - Memory
//! ### Architecture Dependent Code
//! Some of the kernel features need low level architecture specific code. For
//! each of these architectures, we will maintain a folder under src/_arch. These
//! folders mirror the modules in the main folder. Usually, the chosen module name
//! is the generic module's name prefixed with `arch_`. This way, each architecture
//! specific module can provide its implementation of an item, while the caller 
//! must not be concerned which architecture has been conditionally compiled.
//! For Example: 
//!    `kernel`'s MMU subsystem (`src/memory/mmu.rs`)
//!    AARCH64 specific code    (`src/_arch/aarch64/memory/mmu.rs`)
//! top of `src/memory/mmu.rs`:
//! ```
//! #[cfg(target_arch = "aarch64")]
//! #[path = "../_arch/aarch64/memory/mmu.rs"]
//! mod arch_mmu;
//! ```
//! ### Board Support Package (`bsp`)
//! `BSP` code organized under `src/bsp.rs` and contains target board specific 
//! definitions and functions. These are things such as the board's memory map
//! or instances of drivers for devices that are featured on the respective board.
//! Just like processor architecture code, the `BSP` code's module structure tries
//! to mirror the `kernel`'s subsystem modules, but there is no reexporting this
//! time. That means whatever is provided must be called starting from the `bsp`
//! namespace, e.g. `bsp::driver::driver_manager()`.
//!
//! 1. The kernel's entry point is the function `cpu::boot::arch_boot::_start()`.
//!     - It is implemented in `src/_arch/__arch_name__/cpu/boot.s`.

#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod panic_wait;
mod print;
mod synchronization;

unsafe fn kernel_init() -> ! {
	use console::interface::Statistics;

	println!("[0] Hello from Stallion!");
	println!("[1] Chars written:...{}", bsp::console::console().chars_written() );

	println!("[2] See you soon.");
	cpu::wait_forever()
}