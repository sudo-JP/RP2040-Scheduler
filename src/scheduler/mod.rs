pub mod scheduler;
pub mod round_robin;

pub use scheduler::*;
pub use round_robin::*;

use crate::PCB;


const MAX_PROCS: usize = 256;

static mut SCHEDULER: RR = RR::new();
static mut PROCS: [Option<PCB>; MAX_PROCS] = [None; MAX_PROCS];
static mut CURRENT: Option<u8> = None; 
