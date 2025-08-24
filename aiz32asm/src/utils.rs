pub fn parse_reg(reg: &str) -> u8 {
    let r = reg.trim_start_matches('R').trim_start_matches('F');
    r.parse::<u8>().unwrap()
}

pub fn parse_imm(imm: &str, bits: u32) -> u32 {
    let imm = imm
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .trim_end_matches(',')
        .to_lowercase();

    let value: i32 = if let Some(hex) = imm.strip_prefix("0x") {
        i32::from_str_radix(hex, 16).unwrap()
    } else if let Some(dec) = imm.strip_prefix('#') {
        dec.parse::<i32>().unwrap()
    } else {
        imm.parse::<i32>().unwrap()
    };

    (value as u32) & ((1 << bits) - 1)
}

pub fn parse_port(port: &str) -> u16 {
    if port.starts_with("0X") {
        u16::from_str_radix(&port[2..], 16).unwrap()
    } else {
        port.parse::<u16>().unwrap()
    }
}
