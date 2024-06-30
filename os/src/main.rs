//!ASCII Rust TAB4 LF
// Docutitle: Mcca-rCore
// Codifiers: @dosconio: 20240423 ~ <Last-check> 
// Attribute: RISC-V-64

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;

extern crate alloc;

use core::arch::global_asm;
use log::*;

#[macro_use]
mod consio;
pub mod config;
pub mod panics;
pub mod drivers;
pub mod fs;
pub mod logging;
pub mod mm;
pub mod sbi;
pub mod sync;
pub mod syscall;
pub mod task;
pub mod timer;
pub mod trap;

global_asm!(include_str!("boot/entry.asm"));

/// clear BSS segment
pub fn clear_bss() {
	extern "C" {
		fn sbss();// aka start-of-bss
		fn ebss();// aka end-of-bss
	}
	unsafe {
		core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
			.fill(0);
	} // (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

fn kernel_log_info() {
	extern "C" {
		fn stext(); // begin addr of text segment
		fn etext(); // end addr of text segment
		fn srodata(); // start addr of Read-Only data segment
		fn erodata(); // end addr of Read-Only data ssegment
		fn sdata(); // start addr of data segment
		fn edata(); // end addr of data segment
		fn sbss(); // start addr of BSS segment
		fn ebss(); // end addr of BSS segment
		fn boot_stack_lower_bound(); // stack lower bound
		fn boot_stack_top(); // stack top
	}
	logging::init();
	println!("[rkernel] Ciallo, Rust rCore~");
	trace!(
		"[rkernel] .text [{:#x}, {:#x})",
		stext as usize,
		etext as usize
	);
	debug!(
		"[rkernel] .rodata [{:#x}, {:#x})",
		srodata as usize, erodata as usize
	);
	info!(
		"[rkernel] .data [{:#x}, {:#x})",
		sdata as usize, edata as usize
	);
	warn!(
		"[rkernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
		boot_stack_top as usize, boot_stack_lower_bound as usize
	);
	error!("[rkernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
}

/// entry of OS
#[no_mangle]
pub fn rust_main() -> ! {
	clear_bss();
	kernel_log_info();
	mm::init();
    mm::remap_test();
    //{} task::add_initproc();
    //{} println!("after initproc!");
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
	fs::list_apps();
    task::add_initproc();
    task::run_tasks();
    panic!("Unreachable in rust_main!");
}
