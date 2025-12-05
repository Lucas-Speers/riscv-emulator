use crate::{dram::{DRAM_SIZE, Dram}, exception::Exception};

pub const DRAM_START: u64 = 0x8000000;
pub const DRAM_END: u64 = DRAM_START + DRAM_SIZE;

pub const DTB_START: u64 = 0x8000000;
pub const DTB_END: u64 = DTB_START + 0xf000;

pub struct Bus {
    dram: Dram
}

impl Bus {
    pub fn new() -> Self {
        Self {
            dram: Dram::new()
        }
    }

    pub fn load_dram(&mut self, data: &[u8]) {
        self.dram.load(data);
    }

    pub fn read(&self, addr: u64, size: u8) -> Result<u64, Exception> {
        match addr {
            DRAM_START..DRAM_END => self.dram.read(addr-DRAM_START, size),
            _ => Err(Exception::MemoryOOB)
        }
    }

    pub fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Exception> {
        match addr {
            DRAM_START..DRAM_END => self.dram.write(addr-DRAM_START, value, size),
            _ => Err(Exception::MemoryOOB)
        }
    }
}