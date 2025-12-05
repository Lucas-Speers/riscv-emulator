use crate::exception::Exception;

pub struct Rom {
    rom: Vec<u8>
}

impl Rom {
    pub fn new() -> Self {
        Self { rom: Vec::new() }
    }

    pub fn load(&mut self, data: &[u8]) {
        self.rom = data.to_vec()
    }

    pub fn read(&self, addr: u64, size: u8) -> Result<u64, Exception> {
        match size {
            8 => Ok(self.read8(addr)),
            16 => Ok(self.read16(addr)),
            32 => Ok(self.read32(addr)),
            64 => Ok(self.read64(addr)),
            _ => Err(Exception::EmulatorBug)
        }
    }

    fn read8(&self, addr: u64) -> u64 {
        self.rom[addr as usize] as u64
    }

    fn read16(&self, addr: u64) -> u64 {
        self.rom[addr as usize] as u64 |
        (self.rom[addr as usize + 1] as u64) << 8

    }
    
    fn read32(&self, addr: u64) -> u64 {
        self.rom[addr as usize] as u64 |
        (self.rom[addr as usize + 1] as u64) << 8 |
        (self.rom[addr as usize + 2] as u64) << 16 |
        (self.rom[addr as usize + 3] as u64) << 24
    }

    fn read64(&self, addr: u64) -> u64 {
        self.rom[addr as usize] as u64 |
        (self.rom[addr as usize + 1] as u64) << 8 |
        (self.rom[addr as usize + 2] as u64) << 16 |
        (self.rom[addr as usize + 3] as u64) << 24 |
        (self.rom[addr as usize + 4] as u64) << 32 |
        (self.rom[addr as usize + 5] as u64) << 40 |
        (self.rom[addr as usize + 6] as u64) << 48 |
        (self.rom[addr as usize + 7] as u64) << 56
    }
}