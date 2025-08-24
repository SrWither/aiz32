use aiz32core::instruction::Opcode;

/// R-type: opcode(8) | rd(5) | rs1(5) | rs2(5) | unused(9)
pub fn encode_r(opcode: Opcode, rd: u8, rs1: u8, rs2: u8) -> u32 {
    ((opcode as u32) << 24) | ((rd as u32) << 19) | ((rs1 as u32) << 14) | ((rs2 as u32) << 9)
}

/// I-type: opcode(8) | rd(5) | rs1(5) | imm(14)
pub fn encode_i(opcode: Opcode, rd: u8, rs1: u8, imm: u32) -> u32 {
    ((opcode as u32) << 24) | ((rd as u32) << 19) | ((rs1 as u32) << 14) | (imm & 0x3FFF)
}

/// J-type: opcode(8) | addr(24)
pub fn encode_j(opcode: Opcode, offset: u32) -> u32 {
    ((opcode as u32) << 24) | (offset & 0xFFFFFF)
}

/// Mem-type: opcode(8) | rd(5) | rs1(5) | imm(14)
pub fn encode_mem(opcode: Opcode, rd: u8, rs1: u8, imm: u32) -> u32 {
    ((opcode as u32) << 24) | ((rd as u32) << 19) | ((rs1 as u32) << 14) | (imm & 0x3FFF)
}

/// Sys-type: opcode(8) | rd(5) | imm(19)
pub fn encode_sys(opcode: Opcode, rd: u8, imm: u32) -> u32 {
    ((opcode as u32) << 24) | ((rd as u32) << 19) | (imm & 0x7FFFF)
}

/// FP-type: opcode(8) | rd(5) | rs1(5) | rs2(5) | unused(9)
pub fn encode_fp(opcode: Opcode, rd: u8, rs1: u8, rs2: u8) -> u32 {
    ((opcode as u32) << 24) | ((rd as u32) << 19) | ((rs1 as u32) << 14) | ((rs2 as u32) << 9)
}

/// IO-type: opcode(8) | port(16) | rd(5) | unused(3)
pub fn encode_io(opcode: Opcode, port: u16, rd: u8) -> u32 {
    ((opcode as u32) << 24) | ((port as u32) << 8) | ((rd as u32) << 3)
}
