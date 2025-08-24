#[cfg(test)]
mod tests {
    use crate::assemble_from_vec;
    use aiz32core::instruction::Opcode;
    use std::collections::HashMap;

    fn opcode_table() -> HashMap<String, Opcode> {
        use Opcode::*;
        let mut table = HashMap::new();
        // ALU (R-type)
        table.insert("NOP".into(), NOP);
        table.insert("ADD".into(), ADD);
        table.insert("SUB".into(), SUB);
        table.insert("MUL".into(), MUL);
        table.insert("DIV".into(), DIV);
        table.insert("MOD".into(), MOD);
        table.insert("INC".into(), INC);
        table.insert("DEC".into(), DEC);
        table.insert("NEG".into(), NEG);
        table.insert("ABS".into(), ABS);
        table.insert("AND".into(), AND);
        table.insert("OR".into(), OR);
        table.insert("XOR".into(), XOR);
        table.insert("NAND".into(), NAND);
        table.insert("NOR".into(), NOR);
        table.insert("XNOR".into(), XNOR);
        table.insert("NOT".into(), NOT);
        table.insert("SHL".into(), SHL);
        table.insert("SHR".into(), SHR);
        table.insert("SAR".into(), SAR);
        table.insert("ROL".into(), ROL);
        table.insert("ROR".into(), ROR);
        table.insert("SEXTB".into(), SEXTB);
        table.insert("ZEXTB".into(), ZEXTB);
        table.insert("POPCNT".into(), POPCNT);
        table.insert("CMP".into(), CMP);
        table.insert("UCMP".into(), UCMP);
        table.insert("SETZ".into(), SETZ);
        table.insert("SETNZ".into(), SETNZ);
        table.insert("PASS".into(), PASS);
        table.insert("SEXTH".into(), SEXTH);
        table.insert("ZEXTH".into(), ZEXTH);
        // ALU (I-type)
        table.insert("NOPI".into(), NOPI);
        table.insert("ADDI".into(), ADDI);
        table.insert("SUBI".into(), SUBI);
        table.insert("MULI".into(), MULI);
        table.insert("DIVI".into(), DIVI);
        table.insert("MODI".into(), MODI);
        table.insert("INCI".into(), INCI);
        table.insert("DECI".into(), DECI);
        table.insert("NEGI".into(), NEGI);
        table.insert("ABSI".into(), ABSI);
        table.insert("ANDI".into(), ANDI);
        table.insert("ORI".into(), ORI);
        table.insert("XORI".into(), XORI);
        table.insert("NANDI".into(), NANDI);
        table.insert("NORI".into(), NORI);
        table.insert("XNORI".into(), XNORI);
        table.insert("NOTI".into(), NOTI);
        table.insert("SHLI".into(), SHLI);
        table.insert("SHRI".into(), SHRI);
        table.insert("SARI".into(), SARI);
        table.insert("ROLI".into(), ROLI);
        table.insert("RORI".into(), RORI);
        table.insert("SEXTBI".into(), SEXTBI);
        table.insert("ZEXTBI".into(), ZEXTBI);
        table.insert("POPCNTI".into(), POPCNTI);
        table.insert("CMPI".into(), CMPI);
        table.insert("UCMPI".into(), UCMPI);
        table.insert("SETZI".into(), SETZI);
        table.insert("SETNZI".into(), SETNZI);
        table.insert("PASSI".into(), PASSI);
        table.insert("SEXTHI".into(), SEXTHI);
        table.insert("ZEXTHI".into(), ZEXTHI);
        // Memory
        table.insert("LDB".into(), LDB);
        table.insert("LDBU".into(), LDBU);
        table.insert("LDH".into(), LDH);
        table.insert("LDHU".into(), LDHU);
        table.insert("LDW".into(), LDW);
        table.insert("STB".into(), STB);
        table.insert("STH".into(), STH);
        table.insert("STW".into(), STW);
        table.insert("LDLR".into(), LDLR);
        table.insert("STLR".into(), STLR);
        // Jumps & Branch
        table.insert("JMP".into(), JMP);
        table.insert("JZ".into(), JZ);
        table.insert("JNZ".into(), JNZ);
        table.insert("JEQ".into(), JEQ);
        table.insert("JNE".into(), JNE);
        table.insert("JLT".into(), JLT);
        table.insert("JGT".into(), JGT);
        table.insert("JLE".into(), JLE);
        table.insert("JGE".into(), JGE);
        table.insert("JC".into(), JC);
        table.insert("JO".into(), JO);
        table.insert("CALL".into(), CALL);
        table.insert("RET".into(), RET);
        table.insert("HALT".into(), HALT);
        // Move & System
        table.insert("MOV".into(), MOV);
        table.insert("LI".into(), LI);
        table.insert("LUI".into(), LUI);
        table.insert("MOVPC".into(), MOVPC);
        table.insert("MTSR".into(), MTSR);
        table.insert("MFSR".into(), MFSR);
        table.insert("MOVSP".into(), MOVSP);
        table.insert("SETSP".into(), SETSP);
        // Floating Point
        table.insert("FADD".into(), FADD);
        table.insert("FSUB".into(), FSUB);
        table.insert("FMUL".into(), FMUL);
        table.insert("FDIV".into(), FDIV);
        table.insert("FCMP".into(), FCMP);
        table.insert("FEQ".into(), FEQ);
        table.insert("FLT".into(), FLT);
        table.insert("FGT".into(), FGT);
        table.insert("FTOI".into(), FTOI);
        table.insert("ITOF".into(), ITOF);
        table.insert("FMOV".into(), FMOV);
        table.insert("FLD".into(), FLD);
        table.insert("FST".into(), FST);
        // IO
        table.insert("IN".into(), IN);
        table.insert("OUT".into(), OUT);
        table
    }

