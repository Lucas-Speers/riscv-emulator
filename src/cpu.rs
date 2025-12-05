use crate::{bus::{Bus, DRAM_END, DTB_START}, exception::Exception};

struct Xregs {
    xregs: [u64;32]
}

impl Xregs {
    fn new() -> Self {
        Self { xregs: [0;32] }
    }

    fn read(&self, index: u64) -> u64 {
        self.xregs[index as usize]
    }

    fn write(&mut self, index: u64, value: u64) {
        self.xregs[index as usize] = value;
    }
}

struct Csrs {
    csrs: [u64;4096]
}

impl Csrs {
    fn new() -> Self {
        Self { csrs: [0;4096] }
    }

    fn read(&self, index: u64) -> u64 {
        self.csrs[index as usize] as u64
    }

    fn write(&mut self, index: u64, value: u64) {
        self.csrs[index as usize] = value;
    }
}

enum PrivilegeLevel {
    User = 0,
    Supervisor = 1,
    Machine = 3,
    Debug
}

pub struct Cpu {
    pub bus: Bus,
    level: PrivilegeLevel,
    xregs: Xregs,
    pc: u64,
    csrs: Csrs,
}

impl Cpu {
    pub fn new() -> Self {
        let mut xregs = Xregs::new();
        xregs.write(2, DRAM_END);
        xregs.write(11, DTB_START);

        Self {
            bus: Bus::new(),
            level: PrivilegeLevel::Machine,
            xregs,
            pc: 0,
            csrs: Csrs::new(),
        }
    }

    pub fn print_regs(&self) {
        println!("REGS:");
        println!(" {}", self.xregs.read(1));
        println!(" {}", self.xregs.read(2));
        println!(" {}", self.xregs.read(3));
    }

    pub fn set_pc(&mut self, pc: u64) {
        self.pc = pc;
    }

    fn fetch(&mut self, size: u8) -> Result<u64, Exception> {
        self.bus.read(self.pc, size)
    }

    pub fn execute(&mut self) -> Result<(), Exception> {
        let inst = self.fetch(16)?;
        match inst & 0b11 {
            0b10 | 0b01 => {
                self.execute_compressed(inst)?;
                self.pc += 2;
            },
            0b11 => {
                let inst = self.fetch(32)?;
                self.execute_uncompressed(inst)?;
                self.pc += 4;
            }
            _ => return Err(Exception::InvalidInstruction("zero op code".to_owned()))
        }

        Ok(())
    }

    fn execute_compressed(&mut self, _inst: u64) -> Result<(), Exception> {
        todo!("compressed instruction")
    }

