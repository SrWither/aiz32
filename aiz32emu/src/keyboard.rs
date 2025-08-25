use aiz32core::peripheral::Peripheral;
use std::collections::VecDeque;

pub struct Keyboard {
    buffer: VecDeque<u8>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
        }
    }

    pub fn key_down(&mut self, key: u8) {
        self.buffer.push_back(key);
    }

    pub fn key_up(&mut self, _key: u8) {
    }
}

impl Peripheral for Keyboard {
    fn handles_port(&self, port: u16) -> bool {
        (0x3000..=0x3001).contains(&port)
    }

    fn read(&self, port: u16) -> u32 {
        match port {
            0x3000 => {
                self.buffer.front().copied().unwrap_or(0) as u32
            }
            0x3001 => {
                if self.buffer.is_empty() {
                    0
                } else {
                    1
                }
            }
            _ => 0,
        }
    }

    fn write(&mut self, port: u16, value: u32) {
        match port {
            0x3000 => {
                if value == 0 {
                    self.buffer.pop_front();
                }
            }
            _ => {}
        }
    }
}