    fn run(lines: Vec<&str>) -> Vec<u32> {
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        assemble_from_vec(lines, &opcode_table())
    }

    #[test]
    fn test_r_type() {
        let out = run(vec!["ADD r1, r2, r3", "SUB r4, r5, r6"]);
        let add = out[0];
        let sub = out[1];
        assert_eq!(add >> 24, Opcode::ADD as u32);
        assert_eq!((add >> 19) & 0x1F, 1); // rd = r1
        assert_eq!((add >> 14) & 0x1F, 2); // rs1 = r2
        assert_eq!((add >> 9) & 0x1F, 3); // rs2 = r3
        assert_eq!(sub >> 24, Opcode::SUB as u32);
        assert_eq!((sub >> 19) & 0x1F, 4); // rd = r4
        assert_eq!((sub >> 14) & 0x1F, 5); // rs1 = r5
        assert_eq!((sub >> 9) & 0x1F, 6); // rs2 = r6
    }

    #[test]
    fn test_r_type_unary() {
        let out = run(vec!["INC r1, r2", "DEC r3, r4"]);
        let inc = out[0];
        let dec = out[1];
        assert_eq!(inc >> 24, Opcode::INC as u32);
        assert_eq!((inc >> 19) & 0x1F, 1); // rd = r1
        assert_eq!((inc >> 14) & 0x1F, 2); // rs1 = r2
        assert_eq!((inc >> 9) & 0x1F, 0); // rs2 = 0
        assert_eq!(dec >> 24, Opcode::DEC as u32);
        assert_eq!((dec >> 19) & 0x1F, 3); // rd = r3
        assert_eq!((dec >> 14) & 0x1F, 4); // rs1 = r4
        assert_eq!((dec >> 9) & 0x1F, 0); // rs2 = 0
    }

    #[test]
    fn test_nop() {
        let out = run(vec!["NOP"]);
        let nop = out[0];
        assert_eq!(nop >> 24, Opcode::NOP as u32);
        assert_eq!((nop >> 19) & 0x1F, 0); // rd = 0
        assert_eq!((nop >> 14) & 0x1F, 0); // rs1 = 0
        assert_eq!((nop >> 9) & 0x1F, 0); // rs2 = 0
    }

    #[test]
    fn test_i_type_decimal_and_hex() {
        let out = run(vec!["ADDI r1, r2, #5", "SUBI r3, r4, 0x0A"]);
        let addi = out[0];
        let subi = out[1];
        assert_eq!(addi >> 24, Opcode::ADDI as u32);
        assert_eq!((addi >> 19) & 0x1F, 1); // rd = r1
        assert_eq!((addi >> 14) & 0x1F, 2); // rs1 = r2
        assert_eq!(addi & 0x3FFF, 5); // imm = 5
        assert_eq!(subi >> 24, Opcode::SUBI as u32);
        assert_eq!((subi >> 19) & 0x1F, 3); // rd = r3
        assert_eq!((subi >> 14) & 0x1F, 4); // rs1 = r4
        assert_eq!(subi & 0x3FFF, 10); // imm = 0x0A
    }

    #[test]
    fn test_i_type_negative_immediate() {
        let out = run(vec!["ADDI r1, r2, #-5"]);
        let addi = out[0];
        assert_eq!(addi >> 24, Opcode::ADDI as u32);
        assert_eq!((addi >> 19) & 0x1F, 1); // rd = r1
        assert_eq!((addi >> 14) & 0x1F, 2); // rs1 = r2
        assert_eq!(addi & 0x3FFF, (-5i32 as u32) & 0x3FFF); // imm = -5 (14-bit two’s complement)
    }

