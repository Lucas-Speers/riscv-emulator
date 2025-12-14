use crate::exception::Exception;



pub struct Uart {

}

impl Uart {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn read(&self, _addr: u64, _size: u8) -> Result<u64, Exception> {
        Ok(0)
    }

    pub fn write(&self, addr: u64, value: u64, _size: u8) -> Result<(), Exception> {
        if addr == 0 {
            eprint!("{}", value as u8 as char)
        }
        Ok(())
    }
}