//!ASCII Rust SPA4 LF
// Docutitle: ? of Mcca-rCore
// Codifiers: @dosconio: 20240515
// Attribute: RISC-V-64
// Copyright: rCore-Tutorial-Code-2024S

//! Rust wrapper around `__switch`.
//!
//! Switching to a different task's context happens here. The actual
//! implementation must not be in Rust and (essentially) has to be in assembly
//! language (Do you know why?), so this module really is just a wrapper around
//! `switch.S`.

//!Wrap `switch.S` as a function
use super::TaskContext;
use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

extern "C" {
    /// Switch to the context of `next_task_cx_ptr`, saving the current context
    /// in `current_task_cx_ptr`.
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}
