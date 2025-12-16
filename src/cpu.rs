use crate::{bus::{Bus, DRAM_END, DTB_START}, exception::Exception};

struct Xregs {
    xregs: [u64;32]
}

impl Xregs {
    fn new() -> Self {
        Self { xregs: [0;32] }
    }

    fn get_abi(index: u64) -> &'static str {
        match index {
            0 => "zero",
            1 => "ra",
            2 => "sp",
            3 => "gp",
            4 => "tp",
            5 => "t0",
            6 => "t1",
            7 => "t2",
            8 => "s0/fp",
            9 => "s1",
            10 => "a0",
            11 => "a1",
            12 => "a2",
            13 => "a3",
            14 => "a4",
            15 => "a5",
            16 => "a6",
            17 => "a7",
            18 => "s2",
            19 => "s3",
            20 => "s4",
            21 => "s5",
            22 => "s6",
            23 => "s7",
            24 => "s8",
            25 => "s9",
            26 => "s10",
            27 => "s11",
            28 => "t3",
            29 => "t4",
            30 => "t5",
            31 => "t6",
            _ => "ERROR"
        }
    }

    fn read(&self, index: u64) -> u64 {
        // if index != 0 {println!("{} contains {:X}", Self::get_abi(index), self.xregs[index as usize])}
        self.xregs[index as usize]
    }

    fn write(&mut self, index: u64, value: u64) {
        // if index != 0 {println!("{value:X} -> {}", Self::get_abi(index))}
        self.xregs[index as usize] = value;
    }

    fn print_all(&self) {
        for index in 0..32 {
            println!("{}=0x{:X}", Self::get_abi(index), self.xregs[index as usize]);
        }
    }
}

mod csr {
    pub const MISA: u64 = 0x301;
    pub const MTVEC: u64 = 0x305;

    pub const MEPC: u64 = 0x341;
    pub const MCAUSE: u64 = 0x342;
    pub const MTVAL: u64 = 0x343;
}

struct Csrs {
    csrs: [u64;4096]
}

impl Csrs {
    fn new() -> Self {
        Self { csrs: [0;4096] }
    }

    fn get_name(index: u64) -> String {
        match index {
            0xf11 => "mvendorid",
            0xf12 => "marchid",
            0xf13 => "mimpid",
            0xf14 => "mhartid",
            0xf15 => "mconfigptr",

            0x300 => "mstatus",
            0x301 => "misa",
            0x302 => "medeleg",
            0x303 => "mideleg",
            0x304 => "mie",
            0x305 => "mtvec",
            0x306 => "mcounteren",
            0x310 => "mstatush",
            0x312 => "medelegh",

            0x340 => "mscratch",
            0x341 => "mepc",
            0x342 => "mcause",
            0x343 => "mtval",
            0x344 => "mip",
            0x34a => "mtinst",
            0x34b => "mtval2",

            0x3a0..=0x3af => return format!("pmpcfg{}", index-0x3a0),
            0x3b0..=0x3ef => return format!("pmpaddr{}", index-0x3b0),

            _ => return format!("UNKNOWN CSR {index:X}")
        }.to_string()
    }

    fn read(&self, index: u64) -> u64 {
        // println!("CSR READ:  {} {:b}", Self::get_name(index), self.csrs[index as usize]);
        self.csrs[index as usize]
    }

    fn write(&mut self, index: u64, value: u64) {
        // println!("CSR WRITE: {} {value:b}", Self::get_name(index));
        self.csrs[index as usize] = value;
    }
}

enum Mode {
    User,
    Supervisor,
    Machine,
}

pub struct Cpu {
    pub bus: Bus,
    xregs: Xregs,
    pc: u64,
    csrs: Csrs,
    mode: Mode,
    wfi: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let mut xregs = Xregs::new();
        xregs.write(2, DRAM_END);
        xregs.write(11, DTB_START);

        let mut csrs = Csrs::new();
        csrs.write(csr::MISA, 0x8000000000001000);

