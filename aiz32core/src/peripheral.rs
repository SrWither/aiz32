pub trait Peripheral {
    fn handles_port(&self, port: u16) -> bool;
    fn read(&self, port: u16) -> u32;
    fn write(&mut self, port: u16, value: u32);
}