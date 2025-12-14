use crate::{clint::Clint, dram::{DRAM_SIZE, Dram}, exception::Exception, rom::Rom, uart::Uart};

pub const DRAM_START: u64 = 0x80000000;
pub const DRAM_END: u64 = DRAM_START + DRAM_SIZE;

pub const DTB_START: u64 = 0x1000;
pub const DTB_END: u64 = DTB_START + 0xf000;

pub const UART_START: u64 = 0x10000000;
pub const UART_END: u64 = UART_START + 0x100;

pub const CLINT_START: u64 = 0x2000000;
pub const CLINT_END: u64 = CLINT_START + 0x10000;

pub struct Bus {
    pub dtb: Rom,
    pub dram: Dram,
    uart: Uart,
    clint: Clint,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            dtb: Rom::new(),
            dram: Dram::new(),
            uart: Uart::new(),
            clint: Clint::new(),
        }
    }

    pub fn read(&self, addr: u64, size: u8) -> Result<u64, Exception> {
        // dbg!(addr);
        // dbg!(DTB_START);
        match addr {
            DTB_START..DTB_END => self.dtb.read(addr-DTB_START, size),
            UART_START..UART_END => self.uart.read(addr-UART_START, size),
            CLINT_START..CLINT_END => self.clint.read(addr-CLINT_START, size),
            DRAM_START..DRAM_END => self.dram.read(addr-DRAM_START, size),
            _ => {
                println!("addr: {addr:x}");
                Err(Exception::LoadAccessFault)
            }
        }
    }

    pub fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Exception> {
        match addr {
            DTB_START..DTB_END => self.dram.write(addr-DTB_START, value, size),
            UART_START..UART_END => self.uart.write(addr-UART_START, value, size),
            CLINT_START..CLINT_END => self.clint.write(addr-CLINT_START, value, size),
            DRAM_START..DRAM_END => self.dram.write(addr-DRAM_START, value, size),
            _ => {
                println!("addr: {addr:x}");
                Err(Exception::StoreAccessFault)
            }
        }
    }
}