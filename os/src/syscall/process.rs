//!ASCII Rust SPA4 LF
// Docutitle: App management syscalls
// Codifiers: @dosconio: 20240509
// Attribute: RISC-V-64
// Copyright: rCore-Tutorial-Code-2024S

use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, get_start_time, suspend_current_and_run_next, get_syscall_times, TaskStatus},
    timer::{/*get_time,*/ get_time_ms, get_time_us},
    syscall::SYSCALL_TASK_INFO,
};

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
    trace!("[kernel] Application exited with code {}", exit_code);
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
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// @dosconio 20240516
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    unsafe {
        (*_ti).time = get_time_ms() - (get_start_time() as usize);// 返回系统调用时刻距离任务第一次被调度时刻的时长，也就是说这个时长可能包含该任务被其他任务抢占后的等待重新调度的时间, 単位:ms
        get_syscall_times(&mut (*_ti).syscall_times);
        (*_ti).status = TaskStatus::Running;// 由于查询的是当前任务的状态，因此 TaskStatus 一定是 Running
        info!("SYS_TASK_INFO: getinfo_times={} span_time={}ms", (*_ti).syscall_times[SYSCALL_TASK_INFO], (*_ti).time);
    }
    0
}

