#[derive(Debug)]
pub enum Exception {
    EmulatorBug,
    Misc,
    MemoryOOB,
    #[allow(unused)]
    InvalidInstruction(String),
}