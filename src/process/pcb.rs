pub enum ProcessState {
    Ready, 
    Running, 
    Blocked,
}

#[repr(C)]
pub struct PCB {
    pub pid: u8, 
    pub state: ProcessState, 
    pub killed: bool, 

    pub sp: *mut u32,           // Stack pointer, we on 32-bit arch 
    pub stack_base: *mut u8,    // Where stack allocation starts 
    pub stack_size: usize,      // Stack size, native size 
}
