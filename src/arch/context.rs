use crate::process::*;

/*
Save old process context
1. Push R4-R11 onto current stack
2. Save current SP into old_pcb.sp
3. Load new_pcb.sp into SP register
4. Pop R4-R11 from new stack
5. Return (hardware will restore R0-R3, R12, LR, PC, PSR)

r0 correspond to old_pcb 
r1 correspond to new_pcb
*/
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn switch_context(old_pcb: *mut PCB, new_pcb: *const PCB) {
    core::arch::naked_asm!(
        // Save LR first
        "push {{lr}}",
        
        // Save R4-R7
        "push {{r4-r7}}",
        
        // Save R8-R11
        "mov r4, r8",
        "mov r5, r9", 
        "mov r6, r10", 
        "mov r7, r11", 
        "push {{r4-r7}}",

        // Save SP to old_pcb
        "mov r2, sp",
        "str r2, [r0, #0]",

        // Load SP from new_pcb
        "ldr r2, [r1, #0]",
        "mov sp, r2",
        
        // Restore R8-R11
        "pop {{r4-r7}}",
        "mov r8, r4", 
        "mov r9, r5",
        "mov r10, r6",
        "mov r11, r7",
        
        // Restore R4-R7
        "pop {{r4-r7}}",
        
        // Restore LR
        "pop {{r0}}",
        "mov lr, r0",
        
        // Return
        "bx lr",
    );
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jump_to_process(sp: *const u32) {
    core::arch::naked_asm!(
        "mov sp, r0",           // Set stack pointer 
        
        // Pop R4-R11 (our saved context)
        "pop {{r4-r7}}",
        "mov r8, r4",
        "mov r9, r5",
        "mov r10, r6",
        "mov r11, r7",
        "pop {{r4-r7}}",
        
        // Pop exception frame: R0-R3, R12, LR, PC, xPSR
        "pop {{r0-r3}}",
        "pop {{r4}}",           // R12 -> r4
        "mov r12, r4",
        "pop {{r4}}",           // LR -> r4
        "mov lr, r4",
        "pop {{r4}}",           // PC -> r4
        "pop {{r5}}",           // xPSR (ignore)
        
        "bx r4",                // Jump to PC
    );
}
