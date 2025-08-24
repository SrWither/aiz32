mod encode;
pub mod opcode;
mod parser;
mod tests;
mod utils;

use aiz32core::instruction::Opcode;
use parser::*;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
pub struct AssembleError {
    pub line_number: usize,
    pub line_content: String,
    pub message: String,
}

pub fn assemble_lines(lines: Vec<String>, table: &HashMap<String, Opcode>) -> Vec<u32> {
    let asm = first_pass(&lines);
    second_pass(asm, table).map_err(|err| {
        let line_number = err.line_number;
        let line_content = lines.get(line_number).cloned().unwrap_or_default();
        AssembleError {
            line_number,
            line_content,
            message: format!("Error al ensamblar: {}", err.message),
        }
    }).unwrap()
}

pub fn assemble_from_vec(lines: Vec<String>, table: &HashMap<String, Opcode>) -> Vec<u32> {
    let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
    assemble_lines(lines, table)
}

pub fn assemble_file(input: &str, output: &str, table: &HashMap<String, Opcode>) {
    let content = fs::read_to_string(input).expect("No se pudo leer el archivo .asm");
    let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    let encoded = assemble_lines(lines, table);

    let mut bytes = Vec::new();
    for inst in encoded {
        bytes.extend_from_slice(&inst.to_le_bytes());
    }

    fs::write(output, bytes).expect("No se pudo escribir el archivo binario");
}

pub fn assemble_to_formats(
    lines: Vec<String>,
    table: &HashMap<String, Opcode>,
) -> (Vec<String>, Vec<String>) {
    let encoded = assemble_lines(lines, table);

    let rawhex: Vec<String> = encoded
        .iter()
        .map(|&inst| {
            let le_bytes = inst.to_le_bytes();

            le_bytes
                .iter()
                .rev()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join("")
        })
        .collect();

    let rawbin: Vec<String> = encoded
        .iter()
        .map(|&inst| {
            let le_bytes = inst.to_le_bytes();
            le_bytes
                .iter()
                .map(|b| format!("{:08b}", b))
                .collect::<Vec<_>>()
                .join("")
        })
        .collect();

    (rawhex, rawbin)
}
