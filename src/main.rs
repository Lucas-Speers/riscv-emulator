use crate::{bus::DRAM_START, cpu::Cpu};

mod dram;
mod exception;
mod bus;
mod cpu;
mod rom;

fn main() {
    let mut cpu = Cpu::new();

    cpu.bus.dram.load(include_bytes!("../bbl.bin"));
    cpu.bus.dtb.load(include_bytes!("../sixtyfourmb.dtb"));
    cpu.set_pc(DRAM_START);

    loop {
        if let Err(exception) = cpu.execute() {
            cpu.handle_trap(exception);
        }
    }
}
