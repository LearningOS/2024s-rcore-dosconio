//!ASCII Rust SPA4 LF
// Docutitle: Constants of Mcca-rCore
// Codifiers: @dosconio: 20240515
// Attribute: RISC-V-64
// Copyright: rCore-Tutorial-Code-2024S

#[allow(unused)]

/// user app's stack size
pub const USER_STACK_SIZE: usize = 4096;
/// kernel stack size
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
/// kernel heap size
pub const KERNEL_HEAP_SIZE: usize = 0x20000;
/// the max number of apps
pub const MAX_APP_NUM: usize = 16;
/// base_addr(changed) of app
pub const APP_BASE_ADDRESS: usize = 0x80400000;
/// size limit of app
pub const APP_SIZE_LIMIT: usize = 0x20000;

/// the max number of syscall
pub const MAX_SYSCALL_NUM: usize = 500;// 在我们的实验中，系统调用号一定小于 500，所以直接使用一个长为 MAX_SYSCALL_NUM=500 的数组做桶计数。
/// clock frequency
pub const CLOCK_FREQ: usize = 12500000;
/// the physical memory end
pub const MEMORY_END: usize = 0x88000000;


