#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // ALU (R-type)
    NOP = 0x00,
    ADD = 0x01,
    SUB = 0x02,
    MUL = 0x03,
    DIV = 0x04,
    MOD = 0x05,
    INC = 0x06,
    DEC = 0x07,
    NEG = 0x08,
    ABS = 0x09,
    AND = 0x0A,
    OR = 0x0B,
    XOR = 0x0C,
    NAND = 0x0D,
    NOR = 0x0E,
    XNOR = 0x0F,
    NOT = 0x10,
    SHL = 0x11,
    SHR = 0x12,
    SAR = 0x13,
    ROL = 0x14,
    ROR = 0x15,
    SEXTB = 0x16,
    ZEXTB = 0x17,
    POPCNT = 0x18,
    CMP = 0x19,
    UCMP = 0x1A,
    SETZ = 0x1B,
    SETNZ = 0x1C,
    PASS = 0x1D,
    SEXTH = 0x1E,
    ZEXTH = 0x1F,

    // ALU (I-type)
    NOPI = 0x20,
    ADDI = 0x21,
    SUBI = 0x22,
    MULI = 0x23,
    DIVI = 0x24,
    MODI = 0x25,
    INCI = 0x26,
    DECI = 0x27,
    NEGI = 0x28,
    ABSI = 0x29,
    ANDI = 0x2A,
    ORI = 0x2B,
    XORI = 0x2C,
    NANDI = 0x2D,
    NORI = 0x2E,
    XNORI = 0x2F,
    NOTI = 0x30,
    SHLI = 0x31,
    SHRI = 0x32,
    SARI = 0x33,
    ROLI = 0x34,
    RORI = 0x35,
    SEXTBI = 0x36,
    ZEXTBI = 0x37,
    POPCNTI = 0x38,
    CMPI = 0x39,
    UCMPI = 0x3A,
    SETZI = 0x3B,
    SETNZI = 0x3C,
    PASSI = 0x3D,
    SEXTHI = 0x3E,
    ZEXTHI = 0x3F,

    // Memory
    LDB = 0x40,
    LDBU = 0x41,
    LDH = 0x42,
    LDHU = 0x43,
    LDW = 0x44,
    STB = 0x45,
    STH = 0x46,
    STW = 0x47,
    LDLR = 0x48,
    STLR = 0x49,

    // Jumps & Branch
    JMP = 0x60,
    JZ = 0x61,
    JNZ = 0x62,
    JEQ = 0x63,
    JNE = 0x64,
    JLT = 0x65,
    JGT = 0x66,
    JLE = 0x67,
    JGE = 0x68,
    JC = 0x69,
    JO = 0x6A,
    CALL = 0x6B,
    RET = 0x6C,
    HALT = 0x6D, // Note: HALT was missing in the ISA table; added here

    // Move & System
    MOV = 0x80,
    LI = 0x81,
    LUI = 0x82,
    MOVPC = 0x83,
    MTSR = 0x84,
    MFSR = 0x85,
    MOVSP = 0x86,
    SETSP = 0x87,

    // Floating Point
    FADD = 0xA0,
    FSUB = 0xA1,
    FMUL = 0xA2,
    FDIV = 0xA3,
    FCMP = 0xA4,
    FEQ = 0xA5,
    FLT = 0xA6,
    FGT = 0xA7,
    FTOI = 0xA8,
    ITOF = 0xA9,
    FMOV = 0xAA,
    FLD = 0xAB,
    FST = 0xAC,

    // IO
    IN = 0xC0,
    OUT = 0xC1,
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    R {
        opcode: Opcode,
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    I {
        opcode: Opcode,
        rd: u8,
        rs1: u8,
        imm: u32,
    },
    J {
        opcode: Opcode,
        offset: u32,
    },
    Mem {
        opcode: Opcode,
        rd: u8,
        rs1: u8,
        imm: u32,
    },
    Sys {
        opcode: Opcode,
        rd: u8,
        imm: u32,
    },
    FP {
        opcode: Opcode,
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    IO {
        opcode: Opcode,
        port: u16,
        rd: u8,
    },
}

impl Instruction {
    pub fn decode(raw: u32) -> Self {
        let opcode_val = ((raw >> 24) & 0xFF) as u8;

        let opcode = match opcode_val {
            // R-type opcodes
            0x00 => Opcode::NOP,
            0x01 => Opcode::ADD,
            0x02 => Opcode::SUB,
            0x03 => Opcode::MUL,
            0x04 => Opcode::DIV,
            0x05 => Opcode::MOD,
            0x06 => Opcode::INC,
            0x07 => Opcode::DEC,
            0x08 => Opcode::NEG,
            0x09 => Opcode::ABS,
            0x0A => Opcode::AND,
            0x0B => Opcode::OR,
            0x0C => Opcode::XOR,
            0x0D => Opcode::NAND,
            0x0E => Opcode::NOR,
            0x0F => Opcode::XNOR,
            0x10 => Opcode::NOT,
            0x11 => Opcode::SHL,
            0x12 => Opcode::SHR,
            0x13 => Opcode::SAR,
            0x14 => Opcode::ROL,
            0x15 => Opcode::ROR,
            0x16 => Opcode::SEXTB,
            0x17 => Opcode::ZEXTB,
            0x18 => Opcode::POPCNT,
            0x19 => Opcode::CMP,
            0x1A => Opcode::UCMP,
            0x1B => Opcode::SETZ,
            0x1C => Opcode::SETNZ,
            0x1D => Opcode::PASS,
            0x1E => Opcode::SEXTH,
            0x1F => Opcode::ZEXTH,

            // I-type opcodes
            0x20 => Opcode::NOPI,
            0x21 => Opcode::ADDI,
            0x22 => Opcode::SUBI,
            0x23 => Opcode::MULI,
            0x24 => Opcode::DIVI,
            0x25 => Opcode::MODI,
            0x26 => Opcode::INCI,
            0x27 => Opcode::DECI,
            0x28 => Opcode::NEGI,
            0x29 => Opcode::ABSI,
            0x2A => Opcode::ANDI,
            0x2B => Opcode::ORI,
            0x2C => Opcode::XORI,
            0x2D => Opcode::NANDI,
            0x2E => Opcode::NORI,
            0x2F => Opcode::XNORI,
            0x30 => Opcode::NOTI,
            0x31 => Opcode::SHLI,
            0x32 => Opcode::SHRI,
            0x33 => Opcode::SARI,
            0x34 => Opcode::ROLI,
            0x35 => Opcode::RORI,
            0x36 => Opcode::SEXTBI,
            0x37 => Opcode::ZEXTBI,
            0x38 => Opcode::POPCNTI,
            0x39 => Opcode::CMPI,
            0x3A => Opcode::UCMPI,
            0x3B => Opcode::SETZI,
            0x3C => Opcode::SETNZI,
            0x3D => Opcode::PASSI,
            0x3E => Opcode::SEXTHI,
            0x3F => Opcode::ZEXTHI,

            // Memory opcodes
            0x40 => Opcode::LDB,
            0x41 => Opcode::LDBU,
            0x42 => Opcode::LDH,
            0x43 => Opcode::LDHU,
            0x44 => Opcode::LDW,
            0x45 => Opcode::STB,
            0x46 => Opcode::STH,
            0x47 => Opcode::STW,
            0x48 => Opcode::LDLR,
            0x49 => Opcode::STLR,

            // Jump & Branch opcodes
            0x60 => Opcode::JMP,
            0x61 => Opcode::JZ,
            0x62 => Opcode::JNZ,
            0x63 => Opcode::JEQ,
            0x64 => Opcode::JNE,
            0x65 => Opcode::JLT,
            0x66 => Opcode::JGT,
            0x67 => Opcode::JLE,
            0x68 => Opcode::JGE,
            0x69 => Opcode::JC,
            0x6A => Opcode::JO,
            0x6B => Opcode::CALL,
            0x6C => Opcode::RET,
            0x6D => Opcode::HALT,

            // Move & System opcodes
            0x80 => Opcode::MOV,
            0x81 => Opcode::LI,
            0x82 => Opcode::LUI,
            0x83 => Opcode::MOVPC,
            0x84 => Opcode::MTSR,
            0x85 => Opcode::MFSR,
            0x86 => Opcode::MOVSP,
            0x87 => Opcode::SETSP,

            // Floating Point opcodes
            0xA0 => Opcode::FADD,
            0xA1 => Opcode::FSUB,
            0xA2 => Opcode::FMUL,
            0xA3 => Opcode::FDIV,
            0xA4 => Opcode::FCMP,
            0xA5 => Opcode::FEQ,
            0xA6 => Opcode::FLT,
            0xA7 => Opcode::FGT,
            0xA8 => Opcode::FTOI,
            0xA9 => Opcode::ITOF,
            0xAA => Opcode::FMOV,
            0xAB => Opcode::FLD,
            0xAC => Opcode::FST,

            // IO opcodes
            0xC0 => Opcode::IN,
            0xC1 => Opcode::OUT,
            _ => panic!("Unknown opcode {:#04X}", opcode_val),
        };

        match opcode {
            // R-type
            Opcode::NOP
            | Opcode::ADD
            | Opcode::SUB
            | Opcode::MUL
            | Opcode::DIV
            | Opcode::MOD
            | Opcode::INC
            | Opcode::DEC
            | Opcode::NEG
            | Opcode::ABS
            | Opcode::AND
            | Opcode::OR
            | Opcode::XOR
            | Opcode::NAND
            | Opcode::NOR
            | Opcode::XNOR
            | Opcode::NOT
            | Opcode::SHL
            | Opcode::SHR
            | Opcode::SAR
            | Opcode::ROL
            | Opcode::ROR
            | Opcode::SEXTB
            | Opcode::ZEXTB
            | Opcode::POPCNT
            | Opcode::CMP
            | Opcode::UCMP
            | Opcode::SETZ
            | Opcode::SETNZ
            | Opcode::PASS
            | Opcode::SEXTH
            | Opcode::ZEXTH => {
                let rd = ((raw >> 19) & 0x1F) as u8;
                let rs1 = ((raw >> 14) & 0x1F) as u8;
                let rs2 = ((raw >> 9) & 0x1F) as u8;
                Instruction::R {
                    opcode,
                    rd,
                    rs1,
                    rs2,
                }
            }
            // I-type
            Opcode::NOPI
            | Opcode::ADDI
            | Opcode::SUBI
            | Opcode::MULI
            | Opcode::DIVI
            | Opcode::MODI
            | Opcode::INCI
            | Opcode::DECI
            | Opcode::NEGI
            | Opcode::ABSI
            | Opcode::ANDI
            | Opcode::ORI
            | Opcode::XORI
            | Opcode::NANDI
            | Opcode::NORI
            | Opcode::XNORI
            | Opcode::NOTI
            | Opcode::SHLI
            | Opcode::SHRI
            | Opcode::SARI
            | Opcode::ROLI
            | Opcode::RORI
            | Opcode::SEXTBI
            | Opcode::ZEXTBI
            | Opcode::POPCNTI
            | Opcode::CMPI
            | Opcode::UCMPI
            | Opcode::SETZI
            | Opcode::SETNZI
            | Opcode::PASSI
            | Opcode::SEXTHI
            | Opcode::ZEXTHI => {
                let rd = ((raw >> 19) & 0x1F) as u8;
                let rs1 = ((raw >> 14) & 0x1F) as u8;
                let imm = (raw & 0x1FF) as u32;
                Instruction::I {
                    opcode,
                    rd,
                    rs1,
                    imm,
                }
            }

            // Memory
            Opcode::LDB
            | Opcode::LDBU
            | Opcode::LDH
            | Opcode::LDHU
            | Opcode::LDW
            | Opcode::STB
            | Opcode::STH
            | Opcode::STW
            | Opcode::LDLR
            | Opcode::STLR => {
                let rd = ((raw >> 19) & 0x1F) as u8;
                let rs1 = ((raw >> 14) & 0x1F) as u8;
                let imm = (raw & 0x1FF) as u32;
                Instruction::Mem {
                    opcode,
                    rd,
                    rs1,
                    imm,
                }
            }

            // Jumps & Branch
            Opcode::JMP
            | Opcode::JZ
            | Opcode::JNZ
            | Opcode::JEQ
            | Opcode::JNE
            | Opcode::JLT
            | Opcode::JGT
            | Opcode::JLE
            | Opcode::JGE
            | Opcode::JC
            | Opcode::JO
            | Opcode::CALL => {
                let addr = raw & 0xFFFFFF;
                Instruction::J {
                    opcode,
                    offset: addr,
                }
            }
            Opcode::RET | Opcode::HALT => Instruction::J { opcode, offset: 0 },

            // Move & System
            Opcode::MOV
            | Opcode::LI
            | Opcode::LUI
            | Opcode::MOVPC
            | Opcode::MTSR
            | Opcode::MFSR
            | Opcode::MOVSP
            | Opcode::SETSP => {
                let rd = ((raw >> 19) & 0x1F) as u8;
                let imm = (raw & 0x1FF) as u32;
                Instruction::Sys { opcode, rd, imm }
            }

            Opcode::FLD | Opcode::FST => {
                let rd = ((raw >> 19) & 0x1F) as u8;
                let rs1 = ((raw >> 14) & 0x1F) as u8;
                let imm = (raw & 0x1FF) as u32;
                Instruction::Mem {
                    opcode,
                    rd,
                    rs1,
                    imm,
                }
            }

            // Floating Point
            Opcode::FADD
            | Opcode::FSUB
            | Opcode::FMUL
            | Opcode::FDIV
            | Opcode::FCMP
            | Opcode::FEQ
            | Opcode::FLT
            | Opcode::FGT
            | Opcode::FTOI
            | Opcode::ITOF
            | Opcode::FMOV => {
                let rd = ((raw >> 19) & 0x1F) as u8;
                let rs1 = ((raw >> 14) & 0x1F) as u8;
                let rs2 = ((raw >> 9) & 0x1F) as u8;
                Instruction::FP {
                    opcode,
                    rd,
                    rs1,
                    rs2,
                }
            }

            // IO
            Opcode::IN | Opcode::OUT => {
                let rd = ((raw >> 19) & 0x1F) as u8;
                let port = (raw & 0xFFFF) as u16;
                Instruction::IO { opcode, port, rd }
            }
        }
    }
}
