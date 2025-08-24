pub mod console;
pub mod gpu;

use aiz32core::{alu::Flags, cpu::CPU};
use std::fs;
use std::env;

use crate::console::Console;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        eprintln!("Uso: {} <binario> <ram_size> <sp_base> <debug>", args[0]);
        eprintln!("Ejemplo: {} main.bin 1024 512 1", args[0]);
        return;
    }

    let program_path = &args[1];
    let ram_size: usize = args[2].parse().expect("RAM size inválido");
    let sp_base: u32 = args[3].parse().expect("SP base inválida");
    let debug: bool = args[4].parse::<u8>().unwrap_or(0) != 0;

    let program = fs::read(program_path).expect("No se pudo leer el archivo binario");
    let pc_dir = ram_size as u32;
    let mut cpu = CPU::new(ram_size, program.clone(), sp_base, pc_dir);
    let mut console = Console::new();

    cpu.io.register_peripheral(&mut console);

    while !cpu.halted {
        cpu.step();

        if debug {
            let flags = Flags::from_u32(cpu.regs.flags());

            println!("==================== CPU STATE ====================");
            println!("Registers:");
            for i in 0..7 {
                print!("R{}: {:08X}  ", i, cpu.regs.get(i));
                if i == 3 {
                    println!();
                }
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
        }
    }
}
