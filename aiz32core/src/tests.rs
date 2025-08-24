#[cfg(test)]
mod tests {
    use crate::alu::Flags;
    use crate::cpu::CPU;
    use crate::instruction::{Instruction, Opcode};

    #[test]
    fn test_cpu_initialization() {
        let cpu = CPU::new(1024, vec![], 0, 0);
        assert_eq!(cpu.mem.ram_size(), 1024);
        assert_eq!(cpu.mem.rom_size(), 0);
        assert_eq!(cpu.cycle_count, 0);
        assert!(!cpu.halted);
    }

    #[test]
    fn test_cpu_rtype_add() {
        let mut cpu = CPU::new(1024, vec![0, 0, 0, 1], 0, 0); // ADD R0, R0, R0
        cpu.regs.set(0, 5);
        cpu.step();
        assert_eq!(cpu.regs.get(0), 10);
        assert_eq!(cpu.cycle_count, 1);
        let flags = Flags::from_u32(cpu.regs.flags());
        assert!(!flags.zero);
    }

    #[test]
    fn test_cpu_rtype_sub_zero_flag() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);
        cpu.regs.set(1, 5);
        cpu.regs.set(2, 5);
        let instr = Instruction::R {
            opcode: Opcode::SUB,
            rd: 0,
            rs1: 1,
            rs2: 2,
        };
        cpu.execute(instr);

        assert_eq!(cpu.regs.get(0), 0);
        let flags = Flags::from_u32(cpu.regs.flags());
        assert!(flags.zero);
    }

    #[test]
    fn test_cpu_itype_addi() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);
        cpu.regs.set(0, 7);
        let instr = Instruction::I {
            opcode: Opcode::ADDI,
            rd: 1,
            rs1: 0,
            imm: 5,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.get(1), 12);

        let flags = Flags::from_u32(cpu.regs.flags());
        assert!(!flags.zero);
    }

    #[test]
    fn test_cpu_flags_carry_zero() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);
        cpu.regs.set(0, 0xFFFFFFFF);
        let instr = Instruction::I {
            opcode: Opcode::ADDI,
            rd: 1,
            rs1: 0,
            imm: 1,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.get(1), 0);

        let flags = Flags::from_u32(cpu.regs.flags());
        assert!(flags.carry);
        assert!(flags.zero);
    }

    #[test]
    fn test_cpu_memorybus_ram_read_write() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);
        cpu.mem.write32(0, 0x12345678);
        assert_eq!(cpu.mem.read32(0), 0x12345678);
        cpu.mem.write16(2, 0xABCD);
        assert_eq!(cpu.mem.read16(2), 0xABCD);
        assert_eq!(cpu.mem.read8(3), 0xAB);
    }

    #[test]
    fn test_cpu_memorybus_rom_read() {
        let rom_data = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let cpu = CPU::new(1024, rom_data.clone(), 0, 0);

        for i in 0..4 {
            assert_eq!(cpu.mem.read8(i as u32), rom_data[i]);
        }
        assert_eq!(cpu.mem.read16(0), 0xADDE);
        assert_eq!(cpu.mem.read32(0), 0xEFBEADDE);
    }

    #[test]
    fn test_cpu_halt() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);
        cpu.halted = true;
        let pc_before = cpu.regs.pc();
        cpu.step();
        assert_eq!(cpu.regs.pc(), pc_before);
        assert_eq!(cpu.cycle_count, 0);
    }

    #[test]
    fn test_jmp() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);
        cpu.regs.set_pc(10);
        let instr = Instruction::J {
            opcode: Opcode::JMP,
            offset: 100, // relativo
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.pc(), 10 + 100);
    }

    #[test]
    fn test_jz() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);
        cpu.regs.set_pc(20);

        cpu.regs.set_flags(0x01); // zero = 1
        let instr = Instruction::J {
            opcode: Opcode::JZ,
            offset: 50,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.pc(), 20 + 50);

        cpu.regs.set_pc(20);
        cpu.regs.set_flags(0x00); // zero = 0
        let instr = Instruction::J {
            opcode: Opcode::JZ,
            offset: 60,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.pc(), 20); // no cambia
    }

    #[test]
    fn test_jnz() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);
        cpu.regs.set_pc(30);

        cpu.regs.set_flags(0x00); // zero = 0
        let instr = Instruction::J {
            opcode: Opcode::JNZ,
            offset: 30,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.pc(), 30 + 30);

        cpu.regs.set_pc(30);
        cpu.regs.set_flags(0x01); // zero = 1
        let instr = Instruction::J {
            opcode: Opcode::JNZ,
            offset: 40,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.pc(), 30); // no cambia
    }

    #[test]
    fn test_conditional_flags() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);
        cpu.regs.set_pc(50);
        cpu.regs.set_flags(0x10); // greater
        let instr = Instruction::J {
            opcode: Opcode::JGT,
            offset: 123,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.pc(), 50 + 123);

        cpu.regs.set_pc(50);
        cpu.regs.set_flags(0x80); // less
        let instr = Instruction::J {
            opcode: Opcode::JLT,
            offset: 200,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.pc(), 50 + 200);

        cpu.regs.set_pc(50);
        cpu.regs.set_flags(0x00); // neither
        let instr = Instruction::J {
            opcode: Opcode::JGE,
            offset: 250,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.pc(), 50); // no cambia
    }

    #[test]
    fn test_call_ret() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);
        cpu.regs.set_pc(10);
        cpu.regs.set_sp(1024);

        let call_instr = Instruction::J {
            opcode: Opcode::CALL,
            offset: 200,
        };
        cpu.execute(call_instr);
        assert_eq!(cpu.regs.pc(), 10 + 200);
        assert_eq!(cpu.regs.sp(), 1020);
        assert_eq!(cpu.mem.read32(1020), 10 + 1); // dirección siguiente guardada

        let ret_instr = Instruction::J {
            opcode: Opcode::RET,
            offset: 0,
        };
        cpu.execute(ret_instr);
        assert_eq!(cpu.regs.pc(), 10 + 1);
        assert_eq!(cpu.regs.sp(), 1024);
    }

    #[test]
    fn test_halt() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);
        let instr = Instruction::J {
            opcode: Opcode::HALT,
            offset: 0,
        };
        cpu.execute(instr);
        assert!(cpu.halted);
    }

    #[test]
    fn test_jmp_signed_offset() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);

        // PC inicial 100, offset positivo
        cpu.regs.set_pc(100);
        let instr = Instruction::J {
            opcode: Opcode::JMP,
            offset: 50,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.pc(), 150);

        // PC inicial 100, offset negativo
        cpu.regs.set_pc(100);
        let instr = Instruction::J {
            opcode: Opcode::JMP,
            offset: -30i32 as u32,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.pc(), 70);
    }

    #[test]
    fn test_conditional_signed_offset() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);

        // JZ positivo
        cpu.regs.set_pc(200);
        cpu.regs.set_flags(0x01); // zero
        let instr = Instruction::J {
            opcode: Opcode::JZ,
            offset: 25,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.pc(), 225);

        // JZ negativo
        cpu.regs.set_pc(200);
        cpu.regs.set_flags(0x01); // zero
        let instr = Instruction::J {
            opcode: Opcode::JZ,
            offset: -50i32 as u32,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.pc(), 150);

        // JNZ positivo
        cpu.regs.set_pc(300);
        cpu.regs.set_flags(0x00); // not zero
        let instr = Instruction::J {
            opcode: Opcode::JNZ,
            offset: 40,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.pc(), 340);

        // JNZ negativo
        cpu.regs.set_pc(300);
        cpu.regs.set_flags(0x00); // not zero
        let instr = Instruction::J {
            opcode: Opcode::JNZ,
            offset: -100i32 as u32,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.pc(), 200);
    }

    #[test]
    fn test_load_store_byte() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);
        cpu.regs.set(1, 0);
        cpu.regs.set(2, 0xAB);

        let instr = Instruction::Mem {
            opcode: Opcode::STB,
            rd: 2,
            rs1: 1,
            imm: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.mem.read8(0), 0xAB);

        cpu.regs.set(3, 0);
        let instr = Instruction::Mem {
            opcode: Opcode::LDB,
            rd: 3,
            rs1: 1,
            imm: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.get(3), 0xFFFFFFAB);

        let instr = Instruction::Mem {
            opcode: Opcode::LDBU,
            rd: 3,
            rs1: 1,
            imm: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.get(3), 0xAB);
    }

    #[test]
    fn test_load_store_half() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);
        cpu.regs.set(1, 0);
        cpu.regs.set(2, 0xABCD);

        let instr = Instruction::Mem {
            opcode: Opcode::STH,
            rd: 2,
            rs1: 1,
            imm: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.mem.read16(0), 0xABCD);

        cpu.regs.set(3, 0);
        let instr = Instruction::Mem {
            opcode: Opcode::LDH,
            rd: 3,
            rs1: 1,
            imm: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.get(3), 0xFFFFABCD);

        let instr = Instruction::Mem {
            opcode: Opcode::LDHU,
            rd: 3,
            rs1: 1,
            imm: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.get(3), 0xABCD);
    }

    #[test]
    fn test_load_store_word() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);
        cpu.regs.set(1, 0); // base addr
        cpu.regs.set(2, 0x12345678); // value

        let instr = Instruction::Mem {
            opcode: Opcode::STW,
            rd: 2,
            rs1: 1,
            imm: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.mem.read32(0), 0x12345678);

        cpu.regs.set(3, 0);
        let instr = Instruction::Mem {
            opcode: Opcode::LDW,
            rd: 3,
            rs1: 1,
            imm: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.get(3), 0x12345678);
    }

    #[test]
    fn test_ldlr_stlr() {
        let mut cpu = CPU::new(1024, vec![], 0, 0);
        cpu.regs.set(1, 0);
        cpu.regs.set(2, 0xDEADBEEF);

        let instr = Instruction::Mem {
            opcode: Opcode::STLR,
            rd: 2,
            rs1: 1,
            imm: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.mem.read32(0), 0xDEADBEEF);

        cpu.regs.set(3, 0);
        let instr = Instruction::Mem {
            opcode: Opcode::LDLR,
            rd: 3,
            rs1: 1,
            imm: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.get(3), 0xDEADBEEF);
    }

    #[test]
    fn test_sys_li_lui() {
        let mut cpu = CPU::new(0, vec![], 0, 0);

        let instr_li = Instruction::Sys {
            opcode: Opcode::LI,
            rd: 0,
            imm: 0x1234,
        };
        cpu.execute(instr_li);
        assert_eq!(cpu.regs.get(0), 0x1234);

        let instr_lui = Instruction::Sys {
            opcode: Opcode::LUI,
            rd: 1,
            imm: 0x5678,
        };
        cpu.execute(instr_lui);
        assert_eq!(cpu.regs.get(1), 0x56780000);
    }

    #[test]
    fn test_sys_mov_movpc() {
        let mut cpu = CPU::new(0, vec![], 0, 0);
        cpu.regs.set(2, 0x42);

        let instr_mov = Instruction::Sys {
            opcode: Opcode::MOV,
            rd: 0,
            imm: 2,
        };
        cpu.execute(instr_mov);
        assert_eq!(cpu.regs.get(0), 0x42);

        cpu.regs.set_pc(0x100);
        let instr_movpc = Instruction::Sys {
            opcode: Opcode::MOVPC,
            rd: 1,
            imm: 0,
        };
        cpu.execute(instr_movpc);
        assert_eq!(cpu.regs.get(1), 0x100);
    }

    #[test]
    fn test_sys_mtsr_mfsr() {
        let mut cpu = CPU::new(0, vec![], 0, 0);
        cpu.regs.set(0, 0xABCD);

        let instr_mtsr = Instruction::Sys {
            opcode: Opcode::MTSR,
            rd: 0,
            imm: 0,
        };
        cpu.execute(instr_mtsr);
        assert_eq!(cpu.regs.flags(), 0xABCD);

        let instr_mfsr = Instruction::Sys {
            opcode: Opcode::MFSR,
            rd: 1,
            imm: 0,
        };
        cpu.execute(instr_mfsr);
        assert_eq!(cpu.regs.get(1), 0xABCD);
    }

    #[test]
    fn test_sys_movsp_setsp() {
        let mut cpu = CPU::new(0, vec![], 0, 0);
        cpu.regs.set_sp(0x200);

        let instr_movsp = Instruction::Sys {
            opcode: Opcode::MOVSP,
            rd: 0,
            imm: 0,
        };
        cpu.execute(instr_movsp);
        assert_eq!(cpu.regs.get(0), 0x200);

        cpu.regs.set(0, 0x300);
        let instr_setsp = Instruction::Sys {
            opcode: Opcode::SETSP,
            rd: 0,
            imm: 0,
        };
        cpu.execute(instr_setsp);
        assert_eq!(cpu.regs.sp(), 0x300);
    }

    #[test]
    fn test_fp_arithmetic() {
        let mut cpu = CPU::new(1024, vec![], 0x1000, 0x0);

        cpu.regs.fregs[0] = 1.5;
        cpu.regs.fregs[1] = 2.5;

        // FADD
        let instr = Instruction::FP {
            opcode: Opcode::FADD,
            rd: 2,
            rs1: 0,
            rs2: 1,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.fregs[2], 4.0);

        // FSUB
        let instr = Instruction::FP {
            opcode: Opcode::FSUB,
            rd: 2,
            rs1: 1,
            rs2: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.fregs[2], 1.0);

        // FMUL
        let instr = Instruction::FP {
            opcode: Opcode::FMUL,
            rd: 2,
            rs1: 0,
            rs2: 1,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.fregs[2], 3.75);

        // FDIV
        let instr = Instruction::FP {
            opcode: Opcode::FDIV,
            rd: 2,
            rs1: 1,
            rs2: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.fregs[2], 2.5 / 1.5);
    }

    #[test]
    fn test_fp_comparisons() {
        let mut cpu = CPU::new(1024, vec![], 0x1000, 0x0);

        cpu.regs.fregs[0] = 3.0;
        cpu.regs.fregs[1] = 3.0;
        cpu.regs.fregs[2] = 4.0;

        // FCMP
        let instr = Instruction::FP {
            opcode: Opcode::FCMP,
            rd: 0,
            rs1: 0,
            rs2: 1,
        };
        cpu.execute(instr);
        let flags = Flags::from_u32(cpu.regs.flags());
        assert!(flags.zero && !flags.less && !flags.greater);

        // FEQ
        let instr = Instruction::FP {
            opcode: Opcode::FEQ,
            rd: 3,
            rs1: 0,
            rs2: 1,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.get(3), 1);

        // FLT
        let instr = Instruction::FP {
            opcode: Opcode::FLT,
            rd: 3,
            rs1: 0,
            rs2: 2,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.get(3), 1);

        // FGT
        let instr = Instruction::FP {
            opcode: Opcode::FGT,
            rd: 3,
            rs1: 2,
            rs2: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.get(3), 1);
    }

    #[test]
    fn test_fp_conversion_and_mov() {
        let mut cpu = CPU::new(1024, vec![], 0x1000, 0x0);

        cpu.regs.fregs[0] = 3.5;
        cpu.regs.set(0, 7);

        // FTOI
        let instr = Instruction::FP {
            opcode: Opcode::FTOI,
            rd: 1,
            rs1: 0,
            rs2: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.get(1), 3);

        // ITOF
        let instr = Instruction::FP {
            opcode: Opcode::ITOF,
            rd: 1,
            rs1: 0,
            rs2: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.fregs[1], 7.0);

        // FMOV
        let instr = Instruction::FP {
            opcode: Opcode::FMOV,
            rd: 2,
            rs1: 1,
            rs2: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.fregs[2], 7.0);
    }

    #[test]
    fn test_fp_memory_load_store() {
        let mut cpu = CPU::new(1024, vec![], 0x1000, 0x0);

        cpu.regs.fregs[0] = 5.5;
        cpu.regs.set(1, 100);

        // FST
        let instr = Instruction::FP {
            opcode: Opcode::FST,
            rd: 0,
            rs1: 1,
            rs2: 0,
        };
        cpu.execute(instr);
        let bits = cpu.mem.read32(100);
        assert_eq!(f32::from_bits(bits), 5.5);

        // FLD
        cpu.regs.fregs[0] = 0.0;
        let instr = Instruction::FP {
            opcode: Opcode::FLD,
            rd: 0,
            rs1: 1,
            rs2: 0,
        };
        cpu.execute(instr);
        assert_eq!(cpu.regs.fregs[0], 5.5);
    }

    #[test]
    fn test_io_out_in_basic() {
        let mut cpu = CPU::new(1024, vec![], 0x1000, 0);

        // escribimos en el puerto 0x1234
        cpu.regs.set(0, 0xDEADBEEF);
        let port: u16 = 0x1234;

        // OUT
        cpu.execute(crate::instruction::Instruction::IO {
            opcode: Opcode::OUT,
            rd: 0,
            port,
        });
        assert_eq!(cpu.io.read(port), 0xDEADBEEF);

        // limpiamos registro y hacemos IN
        cpu.regs.set(1, 0);
        cpu.execute(crate::instruction::Instruction::IO {
            opcode: Opcode::IN,
            rd: 1,
            port,
        });
        assert_eq!(cpu.regs.get(1), 0xDEADBEEF);
    }

    #[test]
    fn test_io_multiple_ports() {
        let mut cpu = CPU::new(1024, vec![], 0x1000, 0);

        let values = [(0x0000, 123), (0x8000, 456), (0xFFFF, 789)];

        // escribir en varios puertos
        for (port, val) in &values {
            cpu.regs.set(0, *val);
            cpu.execute(crate::instruction::Instruction::IO {
                opcode: Opcode::OUT,
                rd: 0,
                port: *port as u16,
            });
        }

        // leer y verificar
        for (port, val) in &values {
            cpu.regs.set(1, 0);
            cpu.execute(crate::instruction::Instruction::IO {
                opcode: Opcode::IN,
                rd: 1,
                port: *port as u16,
            });
            assert_eq!(cpu.regs.get(1), *val);
        }
    }

    #[test]
    fn test_io_overwrite_port() {
        let mut cpu = CPU::new(1024, vec![], 0x1000, 0);

        let port: u16 = 0x42;

        cpu.regs.set(0, 0xAAAA);
        cpu.execute(crate::instruction::Instruction::IO {
            opcode: Opcode::OUT,
            rd: 0,
            port,
        });
        assert_eq!(cpu.io.read(port), 0xAAAA);

        // sobrescribir mismo puerto
        cpu.regs.set(0, 0x5555);
        cpu.execute(crate::instruction::Instruction::IO {
            opcode: Opcode::OUT,
            rd: 0,
            port,
        });
        assert_eq!(cpu.io.read(port), 0x5555);
    }
}
