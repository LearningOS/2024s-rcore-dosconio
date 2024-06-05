//!ASCII Rust SPA4 LF
// Docutitle: App management syscalls
// Codifiers: @dosconio: 20240509
// Attribute: RISC-V-64
// Copyright: rCore-Tutorial-Code-2024S

use crate::{
    config::MAX_SYSCALL_NUM, memory::translated_byte_buffer, task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_start_time, get_syscall_times, suspend_current_and_run_next, TaskStatus // , TASK_MANAGER
    }, timer::{/*get_time,*/ get_time_ms, get_time_us}
};
use core::mem::size_of;
use crate::task::do_task_mmap;
use crate::task::do_task_munmap;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[rkernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let time_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let buffers = translated_byte_buffer(current_user_token(), ts as *const u8, size_of::<TimeVal>());
    let mut time_val_ptr = &time_val as *const _ as *const u8;
    for buffer in buffers {
        unsafe {
            time_val_ptr.copy_to(buffer.as_mut_ptr(), buffer.len());
            time_val_ptr = time_val_ptr.add(buffer.len());
        }
    }
    0
}

/// @dosconio 20240516~20240518
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let mut tmp = TaskInfo {
        time : get_time_ms() - (get_start_time() as usize),
        syscall_times: [0; 500],
        status : TaskStatus::Running,
    };
    get_syscall_times(&mut tmp.syscall_times);
    let buffers = translated_byte_buffer(current_user_token(), _ti as *const u8, size_of::<TaskInfo>());
    let mut task_info_ptr = &tmp as *const _ as *const u8;
    for buffer in buffers {
        unsafe {
            task_info_ptr.copy_to(buffer.as_mut_ptr(), buffer.len());
            task_info_ptr = task_info_ptr.add(buffer.len());
        }
    }
    0
}

/// @dosconio 20240604
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap");

    if _port & 0x7 == 0 {
        return -1;// Useless Mapping Page
    }
    if _start & 0xfff != 0 || _port & !(0b111 as usize) != 0{
        return -1;// Tutorial Request
    }
    if _len == 0 {
        return 0;//{ISSUE}
    }

    if do_task_mmap(_start, _len, _port) { 0 } else { -1 }
}

/// @dosconio 20240604
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap");
    if _len == 0 {
        return 0;//{ISSUE}
    }
    //info!(">>>>>>>>>{:#x}!!{:#x}", _start, _start + _len);// any info/error! here will stuck system
    if do_task_munmap(_start, _len) { 0 } else { -1 }
}
///{TODO} change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}

