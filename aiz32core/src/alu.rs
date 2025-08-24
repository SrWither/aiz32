use crate::instruction::Opcode;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ALUOp {
    // Aritmética
    Nop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Inc,
    Dec,
    Neg,
    Abs,

    // Lógicos
    And,
    Or,
    Xor,
    Nand,
    Nor,
    Xnor,
    Not,

    // Desplazamientos/rotaciones
    Shl,
    Shr,
    Sar,
    Rol,
    Ror,

    // Extensiones / utilidades
    Sextb,
    Zextb,
    Sexth,
    Zexth,
    Popcnt,

    // Comparaciones
    Cmp,
    Ucmp,
    Setz,
    Setnz,
    Pass,
}

impl ALUOp {
    pub fn from_opcode(opcode: Opcode) -> Self {
        use ALUOp::*;
        match opcode {
            // R-type
            Opcode::NOP => Nop,
            Opcode::ADD => Add,
            Opcode::SUB => Sub,
            Opcode::MUL => Mul,
            Opcode::DIV => Div,
            Opcode::MOD => Mod,
            Opcode::INC => Inc,
            Opcode::DEC => Dec,
            Opcode::NEG => Neg,
            Opcode::ABS => Abs,
            Opcode::AND => And,
            Opcode::OR => Or,
            Opcode::XOR => Xor,
            Opcode::NAND => Nand,
            Opcode::NOR => Nor,
            Opcode::XNOR => Xnor,
            Opcode::NOT => Not,
            Opcode::SHL => Shl,
            Opcode::SHR => Shr,
            Opcode::SAR => Sar,
            Opcode::ROL => Rol,
            Opcode::ROR => Ror,
            Opcode::SEXTB => Sextb,
            Opcode::ZEXTB => Zextb,
            Opcode::SEXTH => Sexth,
            Opcode::ZEXTH => Zexth,
            Opcode::POPCNT => Popcnt,
            Opcode::CMP => Cmp,
            Opcode::UCMP => Ucmp,
            Opcode::SETZ => Setz,
            Opcode::SETNZ => Setnz,
            Opcode::PASS => Pass,

            // I-type
            Opcode::ADDI => Add,
            Opcode::SUBI => Sub,
            Opcode::MULI => Mul,
            Opcode::DIVI => Div,
            Opcode::MODI => Mod,
            Opcode::INCI => Inc,
            Opcode::DECI => Dec,
            Opcode::NEGI => Neg,
            Opcode::ABSI => Abs,
            Opcode::ANDI => And,
            Opcode::ORI => Or,
            Opcode::XORI => Xor,
            Opcode::NANDI => Nand,
            Opcode::NORI => Nor,
            Opcode::XNORI => Xnor,
            Opcode::NOTI => Not,
            Opcode::SHLI => Shl,
            Opcode::SHRI => Shr,
            Opcode::SARI => Sar,
            Opcode::ROLI => Rol,
            Opcode::RORI => Ror,
            Opcode::SEXTBI => Sextb,
            Opcode::ZEXTBI => Zextb,
            Opcode::SEXTHI => Sexth,
            Opcode::ZEXTHI => Zexth,
            Opcode::POPCNTI => Popcnt,
            Opcode::CMPI => Cmp,
            Opcode::UCMPI => Ucmp,
            Opcode::SETZI => Setz,
            Opcode::SETNZI => Setnz,
            Opcode::PASSI => Pass,

            _ => panic!("Opcode {:?} no es una operación ALU válida", opcode),
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Flags {
    pub zero: bool,
    pub carry: bool,
    pub overflow: bool,
    pub sign: bool,

    pub greater: bool,
    pub equal: bool,
    pub not_equal: bool,
    pub less: bool,
    pub greater_equal: bool,
    pub less_equal: bool,
}

impl Flags {
    pub fn from_u32(value: u32) -> Self {
        Self {
            zero: (value & 0x01) != 0,
            carry: (value & 0x02) != 0,
            overflow: (value & 0x04) != 0,
            sign: (value & 0x08) != 0,
            greater: (value & 0x10) != 0,
            equal: (value & 0x20) != 0,
            not_equal: (value & 0x40) != 0,
            less: (value & 0x80) != 0,
            greater_equal: (value & 0x100) != 0,
            less_equal: (value & 0x200) != 0,
        }
    }

    pub fn to_u32(&self) -> u32 {
        (if self.zero { 0x01 } else { 0 })
            | (if self.carry { 0x02 } else { 0 })
            | (if self.overflow { 0x04 } else { 0 })
            | (if self.sign { 0x08 } else { 0 })
            | (if self.greater { 0x10 } else { 0 })
            | (if self.equal { 0x20 } else { 0 })
            | (if self.not_equal { 0x40 } else { 0 })
            | (if self.less { 0x80 } else { 0 })
            | (if self.greater_equal { 0x100 } else { 0 })
            | (if self.less_equal { 0x200 } else { 0 })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ALUResult {
    pub value: u32,
    pub flags: Flags,
}

pub struct ALU;

impl ALU {
    pub fn new() -> Self {
        Self {}
    }

    #[inline]
    pub fn execute(op: ALUOp, a: u32, b: u32, in_flags: Flags) -> ALUResult {
        use ALUOp::*;
        let mut f = Flags { ..in_flags };
        let mut touch_rel = false;

        let (value, set_basic_flags): (u32, bool) = match op {
            Nop => (0, false),

            Add => {
                let (r, c) = a.overflowing_add(b);
                f.carry = c;
                f.overflow = overflow_add_i32(a, b, r);
                (r, true)
            }
            Sub => {
                let (r, c) = a.overflowing_sub(b);
                f.carry = c;
                f.overflow = overflow_sub_i32(a, b, r);
                (r, true)
            }
            Inc => {
                let (r, c) = a.overflowing_add(1);
                f.carry = c;
                f.overflow = overflow_add_i32(a, 1, r);
                (r, true)
            }
            Dec => {
                let (r, c) = a.overflowing_sub(1);
                f.carry = c;
                f.overflow = overflow_sub_i32(a, 1, r);
                (r, true)
            }
            Neg => {
                let r = (0u32).wrapping_sub(a);
                f.carry = a != 0;
                f.overflow = a == 0x8000_0000;
                (r, true)
            }
            Abs => {
                let ai = a as i32;
                let (r, ovf) = if ai == i32::MIN {
                    (ai, true)
                } else {
                    (ai.abs(), false)
                };
                f.carry = false;
                f.overflow = ovf;
                (r as u32, true)
            }
            Mul => {
                let a64 = a as i64;
                let b64 = b as i64;
                let p128 = (a64 as i128) * (b64 as i128);
                let r = (a64.wrapping_mul(b64)) as u32;
                let hi = (p128 >> 32) as i128;
                let sign_ext_ok = hi == ((r as i32 as i128) >> 32);
                f.overflow = !sign_ext_ok;
                f.carry = !sign_ext_ok;
                (r, true)
            }
            Div => {
                let ai = a as i32;
                let bi = b as i32;
                if bi == 0 {
                    f.overflow = true;
                    f.carry = false;
                    (0, true)
                } else if ai == i32::MIN && bi == -1 {
                    f.overflow = true;
                    f.carry = false;
                    (0, true)
                } else {
                    f.overflow = false;
                    f.carry = false;
                    ((ai / bi) as u32, true)
                }
            }
            Mod => {
                let ai = a as i32;
                let bi = b as i32;
                if bi == 0 {
                    f.overflow = true;
                    f.carry = false;
                    (0, true)
                } else if ai == i32::MIN && bi == -1 {
                    f.overflow = false;
                    f.carry = false;
                    (0, true)
                } else {
                    f.overflow = false;
                    f.carry = false;
                    ((ai % bi) as u32, true)
                }
            }

            And => (a & b, true),
            Or => (a | b, true),
            Xor => (a ^ b, true),
            Nand => (!(a & b), true),
            Nor => (!(a | b), true),
            Xnor => (!(a ^ b), true),
            Not => (!a, true),

            Shl => {
                let sh = (b & 31) as u32;
                f.carry = if sh == 0 {
                    false
                } else {
                    (a >> (32 - sh)) & 1 == 1
                };
                (a.wrapping_shl(sh), true)
            }
            Shr => {
                let sh = (b & 31) as u32;
                f.carry = if sh == 0 {
                    false
                } else {
                    (a >> (sh - 1)) & 1 == 1
                };
                (a.wrapping_shr(sh), true)
            }
            Sar => {
                let sh = (b & 31) as u32;
                let ai = a as i32;
                f.carry = if sh == 0 {
                    false
                } else {
                    ((a >> (sh - 1)) & 1) == 1
                };
                ((ai >> sh) as u32, true)
            }
            Rol => {
                let r = rol(a, b);
                f.carry = (r & 1) == 1;
                (r, true)
            }
            Ror => {
                let r = ror(a, b);
                f.carry = (r >> 31) & 1 == 1;
                (r, true)
            }

            Sextb => {
                let byte = (a & 0xFF) as u8 as i8 as i32 as u32;
                f.carry = false;
                f.overflow = false;
                (byte, true)
            }
            Zextb => {
                let byte = a & 0xFF;
                f.carry = false;
                f.overflow = false;
                (byte, true)
            }
            Sexth => {
                let half = (a & 0xFFFF) as u16 as i16 as i32 as u32;
                f.carry = false;
                f.overflow = false;
                (half, true)
            }
            Zexth => {
                let half = a & 0xFFFF;
                f.carry = false;
                f.overflow = false;
                (half, true)
            }
            Popcnt => {
                let c = a.count_ones();
                f.carry = false;
                f.overflow = false;
                (c, true)
            }

            Cmp => {
                touch_rel = true;
                let ai = a as i32;
                let bi = b as i32;
                f.equal = ai == bi;
                f.not_equal = ai != bi;
                f.less = ai < bi;
                f.greater = ai > bi;
                f.less_equal = ai <= bi;
                f.greater_equal = ai >= bi;

                let (r, c) = a.overflowing_sub(b);
                f.carry = c;
                f.overflow = overflow_sub_i32(a, b, r);
                (0, false)
            }
            Ucmp => {
                touch_rel = true;
                f.equal = a == b;
                f.not_equal = a != b;
                f.less = a < b;
                f.greater = a > b;
                f.less_equal = a <= b;
                f.greater_equal = a >= b;

                let (_r, c) = a.overflowing_sub(b);
                f.carry = c;
                f.overflow = false;
                (0, false)
            }
            Setz => {
                let v = if in_flags.zero { 1 } else { 0 };

                (v, true)
            }
            Setnz => {
                let v = if in_flags.zero { 0 } else { 1 };
                (v, true)
            }
            Pass => (a, true),
        };

        if set_basic_flags {
            f.zero = value == 0;
            f.sign = (value as i32) < 0;
        }

        if !touch_rel && !matches!(op, Cmp | Ucmp) {
            // f.greater = f.equal = f.not_equal = f.less = f.greater_equal = f.less_equal = false;
        }

        ALUResult { value, flags: f }
    }
}

#[inline]
fn overflow_add_i32(a: u32, b: u32, r: u32) -> bool {
    let (ai, bi, ri) = (a as i32, b as i32, r as i32);
    (ai ^ ri).is_negative() && (ai ^ bi).is_positive()
}

#[inline]
fn overflow_sub_i32(a: u32, b: u32, r: u32) -> bool {
    let (ai, bi, ri) = (a as i32, b as i32, r as i32);
    (ai ^ bi).is_negative() && (ai ^ ri).is_negative()
}

#[inline]
fn rol(x: u32, n: u32) -> u32 {
    let s = (n & 31) as u32;
    if s == 0 { x } else { x.rotate_left(s) }
}

#[inline]
fn ror(x: u32, n: u32) -> u32 {
    let s = (n & 31) as u32;
    if s == 0 { x } else { x.rotate_right(s) }
}
