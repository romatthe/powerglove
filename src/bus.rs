const RAM_SIZE: usize = 64 * 1024;

#[derive(Debug)]
pub struct Bus {
    pub ram: [u8; RAM_SIZE],
}

impl Bus {
    pub fn new() -> Self {
        Bus { 
            ram: [0x0; RAM_SIZE] 
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            (0x0000..=0xFFFF) => self.ram[address as usize],
            _ => 0x0,
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            (0x0000..=0xFFFF) => self.ram[address as usize] = data,
        }
    }
}