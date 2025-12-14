#![allow(unused)]

#[derive(Debug)]
pub enum Exception {
    IllegalInstruction(String),
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    ECallFromU,
    ECallFromS,
    ECallFromM,
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
            Exception::ECallFromU => 8,
            Exception::ECallFromS => 9,
            Exception::ECallFromM => 11,

            Exception::HardwareError => 19,
        }
    }
}