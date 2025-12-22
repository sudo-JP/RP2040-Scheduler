use crate::{process::*, Scheduler, PROCS, SCHEDULER};
use crate::layout::MemoryLayout;
use core::ptr;

use core::result::Result;
use core::result::Result::{Ok, Err};


static mut NEXT_FREE: usize = 0; 
static mut ID: u8 = 0; 

#[derive(Debug)]
pub enum ProcessError {
    NoMemory, 
    InvalidSize, 
} 

/*
 * Return the start of the stack, given size
 * */
fn allocate_stack(size: usize) -> Result<*mut u8, ProcessError> {
    if size == 0 {
        return Err(ProcessError::InvalidSize);
    }
    let region = MemoryLayout::new(); 
    unsafe {
        if NEXT_FREE + size > region.processes.size {
            return Err(ProcessError::NoMemory);
        }
        else {
            let ptr: *mut u8 = (NEXT_FREE + region.processes.start) as *mut u8;
            NEXT_FREE += size; 
            return Ok(ptr); 
        }
    }
}

#[allow(improper_ctypes_definitions)]
#[allow(unreachable_code)]
unsafe extern "C" fn thread_stub(entry: fn(*mut ()) -> !, 
    arg: *mut ()) -> ! {
    entry(arg);
    loop {}
}

unsafe fn setup_initial_stack(stack_base: *mut u8, 
    stack_size: usize, entry: fn(*mut ()) -> !, arg: *mut()) -> *mut u32 {
    let mut sp = (stack_base as usize + stack_size) as *mut u32; 

    // Move down one and write some value
    unsafe {
        // Build exception frame from top down
        sp = sp.offset(-1); 
        *sp = 0x01000000; // xPSR (Thumb bit)
        
        sp = sp.offset(-1);
        *sp = entry as usize as u32; // PC - entry function, not thread_stub!
        
        sp = sp.offset(-1);
        *sp = 0xFFFFFFFD; // LR - exception return value
        
        sp = sp.offset(-1);
        *sp = 0; // R12
        
        sp = sp.offset(-1);
        *sp = 0; // R3
        
        sp = sp.offset(-1);
        *sp = 0; // R2
        
        sp = sp.offset(-1);
        *sp = 0; // R1
        
        sp = sp.offset(-1);
        *sp = arg as usize as u32; // R0 - argument
        
        // Now R4-R11 (for switch_context compatibility)
        for _ in 0..8 {
            sp = sp.offset(-1);
            *sp = 0;
        }
    }

    sp
}


pub unsafe fn create_process(stack_size: usize, 
    entry: fn(* mut()) -> !, parg: *mut ()) -> Result<u8, ProcessError> {
    let stack_start = allocate_stack(stack_size)?;
    unsafe {
        core::ptr::write_volatile(stack_start, 0xAA);
        if core::ptr::read_volatile(stack_start) != 0xAA {
            loop {
                
            }
        }
    }

    unsafe {
        let sp = setup_initial_stack(stack_start, stack_size, entry, parg);
        let id: u8 = ID; 
        let pcb = PCB {
            sp: sp,
            pid: id, 
            state: ProcessState::Ready, 

            first_run: true, 
            stack_base: stack_start, 
            stack_size: stack_size, 
        };
        PROCS[id as usize] = Some(pcb); 
        let sched = ptr::addr_of_mut!(SCHEDULER); 
        (*sched).enqueue(id).unwrap();
        ID += 1; 
        Ok(id)
    }
}
