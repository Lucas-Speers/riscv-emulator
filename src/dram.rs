use crate::exception::Exception;

pub const DRAM_SIZE: u64 = 1024 * 1024 * 1024;

pub struct Dram {
    dram: Vec<u8>
}

impl Dram {
    pub fn new() -> Self {
        Self { dram: vec![0;DRAM_SIZE as usize] }
    }

    pub fn load(&mut self, data: &[u8]) {
        self.dram[..data.len()].copy_from_slice(data);
    }

    pub fn read(&self, addr: u64, size: u8) -> Result<u64, Exception> {
        match size {
            8 => Ok(self.read8(addr)),
            16 => Ok(self.read16(addr)),
            32 => Ok(self.read32(addr)),
            64 => Ok(self.read64(addr)),
            _ => Err(Exception::HardwareError)
        }
    }

    pub fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Exception> {
        match size {
            8 => self.write8(addr, value),
            16 => self.write16(addr, value),
            32 => self.write32(addr, value),
            64 => self.write64(addr, value),
            _ => return Err(Exception::HardwareError)
        }

        Ok(())
    }

    fn read8(&self, addr: u64) -> u64 {
        self.dram[addr as usize] as u64
    }

    fn read16(&self, addr: u64) -> u64 {
        self.dram[addr as usize] as u64 |
        (self.dram[addr as usize + 1] as u64) << 8

    }
    
    fn read32(&self, addr: u64) -> u64 {
        self.dram[addr as usize] as u64 |
        (self.dram[addr as usize + 1] as u64) << 8 |
        (self.dram[addr as usize + 2] as u64) << 16 |
        (self.dram[addr as usize + 3] as u64) << 24
    }

    fn read64(&self, addr: u64) -> u64 {
        self.dram[addr as usize] as u64 |
        (self.dram[addr as usize + 1] as u64) << 8 |
        (self.dram[addr as usize + 2] as u64) << 16 |
        (self.dram[addr as usize + 3] as u64) << 24 |
        (self.dram[addr as usize + 4] as u64) << 32 |
        (self.dram[addr as usize + 5] as u64) << 40 |
        (self.dram[addr as usize + 6] as u64) << 48 |
        (self.dram[addr as usize + 7] as u64) << 56
    }

    fn write8(&mut self, addr: u64, value: u64) {
        self.dram[addr as usize] = value as u8;
    }

    fn write16(&mut self, addr: u64, value: u64) {
        self.dram[addr as usize] = value as u8;
        self.dram[addr as usize + 1] = (value >> 8) as u8;
    }
    
    fn write32(&mut self, addr: u64, value: u64) {
        self.dram[addr as usize] = value as u8;
        self.dram[addr as usize + 1] = (value >> 8) as u8;
        self.dram[addr as usize + 2] = (value >> 16) as u8;
        self.dram[addr as usize + 3] = (value >> 24) as u8;
        
    }

    fn write64(&mut self, addr: u64, value: u64) {
        self.dram[addr as usize] = value as u8;
        self.dram[addr as usize + 1] = (value >> 8) as u8;
        self.dram[addr as usize + 2] = (value >> 16) as u8;
        self.dram[addr as usize + 3] = (value >> 24) as u8;
        self.dram[addr as usize + 4] = (value >> 32) as u8;
        self.dram[addr as usize + 5] = (value >> 40) as u8;
        self.dram[addr as usize + 6] = (value >> 48) as u8;
        self.dram[addr as usize + 7] = (value >> 56) as u8;
    }
}