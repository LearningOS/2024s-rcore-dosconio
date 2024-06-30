//!ASCII Rust SPA4 LF
// Docutitle: Constants of Mcca-rCore
// Codifiers: @dosconio: 20240515
// Attribute: RISC-V-64
// Copyright: rCore-Tutorial-Code-2024S

#[allow(unused)]

/// user app's stack size
pub const USER_STACK_SIZE: usize = 4096 * 2;
/// kernel stack size
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
/// kernel heap size
pub const KERNEL_HEAP_SIZE: usize = 0x200_0000;

/// page size : 4KB
pub const PAGE_SIZE: usize = 0x1000;
/// page size bits: 12
pub const PAGE_SIZE_BITS: usize = 0xc;
/// the max number of syscall

/// the virtual addr of trapoline
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
/// the virtual addr of trap context
pub const TRAP_CONTEXT_BASE: usize = TRAMPOLINE - PAGE_SIZE;

/// the max number of syscall
pub const MAX_SYSCALL_NUM: usize = 500;// 在我们的实验中，系统调用号一定小于 500，所以直接使用一个长为 MAX_SYSCALL_NUM=500 的数组做桶计数。

/// clock frequency
pub const CLOCK_FREQ: usize = 12500000;
/// the physical memory end
pub const MEMORY_END: usize = 0x88000000;

/// .
pub const BIG_STRIDE: usize = 48;// Least Common Multiple of 12 and 16

/// The base address of control registers in Virtio_Block device
pub const MMIO: &[(usize, usize)] = &[(0x10001000, 0x1000)];
