use crate::exception::Exception;



pub struct Clint {

}

impl Clint {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn read(&self, addr: u64, size: u8) -> Result<u64, Exception> {
        println!("CLINT READ: addr={addr:X} size={size}");
        Ok(0)
    }

    pub fn write(&self, addr: u64, value: u64, size: u8) -> Result<(), Exception> {
        println!("CLINT WRITE: addr={addr:X} value={value:X} size={size}");
        Ok(())
    }
}