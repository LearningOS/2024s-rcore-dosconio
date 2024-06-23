//!ASCII Rust SPA4 LF
// Docutitle: Panic-Mechanism of Mcca-rCore
// Codifiers: @dosconio: 20240423
// Attribute: RISC-V-64

use core::panic::PanicInfo;
use crate::sbi::shutdown;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[rkernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("[rkernel] Panicked: {}", info.message().unwrap());
    }
    shutdown()
}
