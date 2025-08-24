pub mod console;
pub mod gpu;

use aiz32core::cpu::CPU;
use std::fs;

use crate::console::Console;

fn main() {
    let program = fs::read("main.bin").expect("No se pudo leer el archivo binario");
    let ram_size = 1024;

    let pc_dir = ram_size as u32;
    let mut cpu = CPU::new(ram_size, program.clone(), 512, pc_dir);
    let mut console = Console::new();

    cpu.io.register_peripheral(&mut console);

    loop {
        println!(
            "R0: {:08X} R1: {:08X} R2: {:08X} R3: {:08X} R4: {:08X} SP: {:08X} PC: {:08X}\r",
            cpu.regs.get(0),
            cpu.regs.get(1),
            cpu.regs.get(2),
            cpu.regs.get(3),
            cpu.regs.get(4),
            cpu.regs.sp(),
            cpu.regs.pc()
        );

        cpu.step();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