        Self {
            bus: Bus::new(),
            xregs,
            pc: 0,
            csrs,
            mode: Mode::Machine,
            wfi: false,
        }
    }

    pub fn set_pc(&mut self, pc: u64) {
        // if pc != self.pc + 4 {println!("{:X} -> pc", pc)}
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
                self.set_pc(self.pc.wrapping_add(2));
            },
            0b11 => {
                let inst = self.fetch(32)?;
                self.execute_uncompressed(inst)?;
                self.set_pc(self.pc.wrapping_add(4));
            }
            _ => return Err(Exception::IllegalInstruction("zero op code".to_owned()))
        }

        Ok(())
    }

    pub fn handle_trap(&mut self, exception: Exception) {
        println!("--- TRAP --- {exception:?}");
        self.csrs.write(csr::MCAUSE, exception.to_code());
        self.csrs.write(csr::MTVAL, 0);
        self.csrs.write(csr::MEPC, self.pc);

        self.set_pc(self.csrs.read(csr::MTVEC));

        // pause();
    }

    fn execute_compressed(&mut self, _inst: u64) -> Result<(), Exception> {
        Err(Exception::IllegalInstruction("Compressed Instruction".to_owned()))
    }

    fn execute_uncompressed(&mut self, inst: u64) -> Result<(), Exception> {
        self.xregs.write(0, 0);
        
        let opcode = inst & 0b1111111;
        let funct3 = (inst >> 12) & 0b111;
        let funct7 = inst >> 25;
        
        let rd  = (inst >> 7)  & 0b11111;
        let rs1 = (inst >> 15) & 0b11111;
        let rs2 = (inst >> 20) & 0b11111;

        // println!("\npc: {:X} inst: {:08X}", self.pc, inst);

        // simple debuging
        if inst == 0x00000013 {
            println!("NOP encountered");
            self.xregs.print_all();
            pause();
        }

        match inst {
            0x30200073 => { // MRET
                println!("MRET");
                self.set_pc(self.csrs.read(csr::MEPC)-4);
                self.mode = Mode::Supervisor; // TODO
                // self.xregs.print_all();
                return Ok(());
            }
            0x00000073 => { // ECALL
                match self.mode {
                    Mode::User => {
                        return Err(Exception::ECallFromU);
                    },
                    Mode::Supervisor => {
                        return Err(Exception::ECallFromS);
                    },
                    Mode::Machine => {
                        return Err(Exception::ECallFromM);
                    },
                }
            }
            _ => {}
        }
        
        match opcode {
            0b0110011 => { // OP
                self.xregs.write(rd, match (funct3, funct7) {
                    (0b000, 0) => { // ADD
                        self.xregs.read(rs1).wrapping_add(self.xregs.read(rs2))
                    }
                    (0b000, 0b0100000) => { // SUB
                        self.xregs.read(rs1).wrapping_sub(self.xregs.read(rs2))
                    }
                    (0b001, 0) => { // SLL
                        self.xregs.read(rs1) << (self.xregs.read(rs2) & 0b111111)
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
                        self.xregs.read(rs1) >> (self.xregs.read(rs2) & 0b111111)
                    }
                    (0b101, 0b0100000) => { // SRA
                        ((self.xregs.read(rs1) as i64) >> (self.xregs.read(rs2) & 0b111111)) as u64
                    }
                    (0b110, 0) => { // OR
                        self.xregs.read(rs1) | self.xregs.read(rs2)
                    }
                    (0b111, 0) => { // AND
                        self.xregs.read(rs1) & self.xregs.read(rs2)
                    }
                    (0b000, 1) => { // MUL
                        (self.xregs.read(rs1) as i64).wrapping_mul(self.xregs.read(rs2) as i64) as u64
                    }
                    (0b001, 1) => { // MULH
                        ((self.xregs.read(rs1) as i128).wrapping_mul(self.xregs.read(rs2) as i128) >> 64) as u64
                    }
                    (0b010, 1) => { // MULHSU
                        return Err(Exception::IllegalInstruction("MULHU".to_owned()))
                    }
                    (0b011, 1) => { // MULHU
                        ((self.xregs.read(rs1) as u128).wrapping_mul(self.xregs.read(rs2) as u128) >> 64) as u64
                    }
                    (0b100, 1) => { // DIV
                        (self.xregs.read(rs1) as i64).wrapping_div(self.xregs.read(rs2) as i64) as u64
                    }
                    (0b101, 1) => { // DIVU
                        self.xregs.read(rs1).wrapping_div(self.xregs.read(rs2))
                    }
                    (0b110, 1) => { // REM
                        (self.xregs.read(rs1) as i64).wrapping_rem(self.xregs.read(rs2) as i64) as u64
                    }
                    (0b111, 1) => { // REMU
                        self.xregs.read(rs1).wrapping_rem(self.xregs.read(rs2))
                    }
                    _ => return Err(Exception::IllegalInstruction("OP".to_owned()))
                });
            }
            0b0111011 => { // OP-32
                self.xregs.write(rd, match (funct3, funct7) {
                    (0b000, 0) => { // ADDW
                        (self.xregs.read(rs1) as i32).wrapping_add(self.xregs.read(rs2) as i32) as i64 as u64
                    }
                    (0b000, 0b0100000) => { // SUBW
                        (self.xregs.read(rs1) as i32).wrapping_sub(self.xregs.read(rs2) as i32) as i64 as u64
                    }
                    (0b001, 0) => { // SLLW
                        ((self.xregs.read(rs1) as i32) << (self.xregs.read(rs2) & 0b111111)) as i64 as u64
                    }
                    (0b101, 0) => { // SRLW
                        ((self.xregs.read(rs1) as i32) >> (self.xregs.read(rs2) & 0b111111)) as i64 as u64
                    }
                    (0b101, 0b0100000) => { // SRAW
                        ((self.xregs.read(rs1) as i32) >> (self.xregs.read(rs2) & 0b111111)) as i64 as u64
                    }
                    (0b000, 1) => { // MULW
                        (self.xregs.read(rs1) as i32).wrapping_mul(self.xregs.read(rs2) as i32) as i64 as u64
                    }
                    (0b100, 1) => { // DIVW
                        (self.xregs.read(rs1) as i32).wrapping_div(self.xregs.read(rs2) as i32) as i64 as u64
                    }
                    (0b101, 1) => { // DIVUW
                        (self.xregs.read(rs1) as u32).wrapping_div(self.xregs.read(rs2) as u32) as u64
                    }
                    (0b110, 1) => { // REM
                        (self.xregs.read(rs1) as i32).wrapping_rem(self.xregs.read(rs2) as i32) as i64 as u64
                    }
                    (0b111, 1) => { // REMU
                        (self.xregs.read(rs1) as u32).wrapping_rem(self.xregs.read(rs2) as u32) as u64
                    }
                    _ => return Err(Exception::IllegalInstruction("OP".to_owned()))
                } as i32 as i64 as u64);
            }
            0b0010011 => { // OP-IMM
                let imm = ((inst as i32 as i64) >> 20) as u64;

                self.xregs.write(rd, match (funct3, funct7)  {
                    (0b000, _) => { // ADDI
                        self.xregs.read(rs1).wrapping_add(imm)
                    }
                    // the shift amout leaks into funct7 by 1 bit
                    (0b001, 0) | (0b001, 1) => { // SLLI
                        self.xregs.read(rs1) << ((inst >> 20) & 0b111111)
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
                    (0b101, 0) | (0b101, 1) => { // SRLI
                        self.xregs.read(rs1) >> ((inst >> 20) & 0b111111)
                    }
                    (0b101, 0b0100000) | (0b101, 0b0100001) => { // SRAI
                        ((self.xregs.read(rs1) as i64) >> ((inst >> 20) & 0b111111)) as u64
                    }
                    (0b110, _) => { // ORI
                        self.xregs.read(rs1) | imm
                    }
                    (0b111, _) => { // ANDI
                        self.xregs.read(rs1) & imm
                    }
                    _ => return Err(Exception::IllegalInstruction("OP-IMM".to_owned()))
                });
            }
            0b0011011 => { // OP-IMM-32
                let imm = ((inst as i32 as i64) >> 20) as u64;

                self.xregs.write(rd, match (funct3, funct7)  {
                    (0b000, _) => { // ADDIW
                        self.xregs.read(rs1).wrapping_add(imm)
                    }
                    (0b001, 0) => { // SLLIW
                        self.xregs.read(rs1) << ((inst >> 20) & 0b11111)
                    }
                    (0b101, 0) => { // SRLIW
                        self.xregs.read(rs1) >> ((inst >> 20) & 0b11111)
                    }
                    (0b101, 0b0100000) => { // SRAIW
                        ((self.xregs.read(rs1) as i64) >> ((inst >> 20) & 0b11111)) as u64
                    }
                    _ => return Err(Exception::IllegalInstruction("OP-IMM-32".to_owned()))
                } as i32 as i64 as u64);
            }
            0b0110111 => { // LUI
                self.xregs.write(rd, (inst & 0xfffff000) as i32 as i64 as u64);
            }
            0b0010111 => { // AUIPC
                self.xregs.write(rd, self.pc.wrapping_add((inst & 0xfffff000) as i32 as i64 as u64));
            }
            0b1101111 => { // JAL
                let imm = (((inst & 0x80000000) as i32 as i64 >> 11) as u64) |
                (inst & 0xff000) |
                ((inst >> 9) & 0x800) |
                ((inst >> 20) & 0x7fe);
                
                self.xregs.write(rd, self.pc.wrapping_add(4));
                self.set_pc(self.pc.wrapping_add(imm).wrapping_sub(4));
            }
            0b1100111 => { // JALR
                let imm = ((inst as i32 as i64) >> 20) as u64;
                
                self.xregs.write(rd, self.pc.wrapping_add(4));
                self.set_pc(imm.wrapping_add(self.xregs.read(rs1)).wrapping_sub(4) & !1);
            }
            0b1100011 => { // BRANCH
                if match funct3 {
                    0b000 => {self.xregs.read(rs1) == self.xregs.read(rs2)}
                    0b001 => {self.xregs.read(rs1) != self.xregs.read(rs2)}
                    0b100 => {self.xregs.read(rs1) <  self.xregs.read(rs2)}
                    0b101 => {self.xregs.read(rs1) >= self.xregs.read(rs2)}
                    0b110 => {(self.xregs.read(rs1) as u32) <  (self.xregs.read(rs2) as u32)}
                    0b111 => {(self.xregs.read(rs1) as u32) >= (self.xregs.read(rs2) as u32)}
                    _ => return Err(Exception::IllegalInstruction("BRANCH".to_owned()))
                } {
                    let imm = (((inst & 0x80000000) as i32 as i64 >> 19) as u64) |
                        ((inst & 0x80) << 4) |
                        ((inst >> 20) & 0x7e0) |
                        ((inst >> 7) & 0x1e);
                    
                    self.set_pc(self.pc.wrapping_add(imm).wrapping_sub(4));
                }
            }
            0b0000011 => { // LOAD
                let imm = ((inst as i32 as i64) >> 20) as u64;
                let addr = imm.wrapping_add(self.xregs.read(rs1));
                self.xregs.write(rd, match funct3 {
                    // LB
                    0b000 => self.bus.read(addr, 8)? as i8 as i64 as u64,
                    // LH
                    0b001 => self.bus.read(addr, 16)? as i16 as i64 as u64,
                    // LW
                    0b010 => self.bus.read(addr, 32)? as i32 as i64 as u64,
                    // LD
                    0b011 => self.bus.read(addr, 32)?,
                    // LBU
                    0b100 => self.bus.read(addr, 8)?,
                    // LHU
                    0b101 => self.bus.read(addr, 16)?,
                    // LWU
                    0b110 => self.bus.read(addr, 32)?,
                    _ => return Err(Exception::IllegalInstruction("LOAD".to_owned()))
                });
            }
            0b0100011 => { // STORE
                let imm = (((inst & 0xfe000000) as i32 as i64 >> 20) as u64) | ((inst >> 7) & 0x1f);
                let addr = imm.wrapping_add(self.xregs.read(rs1));
                match funct3 {
                    // SB
                    0b000 => self.bus.write(addr, self.xregs.read(rs2), 8)?,
                    // SH
                    0b001 => self.bus.write(addr, self.xregs.read(rs2), 16)?,
                    // SW
                    0b010 => self.bus.write(addr, self.xregs.read(rs2), 32)?,
                    // SD
                    0b011 => self.bus.write(addr, self.xregs.read(rs2), 64)?,
                    _ => return Err(Exception::IllegalInstruction("STORE".to_owned()))
                }
            }
            0b0001111 => {} // MISC-MEM
            0b1110011 => { // SYSTEM
                match inst {
                    0x10500073 => {
                        self.wfi = true;
                        panic!("WFI instruction waits for external interupts which are not implemented");
                    }
                    _ => ()
                }
                let csr = inst >> 20;

                let prev_val = self.csrs.read(csr);
                let new_val = match funct3 {
                    0b000 => {
                        println!("{inst:X}");
                        return Err(Exception::IllegalInstruction("PRIV".to_owned()))
                    },
                    0b001 => { // CSRRW
                        self.xregs.read(rs1)
                    }
                    0b010 => { // CSRRS
                        prev_val | self.xregs.read(rs1)
                    }
                    0b011 => { // CSRRC
                        prev_val & !self.xregs.read(rs1)
                    }
                    0b101 => { // CSRRWI
                        rs1
                    }
                    0b110 => { // CSRRSI
                        prev_val | rs1
                    }
                    0b111 => { // CSRRCI
                        prev_val & !rs1
                    }
                    _ => return Err(Exception::IllegalInstruction("SYSTEM".to_owned()))
                };
                if new_val != prev_val {
                    self.csrs.write(csr, new_val);
                }
                self.xregs.write(rd, prev_val);
            }
            0b0101111 => { // AMO
                println!("AMO {inst:X} {} {} {}", Xregs::get_abi(rd), Xregs::get_abi(rs1), Xregs::get_abi(rs2));
                let value = match funct3 {
                    0b010 => self.bus.read(rs1, 32)? as i32 as i64,
                    0b011 => self.bus.read(rs1, 64)? as i64,
                    _ => return Err(Exception::IllegalInstruction("AMO WRONG SIZE".to_string()))
                };

                self.xregs.write(rd, value as u64);

                let rs2 = rs2 as i64;
                let new_value = match funct7 >> 2 {
                    0b00010 => panic!(),
                    0b00011 => panic!(),
                    0b00001 => rs2, // could be wrong
                    0b00000 => value.wrapping_add(rs2),
                    0b00100 => value ^ rs2,
                    0b01100 => value & rs2,
                    0b01000 => value | rs2,
                    0b10000 => value.min(rs2),
                    0b10100 => value.max(rs2),
                    0b11000 => (value as u64).min(rs2 as u64) as i64,
                    0b11100 => (value as u64).max(rs2 as u64) as i64,
                    _ => return Err(Exception::IllegalInstruction("AMO".to_string()))
                } as u64;

                match funct3 {
                    0b010 => self.bus.write(rs1, new_value, 32)?,
                    0b011 => self.bus.write(rs1, new_value, 64)?,
                    _ => unreachable!()
                };
            }
            _ => return Err(Exception::IllegalInstruction("Not implemented".to_owned()))
        }

        Ok(())
    }
}

fn pause() {
    use std::io::Read as _;
    use std::io::Write as _;

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    write!(stdout, "Press any key to continue...\n").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}