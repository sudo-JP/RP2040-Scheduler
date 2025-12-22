use core::ptr;
use crate::{jump_to_process, scheduler::{CURRENT, PROCS, SCHEDULER}, switch_context, PCB};

#[derive(Debug)]
pub enum SchedulerError {
    NoSpace, 
    Empty, 
    NoCurrent, 
    ProcessNotFound,
    NotRunnable, 
}

pub trait Scheduler {
    fn enqueue(&mut self, pid: u8) -> Result<(), SchedulerError>; 
    fn dequeue(&mut self) -> Result<u8, SchedulerError>;  
}

pub fn current() -> Option<u8> {
    unsafe { CURRENT }
}



pub fn yield_now() -> Result<(), SchedulerError> {
    unsafe {
        let sched = ptr::addr_of_mut!(SCHEDULER); 
        let old_pid = CURRENT.ok_or(SchedulerError::NoCurrent)?;
        let next_pid = (*sched).dequeue()?;

        if old_pid == next_pid {
            (*sched).enqueue(old_pid)?;
            return Ok(());
        }

        (*sched).enqueue(old_pid)?;
        CURRENT = Some(next_pid);
        
        let old_pcb: *mut PCB = PROCS[old_pid as usize].as_mut().ok_or(SchedulerError::ProcessNotFound)?;
        let new_pcb: *mut PCB = PROCS[next_pid as usize].as_mut().ok_or(SchedulerError::ProcessNotFound)?;

        (*old_pcb).state = crate::ProcessState::Ready;
        (*new_pcb).state = crate::ProcessState::Running;
        
        // Always use switch_context
        if (*new_pcb).first_run {
            (*new_pcb).first_run = false;
            CURRENT = Some(next_pid);
            
            // Save old context
            core::arch::asm!(
                "push {{lr}}",
                "push {{r4-r7}}",
                "mov r4, r8", "mov r5, r9", "mov r6, r10", "mov r7, r11",
                "push {{r4-r7}}",
                "mov r2, sp",
                "str r2, [{0}]",
                in(reg) (old_pcb as usize) as *mut *mut u32,
            );
            
            // Jump to new (exception frame)
            jump_to_process((*new_pcb).sp);
        } else {
            CURRENT = Some(next_pid);
            
            // Both have been saved before - use inline asm to switch
            core::arch::asm!(
                // Save old
                "push {{lr}}",
                "push {{r4-r7}}",
                "mov r4, r8", "mov r5, r9", "mov r6, r10", "mov r7, r11",
                "push {{r4-r7}}",
                "mov r2, sp",
                "str r2, [{old}]",
                
                // Load new
                "ldr r2, [{new}]",
                "mov sp, r2",
                
                // Restore new
                "pop {{r4-r7}}",
                "mov r8, r4", "mov r9, r5", "mov r10, r6", "mov r11, r7",
                "pop {{r4-r7}}",
                
                old = in(reg) (old_pcb as usize) as *mut *mut u32,
                new = in(reg) (new_pcb as usize) as *mut *mut u32,
            );
        }
   
    }
    Ok(())
}
