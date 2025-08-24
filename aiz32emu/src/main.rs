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
        cpu.step();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
