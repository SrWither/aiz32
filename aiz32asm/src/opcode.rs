use aiz32core::instruction::Opcode;
use std::collections::HashMap;

pub fn opcode_table() -> HashMap<String, Opcode> {
    use Opcode::*;
    let mut map = HashMap::new();

    // ALU (R-type)
    map.insert("NOP".into(), NOP);
    map.insert("ADD".into(), ADD);
    map.insert("SUB".into(), SUB);
    map.insert("MUL".into(), MUL);
    map.insert("DIV".into(), DIV);
    map.insert("MOD".into(), MOD);
    map.insert("INC".into(), INC);
    map.insert("DEC".into(), DEC);
    map.insert("NEG".into(), NEG);
    map.insert("ABS".into(), ABS);
    map.insert("AND".into(), AND);
    map.insert("OR".into(), OR);
    map.insert("XOR".into(), XOR);
    map.insert("NAND".into(), NAND);
    map.insert("NOR".into(), NOR);
    map.insert("XNOR".into(), XNOR);
    map.insert("NOT".into(), NOT);
    map.insert("SHL".into(), SHL);
    map.insert("SHR".into(), SHR);
    map.insert("SAR".into(), SAR);
    map.insert("ROL".into(), ROL);
    map.insert("ROR".into(), ROR);
    map.insert("SEXTB".into(), SEXTB);
    map.insert("ZEXTB".into(), ZEXTB);
    map.insert("POPCNT".into(), POPCNT);
    map.insert("CMP".into(), CMP);
    map.insert("UCMP".into(), UCMP);
    map.insert("SETZ".into(), SETZ);
    map.insert("SETNZ".into(), SETNZ);
    map.insert("PASS".into(), PASS);
    map.insert("SEXTH".into(), SEXTH);
    map.insert("ZEXTH".into(), ZEXTH);

    // ALU (I-type)
    map.insert("NOPI".into(), NOPI);
    map.insert("ADDI".into(), ADDI);
    map.insert("SUBI".into(), SUBI);
    map.insert("MULI".into(), MULI);
    map.insert("DIVI".into(), DIVI);
    map.insert("MODI".into(), MODI);
    map.insert("INCI".into(), INCI);
    map.insert("DECI".into(), DECI);
    map.insert("NEGI".into(), NEGI);
    map.insert("ABSI".into(), ABSI);
    map.insert("ANDI".into(), ANDI);
    map.insert("ORI".into(), ORI);
    map.insert("XORI".into(), XORI);
    map.insert("NANDI".into(), NANDI);
    map.insert("NORI".into(), NORI);
    map.insert("XNORI".into(), XNORI);
    map.insert("NOTI".into(), NOTI);
    map.insert("SHLI".into(), SHLI);
    map.insert("SHRI".into(), SHRI);
    map.insert("SARI".into(), SARI);
    map.insert("ROLI".into(), ROLI);
    map.insert("RORI".into(), RORI);
    map.insert("SEXTBI".into(), SEXTBI);
    map.insert("ZEXTBI".into(), ZEXTBI);
    map.insert("POPCNTI".into(), POPCNTI);
    map.insert("CMPI".into(), CMPI);
    map.insert("UCMPI".into(), UCMPI);
    map.insert("SETZI".into(), SETZI);
    map.insert("SETNZI".into(), SETNZI);
    map.insert("PASSI".into(), PASSI);
    map.insert("SEXTHI".into(), SEXTHI);
    map.insert("ZEXTHI".into(), ZEXTHI);

    // Memory
    map.insert("LDB".into(), LDB);
    map.insert("LDBU".into(), LDBU);
    map.insert("LDH".into(), LDH);
    map.insert("LDHU".into(), LDHU);
    map.insert("LDW".into(), LDW);
    map.insert("STB".into(), STB);
    map.insert("STH".into(), STH);
    map.insert("STW".into(), STW);
    map.insert("LDLR".into(), LDLR);
    map.insert("STLR".into(), STLR);

    // Jumps & Branch
    map.insert("JMP".into(), JMP);
    map.insert("JZ".into(), JZ);
    map.insert("JNZ".into(), JNZ);
    map.insert("JEQ".into(), JEQ);
    map.insert("JNE".into(), JNE);
    map.insert("JLT".into(), JLT);
    map.insert("JGT".into(), JGT);
    map.insert("JLE".into(), JLE);
    map.insert("JGE".into(), JGE);
    map.insert("JC".into(), JC);
    map.insert("JO".into(), JO);
    map.insert("CALL".into(), CALL);
    map.insert("RET".into(), RET);
    map.insert("HALT".into(), HALT);

    // Move & System
    map.insert("MOV".into(), MOV);
    map.insert("LI".into(), LI);
    map.insert("LUI".into(), LUI);
    map.insert("MOVPC".into(), MOVPC);
    map.insert("MTSR".into(), MTSR);
    map.insert("MFSR".into(), MFSR);
    map.insert("MOVSP".into(), MOVSP);
    map.insert("SETSP".into(), SETSP);

    // Floating Point
    map.insert("FADD".into(), FADD);
    map.insert("FSUB".into(), FSUB);
    map.insert("FMUL".into(), FMUL);
    map.insert("FDIV".into(), FDIV);
    map.insert("FCMP".into(), FCMP);
    map.insert("FEQ".into(), FEQ);
    map.insert("FLT".into(), FLT);
    map.insert("FGT".into(), FGT);
    map.insert("FTOI".into(), FTOI);
    map.insert("ITOF".into(), ITOF);
    map.insert("FMOV".into(), FMOV);
    map.insert("FLD".into(), FLD);
    map.insert("FST".into(), FST);

    // IO
    map.insert("IN".into(), IN);
    map.insert("OUT".into(), OUT);

    map
}
