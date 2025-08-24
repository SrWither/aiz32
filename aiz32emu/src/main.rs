use std::fs;
use std::io::{self, Write};
use aiz32core::cpu::CPU;

fn main() {
    let program = fs::read("main.bin").expect("No se pudo leer el archivo binario");
    let ram_size = 1024;

    let pc_dir = ram_size as u32;
    let mut cpu = CPU::new(ram_size, program.clone(), 512, pc_dir); 

    let mut input = String::new();

    loop {
        let pc = cpu.regs.pc() as usize;

        let inst_bytes = &program[pc - ram_size..pc - ram_size + 4];
        let inst = u32::from_le_bytes([
            inst_bytes[0],
            inst_bytes[1],
            inst_bytes[2],
            inst_bytes[3],
        ]);

        println!("\n================= CPU DEBUG =================");
        println!("Instrucción @PC({:04X}): {:032b}", cpu.regs.pc(), inst);
        println!("---------------------------------------------");
        println!(
            " R1: {:08} | R2: {:08} | R3: {:08} | R4: {:08}",
            cpu.regs.get(1),
            cpu.regs.get(2),
            cpu.regs.get(3),
            cpu.regs.get(4)
        );
        println!(
            " R5: {:08} | R6: {:08} | SP: {:08} | FLAGS: {:08b}",
            cpu.regs.get(5),
            cpu.regs.get(6),
            cpu.regs.sp(),
            cpu.regs.flags()
        );
        println!("=============================================");

        cpu.step();

        // print!("Press enter to continue...");
        // io::stdout().flush().unwrap();
        // input.clear();
        // io::stdin().read_line(&mut input).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