    #[test]
    fn test_i_type_unary() {
        let out = run(vec!["INCI r1, #5", "DECI r2, 0x0A"]);
        let inci = out[0];
        let deci = out[1];
        assert_eq!(inci >> 24, Opcode::INCI as u32);
        assert_eq!((inci >> 19) & 0x1F, 1); // rd = r1
        assert_eq!((inci >> 14) & 0x1F, 0); // rs1 = 0
        assert_eq!(inci & 0x3FFF, 5); // imm = 5
        assert_eq!(deci >> 24, Opcode::DECI as u32);
        assert_eq!((deci >> 19) & 0x1F, 2); // rd = r2
        assert_eq!((deci >> 14) & 0x1F, 0); // rs1 = 0
        assert_eq!(deci & 0x3FFF, 10); // imm = 0x0A
    }

    #[test]
    fn test_nopi() {
        let out = run(vec!["NOPI"]);
        let nopi = out[0];
        assert_eq!(nopi >> 24, Opcode::NOPI as u32);
        assert_eq!((nopi >> 19) & 0x1F, 0); // rd = 0
        assert_eq!((nopi >> 14) & 0x1F, 0); // rs1 = 0
        assert_eq!(nopi & 0x3FFF, 0); // imm = 0
    }

    #[test]
    fn test_mem_type() {
        let out = run(vec!["LDB r6, [r7, #16]", "STW r8, [r9, 0x20]"]);
        let ldb = out[0];
        let stw = out[1];
        assert_eq!(ldb >> 24, Opcode::LDB as u32);
        assert_eq!((ldb >> 19) & 0x1F, 6); // rd = r6
        assert_eq!((ldb >> 14) & 0x1F, 7); // rs1 = r7
        assert_eq!(ldb & 0x3FFF, 16); // imm = 16
        assert_eq!(stw >> 24, Opcode::STW as u32);
        assert_eq!((stw >> 19) & 0x1F, 8); // rd = r8
        assert_eq!((stw >> 14) & 0x1F, 9); // rs1 = r9
        assert_eq!(stw & 0x3FFF, 32); // imm = 0x20
    }

    #[test]
    fn test_mem_type_link_register() {
        let out = run(vec!["LDLR r1, r2", "STLR r3, r4"]);
        let ldlr = out[0];
        let stlr = out[1];
        assert_eq!(ldlr >> 24, Opcode::LDLR as u32);
        assert_eq!((ldlr >> 19) & 0x1F, 1); // rd = r1
        assert_eq!((ldlr >> 14) & 0x1F, 2); // rs1 = r2
        assert_eq!(ldlr & 0x3FFF, 0); // imm = 0
        assert_eq!(stlr >> 24, Opcode::STLR as u32);
        assert_eq!((stlr >> 19) & 0x1F, 3); // rd = r3
        assert_eq!((stlr >> 14) & 0x1F, 4); // rs1 = r4
        assert_eq!(stlr & 0x3FFF, 0); // imm = 0
    }

    #[test]
    fn test_j_type_forward_jump() {
        let out = run(vec![
            "JMP TARGET",
            "ADD r1, r1, r2",
            "ADD r1, r1, r2",
            "TARGET: NOP",
        ]);
        let jmp = out[0];
        assert_eq!(jmp >> 24, Opcode::JMP as u32);
        assert_eq!(jmp & 0xFFFFFF, 3);
    }

    #[test]
    fn test_j_type_backward_jump() {
        let out = run(vec![
            "TARGET: NOP",
            "ADD r1, r1, r2",
            "ADD r1, r1, r2",
            "JMP TARGET",
        ]);
        let jmp = out[3];
        assert_eq!(jmp >> 24, Opcode::JMP as u32);
        assert_eq!(jmp & 0xFFFFFF, (-3i32 as u32) & 0xFFFFFF);
    }

    #[test]
    fn test_j_type_call() {
        let out = run(vec!["CALL SUB", "ADD r1, r1, r2", "SUB: NOP"]);
        let call = out[0];
        assert_eq!(call >> 24, Opcode::CALL as u32);
        assert_eq!(call & 0xFFFFFF, 2);
    }

    #[test]
    fn test_j_type_ret_halt() {
        let out = run(vec!["RET", "HALT"]);
        let ret = out[0];
        let halt = out[1];
        assert_eq!(ret >> 24, Opcode::RET as u32);
        assert_eq!(ret & 0xFFFFFF, 0); // offset = 0
        assert_eq!(halt >> 24, Opcode::HALT as u32);
        assert_eq!(halt & 0xFFFFFF, 0); // offset = 0
    }

