#!ASCII Rust TAB4 LF
# Docutitle: Trap of Mcca-rCore
# Codifiers: @dosconio: 20240509
# Attribute: RISC-V-64
# Copyright: rCore-Tutorial-Code-2024S

# - Context
# - Switch Stack

.altmacro # for using .rept directive
.macro SAVE_GP n
	sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
	ld x\n, \n*8(sp)
.endm
    
	.section .text.trampoline
	.globl __alltraps
	.globl __restore
	.align 2 # align to 4-byte boundary マ
__alltraps:# entry of trap, will save trap context for trap_handler()
	# here, sp->user stack, sscratch->kernel stack
	csrrw sp, sscratch, sp # aka Assign3, aka xchg(sp, sscratch)
	# below sp->kernel stack, sscratch->user stack
	# allocate a TrapContext on kernel stack
	#addi sp, sp, -34*8
	# save general-purpose registers
		# skip sp(x0), it is always encoded as 0
	sd x1, 1*8(sp)
		# skip sp(x2), we will save it later
	sd x3, 3*8(sp)
		# skip tp(x4), application does not use it at most cases
		# save x5~x31
	.set n, 5
	.rept 27
		SAVE_GP %n
		.set n, n+1
	.endr
	# we can use t0/t1/t2 freely, because they were saved on kernel stack
	csrr t0, sstatus # t0 as transfer correspondence, mov(t0, sstatus)
	csrr t1, sepc    # t1 as transfer correspondence, mov(t1, sepc)
	sd t0, 32*8(sp)
	sd t1, 33*8(sp)
	# read user stack from sscratch and save it on the kernel stack
	csrr t2, sscratch # sp->kernel stack, sscratch->user stack
	sd t2, 2*8(sp)
    # load kernel_satp into t0
    ld t0, 34*8(sp)
    # load trap_handler into t1
    ld t1, 36*8(sp)
    # move to kernel_sp
    ld sp, 35*8(sp)
    # switch to kernel space
    csrw satp, t0
    sfence.vma
    # jump to trap_handler
    jr t1

	# restore trap context
__restore:
    # a0: *TrapContext in user space(Constant); 
	# a1: user space token
    # switch to user space
    csrw satp, a1
    sfence.vma
    csrw sscratch, a0
    mv sp, a0
    # now sp points to TrapContext in user space, start restoring based on it
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    # restore general purpose registers except x0/sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # back to user stack
    ld sp, 2*8(sp)
    sret