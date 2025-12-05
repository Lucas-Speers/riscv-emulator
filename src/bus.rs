use crate::{dram::{DRAM_SIZE, Dram}, exception::Exception, rom::Rom};

pub const DRAM_START: u64 = 0x8000000;
pub const DRAM_END: u64 = DRAM_START + DRAM_SIZE;

pub const DTB_START: u64 = 0x1000;
pub const DTB_END: u64 = DTB_START + 0xf000;

pub struct Bus {
    pub dtb: Rom,
    pub dram: Dram,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            dtb: Rom::new(),
            dram: Dram::new(),
        }
    }

    pub fn read(&self, addr: u64, size: u8) -> Result<u64, Exception> {
        // dbg!(addr);
        // dbg!(DTB_START);
        match addr {
            DTB_START..DTB_END => self.dtb.read(addr-DTB_START, size),
            DRAM_START..DRAM_END => self.dram.read(addr-DRAM_START, size),
            _ => Err(Exception::MemoryOOB)
        }
    }

    pub fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Exception> {
        match addr {
            DTB_START..DTB_END => self.dram.write(addr-DTB_START, value, size),
            DRAM_START..DRAM_END => self.dram.write(addr-DRAM_START, value, size),
            _ => Err(Exception::MemoryOOB)
        }
    }
}