use crate::{bus::DRAM_START, cpu::Cpu, exception::Exception};

mod dram;
mod exception;
mod bus;
mod cpu;
mod rom;
mod uart;
mod clint;

fn main() {
    let mut cpu = Cpu::new();

    cpu.bus.dram.load(include_bytes!("../riscv-pk/build/bbl.bin"));
    cpu.bus.dtb.load(include_bytes!("../emulator.dtb"));
    cpu.set_pc(DRAM_START);

    loop {
        if let Err(exception) = cpu.execute() {
            match exception {
                Exception::IllegalInstruction(_) => todo!(),
                Exception::LoadAddressMisaligned => todo!(),
                Exception::LoadAccessFault => todo!(),
                Exception::StoreAddressMisaligned => todo!(),
                Exception::StoreAccessFault => todo!(),
                Exception::ECallFromU => (),
                Exception::ECallFromS => (),
                Exception::ECallFromM => (),
                Exception::HardwareError => todo!(),
            }
            cpu.handle_trap(exception);
        }
    }
}
