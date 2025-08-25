use std::{cell::RefCell, rc::Rc};

use crate::peripheral::Peripheral;

pub struct RAM {
    pub data: Vec<u8>,
}

impl RAM {
    #[inline]
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    #[inline]
    pub fn read8(&self, addr: u32) -> u8 {
        self.data[addr as usize]
    }
    #[inline]
    pub fn write8(&mut self, addr: u32, value: u8) {
        self.data[addr as usize] = value;
    }

    #[inline]
    pub fn read16(&self, addr: u32) -> u16 {
        let i = addr as usize;
        u16::from_le_bytes([self.data[i], self.data[i + 1]])
    }
    #[inline]
    pub fn write16(&mut self, addr: u32, value: u16) {
        let i = addr as usize;
        let b = value.to_le_bytes();
        self.data[i] = b[0];
        self.data[i + 1] = b[1];
    }

    #[inline]
    pub fn read32(&self, addr: u32) -> u32 {
        let i = addr as usize;
        u32::from_le_bytes([
            self.data[i],
            self.data[i + 1],
            self.data[i + 2],
            self.data[i + 3],
        ])
    }
    #[inline]
    pub fn write32(&mut self, addr: u32, value: u32) {
        let i = addr as usize;
        let b = value.to_le_bytes();
        self.data[i] = b[0];
        self.data[i + 1] = b[1];
        self.data[i + 2] = b[2];
        self.data[i + 3] = b[3];
    }
}

pub struct ROM {
    pub data: Vec<u8>,
}

impl ROM {
    #[inline]
    pub fn new(contents: Vec<u8>) -> Self {
        Self { data: contents }
    }

    #[inline]
    pub fn read8(&self, addr: u32) -> u8 {
        self.data[addr as usize]
    }

    #[inline]
    pub fn read16(&self, addr: u32) -> u16 {
        let i = addr as usize;
        u16::from_le_bytes([self.data[i], self.data[i + 1]])
    }

    #[inline]
    pub fn read32(&self, addr: u32) -> u32 {
        let i = addr as usize;
        u32::from_le_bytes([
            self.data[i],
            self.data[i + 1],
            self.data[i + 2],
            self.data[i + 3],
        ])
    }
}

pub struct MemoryBus {
    pub ram: RAM,
    pub rom: ROM,
}

impl MemoryBus {
    pub fn new(ram_size: usize, rom_contents: Vec<u8>) -> Self {
        Self {
            ram: RAM::new(ram_size),
            rom: ROM::new(rom_contents),
        }
    }

    pub fn read8(&self, addr: u32) -> u8 {
        if addr < self.ram.data.len() as u32 {
            self.ram.read8(addr)
        } else {
            self.rom.read8(addr - self.ram.data.len() as u32)
        }
    }

    pub fn write8(&mut self, addr: u32, value: u8) {
        if addr < self.ram.data.len() as u32 {
            self.ram.write8(addr, value);
        } else {
            panic!("Cannot write to ROM");
        }
    }

    pub fn read16(&self, addr: u32) -> u16 {
        if addr < self.ram.data.len() as u32 {
            self.ram.read16(addr)
        } else {
            self.rom.read16(addr - self.ram.data.len() as u32)
        }
    }

    pub fn write16(&mut self, addr: u32, value: u16) {
        if addr < self.ram.data.len() as u32 {
            self.ram.write16(addr, value);
        } else {
            panic!("Cannot write to ROM");
        }
    }

    pub fn read32(&self, addr: u32) -> u32 {
        if addr < self.ram.data.len() as u32 {
            self.ram.read32(addr)
        } else {
            self.rom.read32(addr - self.ram.data.len() as u32)
        }
    }

    pub fn write32(&mut self, addr: u32, value: u32) {
        if addr < self.ram.data.len() as u32 {
            self.ram.write32(addr, value);
        } else {
            panic!("Cannot write to ROM");
        }
    }

    pub fn ram_size(&self) -> usize {
        self.ram.data.len()
    }

    pub fn rom_size(&self) -> usize {
        self.rom.data.len()
    }
}

pub struct IO {
    ports: Vec<u32>,
    peripherals: Vec<Rc<RefCell<dyn Peripheral>>>,
}

impl IO {
    pub fn new() -> Self {
        Self {
            ports: vec![0; 65536],
            peripherals: Vec::new(),
        }
    }

    pub fn register_peripheral(&mut self, peripheral: Rc<RefCell<dyn Peripheral>>) {
        self.peripherals.push(peripheral);
    }

    pub fn read(&self, port: u16) -> u32 {
        for peripheral in &self.peripherals {
            let p = peripheral.borrow();
            if p.handles_port(port) {
                return p.read(port);
            }
        }
        self.ports[port as usize]
    }

    pub fn write(&mut self, port: u16, value: u32) {
        self.ports[port as usize] = value;
        for peripheral in &self.peripherals {
            let mut p = peripheral.borrow_mut();
            if p.handles_port(port) {
                p.write(port, value);
            }
        }
    }
}
