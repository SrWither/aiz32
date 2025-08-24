use crate::alu::{ALU, ALUOp, ALUResult, Flags};
use crate::instruction::{Instruction, Opcode};
use crate::memory::{IO, MemoryBus};
use crate::registers::RegisterBank;

fn sign_extend_24(offset: u32) -> i32 {
    if offset & 0x800000 != 0 {
        (offset | 0xFF000000) as i32
    } else {
        offset as i32
    }
}

pub struct CPU<'a> {
    pub regs: RegisterBank,
    pub mem: MemoryBus,
    pub alu: ALU,
    pub cycle_count: u64,
    pub halted: bool,
    pub io: IO<'a>,
}

impl<'a> CPU<'a> {
    pub fn new(ram_size: usize, rom_contents: Vec<u8>, sp_dir: u32, pc_dir: u32) -> Self {
        Self {
            regs: RegisterBank::new(pc_dir, sp_dir),
            mem: MemoryBus::new(ram_size, rom_contents),
            alu: ALU::new(),
            cycle_count: 0,
            halted: false,
            io: IO::new(),
        }
    }

    pub fn step(&mut self) {
        if self.halted {
            return;
        }

        let pc = self.regs.pc();
        let raw_instr = self.mem.read32(pc);

        let instr = Instruction::decode(raw_instr);

        let update_pc = self.execute(instr);

        if !update_pc {
            self.regs.set_pc(pc.wrapping_add(4));
        }

        self.cycle_count += 1;
    }

