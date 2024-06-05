//!ASCII Rust SPA4 LF
// Docutitle: File and filesystem-related syscalls
// Codifiers: @dosconio: 20240509
// Attribute: RISC-V-64
// Copyright: rCore-Tutorial-Code-2024S

use crate::memory::translated_byte_buffer;
use crate::task::current_user_token;

const FD_STDOUT: usize = 1;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel: sys_write");
    match fd {
        FD_STDOUT => {
            let buffers = translated_byte_buffer(current_user_token(), buf, len);
            for buffer in buffers {
                print!("{}", core::str::from_utf8(buffer).unwrap());
            }
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}

