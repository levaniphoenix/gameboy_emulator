#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

pub enum CpuFlag {
    C = 0b00010000,
    H = 0b00100000,
    N = 0b01000000,
    Z = 0b10000000,
}
impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x00,
            f: 0x00,
            b: 0x70,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00,
            pc: 0x0000,
            sp: 0x0000,
        }
    }
    pub fn flag(&mut self, flags: CpuFlag, set: bool) {
        let mask = flags as u8;
        match set {
            true => self.f |= mask,
            false => self.f &= !mask,
        }
        self.f &= 0xF0;
    }
    pub fn getflag(&self, flags: CpuFlag) -> bool {
        let mask = flags as u8;
        self.f & mask > 0
    }
}
