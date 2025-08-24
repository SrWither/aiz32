use aiz32core::instruction::Opcode;
use std::collections::HashMap;

use crate::utils::{parse_imm, parse_port, parse_reg};
use crate::{AssembleError, encode::*};

pub struct Assembly {
    pub labels: HashMap<String, u32>,
    pub lines: Vec<(u32, Vec<String>)>,
}

pub fn tokenize_line(line: &str) -> Option<Vec<String>> {
    let line = line.split(';').next().unwrap_or("");
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    let clean = line.replace(",", " ").replace("[", " ").replace("]", " ");

    let tokens: Vec<String> = clean.split_whitespace().map(|s| s.to_uppercase()).collect();

    if tokens.is_empty() {
        None
    } else {
        Some(tokens)
    }
}

pub fn first_pass(lines: &[String]) -> Assembly {
    let mut labels = HashMap::new();
    let mut parsed_lines = Vec::new();
    let mut pc: u32 = 0;

    for line in lines {
        if let Some(mut tokens) = tokenize_line(line) {
            if tokens[0].ends_with(":") {
                let label = tokens[0].trim_end_matches(":").to_string();
                labels.insert(label, pc);
                tokens.remove(0);
                if tokens.is_empty() {
                    continue;
                }
            }

            parsed_lines.push((pc, tokens));
            pc += 1;
        }
    }

    Assembly {
        labels,
        lines: parsed_lines,
    }
}

pub fn second_pass(
    asm: Assembly,
    table: &HashMap<String, Opcode>,
) -> Result<Vec<u32>, AssembleError> {
    let mut result = Vec::new();

    for (current_pc, tokens) in asm.lines {
        let opcode_str = &tokens[0];
        let opcode = table
            .get(opcode_str)
            .unwrap_or_else(|| panic!("Unknown opcode: {}", opcode_str));

        let encoded = match std::panic::catch_unwind(|| match opcode {
            Opcode::ADD
            | Opcode::SUB
            | Opcode::MUL
            | Opcode::DIV
            | Opcode::MOD
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
            | Opcode::SETZ
            | Opcode::SETNZ
            | Opcode::PASS
            | Opcode::SEXTH
            | Opcode::ZEXTH => {
                let rd = parse_reg(&tokens[1]);
                let rs1 = parse_reg(&tokens[2]);
                let rs2 = parse_reg(&tokens[3]);
                encode_r(*opcode, rd, rs1, rs2)
            }
            Opcode::INC | Opcode::DEC | Opcode::NEG | Opcode::ABS => {
                let rd = parse_reg(&tokens[1]);
                let rs1 = parse_reg(&tokens[2]);
                encode_r(*opcode, rd, rs1, 0)
            }
            Opcode::CMP | Opcode::UCMP => {
                let rd = parse_reg(&tokens[1]);
                let rs1 = parse_reg(&tokens[2]);
                encode_r(*opcode, rd, rs1, 0)
            }

            Opcode::NOP => encode_r(*opcode, 0, 0, 0),

            Opcode::ADDI
            | Opcode::SUBI
            | Opcode::MULI
            | Opcode::DIVI
            | Opcode::MODI
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
            | Opcode::SETZI
            | Opcode::SETNZI
            | Opcode::PASSI
            | Opcode::SEXTHI
            | Opcode::ZEXTHI => {
                let rd = parse_reg(&tokens[1]);
                let rs1 = parse_reg(&tokens[2]);
                let imm = parse_imm(&tokens[3], 14);
                encode_i(*opcode, rd, rs1, imm)
            }
            Opcode::CMPI | Opcode::UCMPI => {
                let rd = parse_reg(&tokens[1]);
                let imm = parse_imm(&tokens[2], 14);
                encode_i(*opcode, rd, 0, imm)
            }

            Opcode::INCI | Opcode::DECI | Opcode::NEGI | Opcode::ABSI => {
                let rd = parse_reg(&tokens[1]);
                let imm = parse_imm(&tokens[2], 14);
                encode_i(*opcode, rd, 0, imm)
            }

            Opcode::NOPI => encode_i(*opcode, 0, 0, 0),

            Opcode::LDB
            | Opcode::LDBU
            | Opcode::LDH
            | Opcode::LDHU
            | Opcode::LDW
            | Opcode::STB
            | Opcode::STH
            | Opcode::STW => {
                let rd = parse_reg(&tokens[1]);
                let rs1 = parse_reg(&tokens[2]);
                let imm = parse_imm(&tokens[3], 14);
                encode_mem(*opcode, rd, rs1, imm)
            }

            Opcode::LDLR | Opcode::STLR => {
                let rd = parse_reg(&tokens[1]);
                let rs1 = parse_reg(&tokens[2]);
                encode_mem(*opcode, rd, rs1, 0)
            }

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
                let label = &tokens[1];
                let target_address = *asm
                    .labels
                    .get(label)
                    .unwrap_or_else(|| panic!("Unknown label: {}", label));
                let offset = (target_address as i32).wrapping_sub(current_pc as i32) as u32;
                encode_j(*opcode, offset)
            }

            Opcode::RET | Opcode::HALT => encode_j(*opcode, 0),

            Opcode::MOV
            | Opcode::MOVPC
            | Opcode::MTSR
            | Opcode::MFSR
            | Opcode::MOVSP
            | Opcode::SETSP => {
                let rd = parse_reg(&tokens[1]);
                let rs1 = if tokens.len() > 2 {
                    parse_reg(&tokens[2])
                } else {
                    0
                };
                encode_r(*opcode, rd, rs1, 0)
            }

            Opcode::LI | Opcode::LUI => {
                let rd = parse_reg(&tokens[1]);
                let imm = parse_imm(&tokens[2], 19);
                encode_sys(*opcode, rd, imm)
            }

            Opcode::IN | Opcode::OUT => {
                let rd = parse_reg(&tokens[1]);
                let port = parse_port(&tokens[2]);
                encode_io(*opcode, port, rd)
            }

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
                let rd = parse_reg(&tokens[1]);
                let rs1 = parse_reg(&tokens[2]);
                let rs2 = parse_reg(&tokens[3]);
                encode_fp(*opcode, rd, rs1, rs2)
            }

            Opcode::FLD | Opcode::FST => {
                let rd = parse_reg(&tokens[1]);
                let rs1 = parse_reg(&tokens[2]);
                let imm = parse_imm(&tokens[3], 14);
                encode_mem(*opcode, rd, rs1, imm)
            }
        }) {
            Ok(enc) => enc,
            Err(_) => {
                return Err(AssembleError {
                    line_number: current_pc as usize,
                    line_content: tokens.join(" "),
                    message: "Pánico al ensamblar la instrucción".to_string(),
                });
            }
        };

        result.push(encoded);
    }

    Ok(result)
}
