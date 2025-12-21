use crate::{scheduler::{CURRENT, PROCS, SCHEDULER}, switch_context, PCB};

#[derive(Debug)]
pub enum SchedulerError {
    NoSpace, 
    Empty, 
    NoCurrent, 
    ProcessNotFound,
}

pub trait Scheduler {
    fn enqueue(&mut self, pid: u8) -> Result<(), SchedulerError>; 
    fn dequeue(&mut self) -> Result<u8, SchedulerError>;  
}

pub fn current() -> Option<u8> {
    unsafe { CURRENT }
}