    #[test]
    fn test_move_system_r_type() {
        let out = run(vec!["MOV r1, r2", "MOVPC r3, r4"]);
        let mov = out[0];
        let movpc = out[1];
        assert_eq!(mov >> 24, Opcode::MOV as u32);
        assert_eq!((mov >> 19) & 0x1F, 1); // rd = r1
        assert_eq!((mov >> 14) & 0x1F, 2); // rs1 = r2
        assert_eq!((mov >> 9) & 0x1F, 0); // rs2 = 0
        assert_eq!(movpc >> 24, Opcode::MOVPC as u32);
        assert_eq!((movpc >> 19) & 0x1F, 3); // rd = r3
        assert_eq!((movpc >> 14) & 0x1F, 4); // rs1 = r4
        assert_eq!((movpc >> 9) & 0x1F, 0); // rs2 = 0
    }

    #[test]
    fn test_move_system_sys_type() {
        let out = run(vec!["LI r1, #100", "SETSP r2, 0x200"]);
        let li = out[0];
        let setsp = out[1];
        assert_eq!(li >> 24, Opcode::LI as u32);
        assert_eq!((li >> 19) & 0x1F, 1); // rd = r1
        assert_eq!(li & 0x7FFFF, 100); // imm = 100
        assert_eq!(setsp >> 24, Opcode::SETSP as u32);
        assert_eq!((setsp >> 19) & 0x1F, 2); // rd = r2
        assert_eq!(setsp & 0x7FFFF, 512); // imm = 0x200
    }

    #[test]
    fn test_fp_type() {
        let out = run(vec!["FADD r1, r2, r3", "FMOV r4, r5, r6"]);
        let fadd = out[0];
        let fmov = out[1];
        assert_eq!(fadd >> 24, Opcode::FADD as u32);
        assert_eq!((fadd >> 19) & 0x1F, 1); // rd = r1
        assert_eq!((fadd >> 14) & 0x1F, 2); // rs1 = r2
        assert_eq!((fadd >> 9) & 0x1F, 3); // rs2 = r3
        assert_eq!(fmov >> 24, Opcode::FMOV as u32);
        assert_eq!((fmov >> 19) & 0x1F, 4); // rd = r4
        assert_eq!((fmov >> 14) & 0x1F, 5); // rs1 = r5
        assert_eq!((fmov >> 9) & 0x1F, 6); // rs2 = r6
    }

    #[test]
    fn test_fp_type_mem() {
        let out = run(vec!["FLD r1, [r2, #16]", "FST r3, [r4, 0x20]"]);
        let fld = out[0];
        let fst = out[1];
        assert_eq!(fld >> 24, Opcode::FLD as u32);
        assert_eq!((fld >> 19) & 0x1F, 1); // rd = r1
        assert_eq!((fld >> 14) & 0x1F, 2); // rs1 = r2
        assert_eq!(fld & 0x3FFF, 16); // imm = 16
        assert_eq!(fst >> 24, Opcode::FST as u32);
        assert_eq!((fst >> 19) & 0x1F, 3); // rd = r3
        assert_eq!((fst >> 14) & 0x1F, 4); // rs1 = r4
        assert_eq!(fst & 0x3FFF, 32); // imm = 0x20
    }

    #[test]
    fn test_io_type() {
        let out = run(vec!["IN r1, 0x10", "OUT r2, 0x20"]);
        let in_ = out[0];
        let out_ = out[1];
        assert_eq!(in_ >> 24, Opcode::IN as u32);
        assert_eq!((in_ >> 3) & 0x1F, 1); // rd = r1
        assert_eq!((in_ >> 8) & 0xFFFF, 16); // port = 0x10
        assert_eq!(out_ >> 24, Opcode::OUT as u32);
        assert_eq!((out_ >> 3) & 0x1F, 2); // rd = r2
        assert_eq!((out_ >> 8) & 0xFFFF, 32); // port = 0x20
    }

    #[test]
    fn test_ignore_comments_and_case() {
        let out = run(vec![
            "; comentario",
            "AdD r1, r2, r3 ; otro comentario",
            "addi R4, R5, #10",
        ]);
        let add = out[0];
        let addi = out[1];
        assert_eq!(add >> 24, Opcode::ADD as u32);
        assert_eq!((add >> 19) & 0x1F, 1); // rd = r1
        assert_eq!((add >> 14) & 0x1F, 2); // rs1 = r2
        assert_eq!((add >> 9) & 0x1F, 3); // rs2 = r3
        assert_eq!(addi >> 24, Opcode::ADDI as u32);
        assert_eq!((addi >> 19) & 0x1F, 4); // rd = r4
        assert_eq!((addi >> 14) & 0x1F, 5); // rs1 = r5
        assert_eq!(addi & 0x3FFF, 10); // imm = 10
    }

    #[test]
    #[should_panic(expected = "Unknown opcode: INVALID")]
    fn test_invalid_opcode() {
        run(vec!["INVALID r1, r2, r3"]);
    }

    #[test]
    #[should_panic(expected = "Unknown label: NONEXISTENT")]
    fn test_invalid_label() {
        run(vec!["JMP NONEXISTENT"]);
    }
}
