//!ASCII Rust SPA4 LF
// Docutitle: Console of Mcca-rCore
// Codifiers: @dosconio: 20240424
// Attribute: RISC-V-64

use core::fmt::{self, Write};
use crate::sbi::console_putchar;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // syswrite(1, s.as_bytes());
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

/// 
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// usual macro
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::consio::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// usual macro
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::consio::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