    pub fn execute(&mut self, instr: Instruction) -> bool {
        let mut update_pc = false;
        match instr {
            // R-type
            Instruction::R {
                opcode,
                rd,
                rs1,
                rs2,
            } => {
                let a = self.regs.get(rs1);
                let b = self.regs.get(rs2);

                let alu_op = ALUOp::from_opcode(opcode);
                let in_flags = Flags::from_u32(self.regs.flags());

                let result: ALUResult = ALU::execute(alu_op, a, b, in_flags);

                self.regs.set(rd, result.value);
                self.regs.set_flags(result.flags.to_u32());
            }

            // I-type
            Instruction::I {
                opcode,
                rd,
                rs1,
                imm,
            } => {
                let a = self.regs.get(rs1);

                let alu_op = ALUOp::from_opcode(opcode);
                let in_flags = Flags::from_u32(self.regs.flags());

                let result: ALUResult = ALU::execute(alu_op, a, imm, in_flags);

                self.regs.set(rd, result.value);
                self.regs.set_flags(result.flags.to_u32());
            }

            // J-type
            Instruction::J { opcode, offset } => {
                let flags = Flags::from_u32(self.regs.flags());
                let pc = self.regs.pc();
                let offset = sign_extend_24(offset as u32) as i32;

                let target = match opcode {
                    Opcode::JMP => {
                        update_pc = true;
                        pc.wrapping_add((offset * 4) as u32)
                    }
                    Opcode::JZ => {
                        if flags.zero {
                            update_pc = true;
                            pc.wrapping_add((offset * 4) as u32)
                        } else {
                            pc
                        }
                    }
                    Opcode::JNZ => {
                        if !flags.zero {
                            update_pc = true;
                            pc.wrapping_add((offset * 4) as u32)
                        } else {
                            pc
                        }
                    }
                    Opcode::JEQ => {
                        if flags.equal {
                            update_pc = true;
                            pc.wrapping_add((offset * 4) as u32)
                        } else {
                            pc
                        }
                    }
                    Opcode::JNE => {
                        if flags.not_equal {
                            update_pc = true;
                            pc.wrapping_add((offset * 4) as u32)
                        } else {
                            pc
                        }
                    }
                    Opcode::JLT => {
                        if flags.less {
                            update_pc = true;
                            pc.wrapping_add((offset * 4) as u32)
                        } else {
                            pc
                        }
                    }
                    Opcode::JGT => {
                        if flags.greater {
                            update_pc = true;
                            pc.wrapping_add((offset * 4) as u32)
                        } else {
                            pc
                        }
                    }
                    Opcode::JLE => {
                        if flags.less_equal {
                            update_pc = true;
                            pc.wrapping_add((offset * 4) as u32)
                        } else {
                            pc
                        }
                    }
                    Opcode::JGE => {
                        if flags.greater_equal {
                            update_pc = true;
                            pc.wrapping_add((offset * 4) as u32)
                        } else {
                            pc
                        }
                    }
                    Opcode::JC => {
                        if flags.carry {
                            update_pc = true;
                            pc.wrapping_add((offset * 4) as u32)
                        } else {
                            pc
                        }
                    }
                    Opcode::JO => {
                        if flags.overflow {
                            update_pc = true;
                            pc.wrapping_add((offset * 4) as u32)
                        } else {
                            pc
                        }
                    }
                    Opcode::CALL => {
                        update_pc = true;
                        let ret_addr = pc.wrapping_add(4);
                        let sp = self.regs.sp().wrapping_sub(4);
                        self.mem.write32(sp, ret_addr);
                        self.regs.set_sp(sp);
                        pc.wrapping_add((offset * 4) as u32)
                    }
                    Opcode::RET => {
                        update_pc = true;
                        let sp = self.regs.sp();
                        let ret_addr = self.mem.read32(sp);
                        self.regs.set_sp(sp.wrapping_add(4));
                        ret_addr
                    }

                    Opcode::HALT => {
                        self.halted = true;
                        pc
                    }
                    _ => unimplemented!(),
                };

                self.regs.set_pc(target);
            }

            Instruction::Mem {
                opcode,
                rd,
                rs1,
                imm,
            } => {
                let addr = self.regs.get(rs1).wrapping_add(imm);

                match opcode {
                    Opcode::LDB => {
                        let value = self.mem.read8(addr) as i8 as i32 as u32;
                        self.regs.set(rd, value);
                    }
                    Opcode::LDBU => {
                        let value = self.mem.read8(addr) as u32;
                        self.regs.set(rd, value);
                    }
                    Opcode::LDH => {
                        let value = self.mem.read16(addr) as i16 as i32 as u32;
                        self.regs.set(rd, value);
                    }
                    Opcode::LDHU => {
                        let value = self.mem.read16(addr) as u32;
                        self.regs.set(rd, value);
                    }
                    Opcode::LDW | Opcode::LDLR => {
                        let value = self.mem.read32(addr);
                        self.regs.set(rd, value);
                    }

                    // Store
                    Opcode::STB => self.mem.write8(addr, self.regs.get(rd) as u8),
                    Opcode::STH => self.mem.write16(addr, self.regs.get(rd) as u16),
                    Opcode::STW | Opcode::STLR => self.mem.write32(addr, self.regs.get(rd)),

                    Opcode::PUSH => {
                        let sp = self.regs.sp().wrapping_sub(4);
                        self.mem.write32(sp, self.regs.get(rd));
                        self.regs.set_sp(sp);
                    }

                    Opcode::POP => {
                        let sp = self.regs.sp();
                        let value = self.mem.read32(sp);
                        self.regs.set(rd, value);
                        self.regs.set_sp(sp.wrapping_add(4));
                    }

                    _ => unimplemented!(),
                }
            }

            // Move & System Instructions
            Instruction::Sys { opcode, rd, imm } => match opcode {
                Opcode::MOV => {
                    let value = self.regs.get(imm as u8);
                    self.regs.set(rd, value);
                }
                Opcode::LI => {
                    self.regs.set(rd, imm & 0xFFFF);
                }
                Opcode::LUI => {
                    self.regs.set(rd, imm << 16);
                }
                Opcode::MOVPC => {
                    self.regs.set(rd, self.regs.pc());
                }
                Opcode::MTSR => {
                    self.regs.set_flags(self.regs.get(rd));
                }
                Opcode::MFSR => {
                    self.regs.set(rd, self.regs.flags());
                }
                Opcode::MOVSP => {
                    self.regs.set(rd, self.regs.sp());
                }
                Opcode::SETSP => {
                    self.regs.set_sp(self.regs.get(rd));
                }
                _ => unimplemented!(),
            },

            Instruction::FP {
                opcode,
                rd,
                rs1,
                rs2,
            } => match opcode {
                Opcode::FADD => {
                    let a = self.regs.fget(rs1);
                    let b = self.regs.fget(rs2);
                    self.regs.fset(rd, a + b);
                }
                Opcode::FSUB => {
                    let a = self.regs.fget(rs1);
                    let b = self.regs.fget(rs2);
                    self.regs.fset(rd, a - b);
                }
                Opcode::FMUL => {
                    let a = self.regs.fget(rs1);
                    let b = self.regs.fget(rs2);
                    self.regs.fset(rd, a * b);
                }
                Opcode::FDIV => {
                    let a = self.regs.fget(rs1);
                    let b = self.regs.fget(rs2);
                    self.regs.fset(rd, a / b);
                }

                Opcode::FCMP => {
                    let a = self.regs.fget(rs1);
                    let b = self.regs.fget(rs2);
                    let mut flags = Flags::from_u32(self.regs.flags());
                    flags.zero = a == b;
                    flags.less = a < b;
                    flags.greater = a > b;
                    self.regs.set_flags(flags.to_u32());
                }

                Opcode::FEQ => self
                    .regs
                    .set(rd, (self.regs.fget(rs1) == self.regs.fget(rs2)) as u32),
                Opcode::FLT => self
                    .regs
                    .set(rd, (self.regs.fget(rs1) < self.regs.fget(rs2)) as u32),
                Opcode::FGT => self
                    .regs
                    .set(rd, (self.regs.fget(rs1) > self.regs.fget(rs2)) as u32),

                Opcode::FTOI => self.regs.set(rd, self.regs.fget(rs1) as u32),
                Opcode::ITOF => self.regs.fset(rd, self.regs.get(rs1) as f32),

                Opcode::FMOV => self.regs.fset(rd, self.regs.fget(rs1)),
                Opcode::FLD => {
                    let addr = self.regs.get(rs1);
                    let bits = self.mem.read32(addr);
                    self.regs.fset(rd, f32::from_bits(bits));
                }

                Opcode::FST => {
                    let addr = self.regs.get(rs1);
                    let bits = self.regs.fget(rd).to_bits();
                    self.mem.write32(addr, bits);
                }

                _ => unimplemented!(),
            },

            Instruction::IO { opcode, rd, port } => match opcode {
                Opcode::IN => {
                    let value = self.io.read(port);
                    self.regs.set(rd, value);
                }
                Opcode::OUT => {
                    let value = self.regs.get(rd);
                    self.io.write(port, value);
                }
                _ => unimplemented!(),
            },
        }
        update_pc
    }
}