    fn execute_uncompressed(&mut self, inst: u64) -> Result<(), Exception> {
        self.xregs.write(0, 0);
        
        let opcode = inst & 0b1111111;
        let funct3 = (inst >> 12) & 0b111;
        let funct7 = inst >> 25;
        
        let rd  = (inst >> 7)  & 0b11111;
        let rs1 = (inst >> 15) & 0b11111;
        let rs2 = (inst >> 20) & 0b11111;

        println!("pc: {:016X} inst: {:08X}", self.pc, inst);
        
        match opcode {
            0b0110011 => { // OP
                println!("OP");
                self.xregs.write(rd, match (funct3, funct7) {
                    (0b000, 0) => { // ADD
                        self.xregs.read(rs1).wrapping_add(self.xregs.read(rs2))
                    }
                    (0b000, 0b0100000) => { // SUB
                        self.xregs.read(rs1).wrapping_sub(self.xregs.read(rs2))
                    }
                    (0b001, 0) => { // SLL
                        self.xregs.read(rs1) << (self.xregs.read(rs2) & 0b11111)
                    }
                    (0b010, 0) => { // SLT
                        if (self.xregs.read(rs1) as i64) < (self.xregs.read(rs2) as i64) {1} else {0}
                    }
                    (0b011, 0) => { // SLTU
                        if self.xregs.read(rs1) < self.xregs.read(rs2) {1} else {0}
                    }
                    (0b100, 0) => { // XOR
                        self.xregs.read(rs1) ^ self.xregs.read(rs2)
                    }
                    (0b101, 0) => { // SRL
                        self.xregs.read(rs1) >> (self.xregs.read(rs2) & 0b11111)
                    }
                    (0b101, 0b0100000) => { // SRA
                        ((self.xregs.read(rs1) as i64) >> (self.xregs.read(rs2) & 0b11111)) as u64
                    }
                    (0b110, 0) => { // OR
                        self.xregs.read(rs1) | self.xregs.read(rs2)
                    }
                    (0b111, 0) => { // AND
                        self.xregs.read(rs1) & self.xregs.read(rs2)
                    }
                    _ => return Err(Exception::InvalidInstruction("OP".to_owned()))
                });
            }
            0b0010011 => { // OP-IMM
                println!("OP-IMM");
                let imm = ((inst as i32 as i64) >> 20) as u64;

                self.xregs.write(rd, match (funct3, funct7)  {
                    (0b000, _) => { // ADDI
                        self.xregs.read(rs1).wrapping_add(imm)
                    }
                    (0b001, 0) => { // SLLI
                        self.xregs.read(rs1) << rs2
                    }
                    (0b010, _) => { // SLTI
                        if (self.xregs.read(rs1) as i64) < (imm as i64) {1} else {0}
                    }
                    (0b011, _) => { // SLTIU
                        if self.xregs.read(rs1) < imm {1} else {0}
                    }
                    (0b100, _) => { // XORI
                        self.xregs.read(rs1) ^ imm
                    }
                    (0b101, 0) => { // SRLI
                        self.xregs.read(rs1) >> rs2
                    }
                    (0b101, 0b0100000) => { // SRAI
                        ((self.xregs.read(rs1) as i64) >> rs2) as u64
                    }
                    (0b110, _) => { // ORI
                        self.xregs.read(rs1) | imm
                    }
                    (0b111, _) => { // ANDI
                        self.xregs.read(rs1) & imm
                    }
                    _ => return Err(Exception::InvalidInstruction("OP-IMM".to_owned()))
                });
            }
            0b0110111 => { // LUI
                println!("LUI");
                self.xregs.write(rd, (inst & 0xfffff000) as i32 as i64 as u64);
            }
            0b0010111 => { // AUIPC
                println!("AUIPC");
                self.xregs.write(rd, self.pc.wrapping_add((inst & 0xfffff000) as i32 as i64 as u64));
            }
            0b1101111 => { // JAL
                println!("JAL");
                let imm = (((inst & 0x80000000) as i32 as i64 >> 11) as u64) |
                (inst & 0xff000) |
                ((inst >> 9) & 0x800) |
                ((inst >> 20) & 0x7fe);
                
                self.xregs.write(rd, self.pc.wrapping_add(4));
                self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
            0b1100111 => { // JALR
                println!("JALR");
                let imm = ((inst as i32 as i64) >> 20) as u64;
                
                self.xregs.write(rd, self.pc.wrapping_add(4));
                self.pc = imm.wrapping_add(self.xregs.read(rs1)).wrapping_sub(4) & !1;
            }
            0b1100011 => { // BRANCH
                println!("BRANCH");
                if match funct3 {
                    0b000 => {self.xregs.read(rs1) == self.xregs.read(rs2)}
                    0b001 => {self.xregs.read(rs1) != self.xregs.read(rs2)}
                    0b100 => {self.xregs.read(rs1) <  self.xregs.read(rs2)}
                    0b101 => {self.xregs.read(rs1) >= self.xregs.read(rs2)}
                    0b110 => {(self.xregs.read(rs1) as u32) <  (self.xregs.read(rs2) as u32)}
                    0b111 => {(self.xregs.read(rs1) as u32) >= (self.xregs.read(rs2) as u32)}
                    _ => return Err(Exception::InvalidInstruction("BRANCH".to_owned()))
                } {
                    let imm = (((inst & 0x80000000) as i32 as i64 >> 19) as u64) |
                        ((inst & 0x80) << 4) |
                        ((inst >> 20) & 0x7e0) |
                        ((inst >> 7) & 0x1e);
                    
                    self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                }
            }
            0b0000011 => { // LOAD
                println!("LOAD");
                let imm = ((inst as i32 as i64) >> 20) as u64;
                let addr = imm.wrapping_add(self.xregs.read(rs1));
                self.xregs.write(rd, match funct3 {
                    0b000 => self.bus.read(addr, 8)? as i8 as i64 as u64,
                    0b001 => self.bus.read(addr, 16)? as i16 as i64 as u64,
                    0b010 => self.bus.read(addr, 32)? as i32 as i64 as u64,
                    0b011 => self.bus.read(addr, 32)?,
                    0b100 => self.bus.read(addr, 8)?,
                    0b101 => self.bus.read(addr, 16)?,
                    0b110 => self.bus.read(addr, 32)?,
                    _ => return Err(Exception::InvalidInstruction("LOAD".to_owned()))
                });
            }
            0b0100011 => { // STORE
                println!("STORE");
                let imm = (((inst & 0xfe000000) as i32 as i64 >> 20) as u64) | ((inst >> 7) & 0x1f);
                let addr = imm.wrapping_add(self.xregs.read(rs1));
                match funct3 {
                    0b000 => self.bus.write(addr, self.xregs.read(rs2), 8)?,
                    0b001 => self.bus.write(addr, self.xregs.read(rs2), 16)?,
                    0b010 => self.bus.write(addr, self.xregs.read(rs2), 32)?,
                    0b011 => self.bus.write(addr, self.xregs.read(rs2), 64)?,
                    _ => return Err(Exception::InvalidInstruction("STORE".to_owned()))
                }
            }
            0b0001111 => { // MISC-MEM
                println!("MISC-MEM");
            }
            0b1110011 => { // SYSTEM
                println!("SYSTEM");
                let csr = inst >> 20;

                match funct3 {
                    0b000 => return Err(Exception::InvalidInstruction("PRIV".to_owned())),
                    0b001 => { // CSRRW
                        let prev_val = self.csrs.read(csr);
                        self.csrs.write(csr, self.xregs.read(rs1));
                        self.xregs.write(rd, prev_val);
                    }
                    0b010 => { // CSRRS
                        let prev_val = self.csrs.read(csr);
                        self.csrs.write(csr, prev_val | self.xregs.read(rs1));
                        self.xregs.write(rd, prev_val);
                    }
                    0b011 => { // CSRRC
                        let prev_val = self.csrs.read(csr);
                        self.csrs.write(csr, prev_val & !self.xregs.read(rs1));
                        self.xregs.write(rd, prev_val);
                    }
                    0b101 => { // CSRRWI
                        let prev_val = self.csrs.read(csr);
                        self.csrs.write(csr, rs1);
                        self.xregs.write(rd, prev_val);
                    }
                    0b110 => { // CSRRSI
                        let prev_val = self.csrs.read(csr);
                        self.csrs.write(csr, prev_val | rs1);
                        self.xregs.write(rd, prev_val);
                    }
                    0b111 => { // CSRRCI
                        let prev_val = self.csrs.read(csr);
                        self.csrs.write(csr, prev_val & !rs1);
                        self.xregs.write(rd, prev_val);
                    }
                    _ => return Err(Exception::InvalidInstruction("SYSTEM".to_owned()))
                }
            }
            _ => return Err(Exception::InvalidInstruction("Not implemented".to_owned()))
        }

        Ok(())
    }
}