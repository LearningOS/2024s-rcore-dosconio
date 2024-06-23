//!ASCII Rust SPA4 LF
// Docutitle: ? of Mcca-rCore
// Codifiers: @dosconio: 20240515
// Attribute: RISC-V-64
// Copyright: rCore-Tutorial-Code-2024S

//! Task management implementation
//!
//! Everything about task management, like starting and switching tasks is
//! implemented here.
//!
//! A single global instance of [`TaskManager`] called `TASK_MANAGER` controls
//! all the tasks in the operating system.
//!
//! Be careful when you see `__switch` ASM function in `switch.S`. Control flow around this function
//! might not be what you expect.

mod context;
mod id;
mod manager;
mod processor;
mod switch;
#[allow(clippy::module_inception)]
mod task;

use crate::loader::get_app_data_by_name;
use alloc::sync::Arc;
use lazy_static::*;
pub use manager::{fetch_task, TaskManager};
use switch::__switch;
pub use task::{TaskControlBlock, TaskStatus};
pub use processor::PROCESSOR;
pub use processor::{current_user_token, current_trap_cx};
pub use context::TaskContext;
pub use id::{kstack_alloc, pid_alloc, KernelStack, PidHandle};
pub use manager::{add_task, TASK_MANAGER};
pub use processor::{
    current_task, run_tasks, schedule, take_current_task,
    Processor,
};//{TEMP} , current_trap_cx, current_user_token
use crate::config::*;
use crate::mm::MapPermission;
use crate::mm::VirtAddr;

/// Suspend the current 'Running' task and run the next task in task list.
pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();

    // ---- access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    task_inner.task_stride = (task_inner.task_stride + BIG_STRIDE / task_inner.task_priority) % BIG_STRIDE;
    // info!("!");

    drop(task_inner);
    // push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    schedule(task_cx_ptr);
}

/// pid of usertests app in make run TEST=1
pub const IDLE_PID: usize = 0;

/// Exit the current 'Running' task and run the next task in task list.
pub fn exit_current_and_run_next(exit_code: i32) {
    // take from Processor
    let task = take_current_task().unwrap();

    let pid = task.getpid();
    if pid == IDLE_PID {
        println!(
            "[kernel] Idle process exit with exit_code {} ...",
            exit_code
        );
        panic!("All applications completed!");
    }

    // **** access current TCB exclusively
    let mut inner = task.inner_exclusive_access();
    // Change status to Zombie
    inner.task_status = TaskStatus::Zombie;
    // Record exit code
    inner.exit_code = exit_code;
    // do not move to its parent but under initproc

    // ++++++ access initproc TCB exclusively
    {
        let mut initproc_inner = INITPROC.inner_exclusive_access();
        for child in inner.children.iter() {
            child.inner_exclusive_access().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }
    // ++++++ release parent PCB

    inner.children.clear();
    // deallocate user space
    inner.memory_set.recycle_data_pages();
    drop(inner);
    // **** release current PCB
    // drop task manually to maintain rc correctly
    drop(task);
    // we do not have to save task context
    let mut _unused = TaskContext::zero_init();
    schedule(&mut _unused as *mut _);
}

lazy_static! {
    /// Creation of initial process
    ///
    /// the name "initproc" may be changed to any other app name like "usertests",
    /// but we have user_shell, so we don't need to change it.
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
        get_app_data_by_name("ch5b_initproc").unwrap()
    ));
}

///Add init process to the manager
pub fn add_initproc() {
    add_task(INITPROC.clone());
}

// ---- BELOW ARE INTERFACES ----
//{?} take_current_task()

/// Get start time (unit/ms)
#[allow(warnings)]
pub fn get_start_time() -> isize {
    let crt = current_task().unwrap();
    let res = crt.inner_exclusive_access();
    // info!(" start time: {}.", res.start_time);
    res.start_time
}

/// Increase syscall times
pub fn increase_syscall_times(callno: usize) {
    if callno >= MAX_SYSCALL_NUM {
        panic!("Invalid syscall number!");
    }
    let crt = current_task().unwrap();
    let mut res = crt.inner_exclusive_access();
    res.syscall_times[callno] += 1;
}

/// Get syscall times
pub fn get_syscall_times(systab: &mut [u32; MAX_SYSCALL_NUM]) {
    let crt = current_task().unwrap();
    let res = crt.inner_exclusive_access();
    for i in 0..MAX_SYSCALL_NUM {
        systab[i] = res.syscall_times[i];
        if systab[i] > 0 {
            info!("Syscall {} times: {}", i, systab[i]);
        }
    }
}

/// ...
pub fn do_task_mmap(start: usize, len: usize, port: usize) -> bool {
    trace!("Paging Map: {:#x}(inc) ~ {:#x}(exc)", start, start + len);
    let mut port = MapPermission::from_bits((port as u8) << 1).unwrap();
    port.set(MapPermission::U, true);
    let crt = current_task().unwrap();
    let mut res = crt.inner_exclusive_access();
    let crt_memset = &mut res.memory_set;
    let state: bool = !crt_memset.if_overlap(VirtAddr::from(start), VirtAddr::from(start + len));
    if state {
        crt_memset.insert_framed_area(
            VirtAddr::from(start), VirtAddr::from(start + len), port);
    }
    state
} 

/// ...
pub fn do_task_munmap(start: usize, len: usize) -> bool {
    trace!("Paging Unmap: {:#x}(inc) ~ {:#x}(exc)", start, start + len);
    let crt = current_task().unwrap();
    let mut res = crt.inner_exclusive_access();
    let crt_memset = &mut res.memory_set;
    let state: bool = crt_memset.if_matched(start, start + len);
    if state {
        crt_memset.remove_area(VirtAddr::from(VirtAddr::from(start).floor()) ,
        VirtAddr::from(VirtAddr::from(start + len).ceil()));
    }
    state
}
