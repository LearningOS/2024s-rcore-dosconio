//!ASCII Rust SPA4 LF
// Docutitle: Sys-Call of Mcca-rCore
// Codifiers: @dosconio: 20240509
// Attribute: RISC-V-64
// Copyright: rCore-Tutorial-Code-2024S


//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

//pub const SYSCALL_DUP: usize = 24;
//pub const SYSCALL_UNLINKAT: usize = 35;
//pub const SYSCALL_OPENAT: usize = 56;
//pub const SYSCALL_CLOSE: usize = 57;
//pub const SYSCALL_PIPE: usize = 59;
/// fget
pub const SYSCALL_READ: usize = 63;
/// fprintf
pub const SYSCALL_WRITE: usize = 64;
//pub const SYSCALL_LINKAT: usize = 37;
//pub const SYSCALL_FSTAT: usize = 80;
/// quit and execute next subapp
pub const SYSCALL_EXIT: usize = 93;
//pub const SYSCALL_SLEEP: usize = 101;
/// yield syscall
pub const SYSCALL_YIELD: usize = 124;
//pub const SYSCALL_KILL: usize = 129;
//pub const SYSCALL_SIGACTION: usize = 134;
//pub const SYSCALL_SIGPROCMASK: usize = 135;
//pub const SYSCALL_SIGRETURN: usize = 139;
/// setpriority syscall
pub const SYSCALL_SET_PRIORITY: usize = 140;
/// gettime syscall
pub const SYSCALL_GET_TIME: usize = 169;
/// getpid syscall
pub const SYSCALL_GETPID: usize = 172;
//pub const SYSCALL_GETTID: usize = 178;
/// sbrk syscall
pub const SYSCALL_SBRK: usize = 214;
/// munmap syscall
pub const SYSCALL_MUNMAP: usize = 215;
/// fork syscall
pub const SYSCALL_FORK: usize = 220;
/// exec syscall
pub const SYSCALL_EXEC: usize = 221;
/// mmap syscall
pub const SYSCALL_MMAP: usize = 222;
/// waitpid syscall
pub const SYSCALL_WAITPID: usize = 260;
/// spawn syscall
pub const SYSCALL_SPAWN: usize = 400;
//pub const SYSCALL_MAIL_READ: usize = 401;
//pub const SYSCALL_MAIL_WRITE: usize = 402;
/// taskinfo syscall
pub const SYSCALL_TASK_INFO: usize = 410;
//pub const SYSCALL_THREAD_CREATE: usize = 460;
//pub const SYSCALL_WAITTID: usize = 462;
//pub const SYSCALL_MUTEX_CREATE: usize = 463;
//pub const SYSCALL_MUTEX_LOCK: usize = 464;
//pub const SYSCALL_MUTEX_UNLOCK: usize = 466;
//pub const SYSCALL_SEMAPHORE_CREATE: usize = 467;
//pub const SYSCALL_SEMAPHORE_UP: usize = 468;
//pub const SYSCALL_ENABLE_DEADLOCK_DETECT: usize = 469;
//pub const SYSCALL_SEMAPHORE_DOWN: usize = 470;
//pub const SYSCALL_CONDVAR_CREATE: usize = 471;
//pub const SYSCALL_CONDVAR_SIGNAL: usize = 472;
//pub const SYSCALL_CONDVAR_WAIT: usize = 473;

mod fs;
mod process;

use fs::*;
use process::*;
use crate::task::increase_syscall_times;

/// interface, handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    increase_syscall_times(syscall_id);
    match syscall_id {
        SYSCALL_READ => sys_read(args[0], args[1] as *const u8, args[2]),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GETPID => sys_getpid(),
        SYSCALL_FORK => sys_fork(),
        SYSCALL_EXEC => sys_exec(args[0] as *const u8),
        SYSCALL_WAITPID => sys_waitpid(args[0] as isize, args[1] as *mut i32),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        SYSCALL_TASK_INFO => sys_task_info(args[0] as *mut TaskInfo),
        SYSCALL_MMAP => sys_mmap(args[0], args[1], args[2]),
        SYSCALL_MUNMAP => sys_munmap(args[0], args[1]),
        SYSCALL_SBRK => sys_sbrk(args[0] as i32),
        SYSCALL_SPAWN => sys_spawn(args[0] as *const u8),
        SYSCALL_SET_PRIORITY => sys_set_priority(args[0] as isize),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}


