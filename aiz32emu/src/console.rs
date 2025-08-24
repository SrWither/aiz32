use aiz32core::peripheral::Peripheral;

pub struct Console {
    last_value: u32,
}

impl Console {
    pub fn new() -> Self {
        Self { last_value: 0 }
    }
}

impl Peripheral for Console {
    fn handles_port(&self, port: u16) -> bool {
        matches!(port, 0x00 | 0x01 | 0x02 | 0x03)
    }

    fn read(&self, port: u16) -> u32 {
        let last_value = match port {
            0x02 => 0x100,
            0x03 => 0x101,
            _ => self.last_value,
        };

        last_value
    }

    fn write(&mut self, port: u16, value: u32) {
        self.last_value = value;
        println!("[Console] OUT a puerto 0x{:X}: {}", port, value);
    }
}
