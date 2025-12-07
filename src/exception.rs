#![allow(unused)]

#[derive(Debug)]
pub enum Exception {
    IllegalInstruction(String),
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    HardwareError,
}

impl Exception {
    pub fn to_code(&self) -> u64 {
        match self {
            Exception::IllegalInstruction(_) => 2,

            Exception::LoadAddressMisaligned => 4,
            Exception::LoadAccessFault => 5,
            Exception::StoreAddressMisaligned => 6,
            Exception::StoreAccessFault => 7,

            Exception::HardwareError => 13,
        }
    }
}