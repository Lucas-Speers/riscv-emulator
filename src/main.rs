use crate::{bus::DRAM_START, cpu::Cpu};

mod dram;
mod exception;
mod bus;
mod cpu;
mod rom;

fn main() {
    let mut cpu = Cpu::new();

    cpu.bus.load_dram(include_bytes!("../bbl.bin"));
    cpu.set_pc(DRAM_START);

    loop {
        // cpu.print_regs();
        cpu.execute().unwrap();
    }
}
