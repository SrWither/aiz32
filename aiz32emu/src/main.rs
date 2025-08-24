pub mod console;
pub mod gpu;

use aiz32core::{alu::Flags, cpu::CPU};
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

        let flags = Flags::from_u32(cpu.regs.flags());

        println!("==================== CPU STATE ====================");
        println!("Registers:");
        for i in 0..7 {
            print!("R{}: {:08X}  ", i, cpu.regs.get(i));
            if i == 3 {
                println!();
            } // salto de línea después de R3
        }
        println!();
        println!(
            "SP: {:08X}  PC: {:08X}  LR: {:08X}  (dec: SP={}, PC={}, LR={})",
            cpu.regs.sp(),
            cpu.regs.pc(),
            cpu.regs.lr(),
            cpu.regs.sp(),
            cpu.regs.pc(),
            cpu.regs.lr()
        );

        // imprimir cada flag por separado
        println!("Flags:");
        println!(
            "Z:{} C:{} O:{} S:{} G:{} E:{} NE:{} L:{} GE:{} LE:{}",
            flags.zero as u8,
            flags.carry as u8,
            flags.overflow as u8,
            flags.sign as u8,
            flags.greater as u8,
            flags.equal as u8,
            flags.not_equal as u8,
            flags.less as u8,
            flags.greater_equal as u8,
            flags.less_equal as u8,
        );
        println!("==================================================\n");

        // std::thread::sleep(std::time::Duration::from_millis(100));
        // press enter to continue
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input).unwrap();
    }
}
