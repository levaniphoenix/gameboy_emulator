#[derive(Debug)]
pub struct MMU {
    pub memory: [u8; 0xffff],
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            memory: [0; 0xffff],
        }
    }
    pub fn read_byte(&mut self, address: u16) -> u8 {
        // match address {
        //     0x0000..=0x7FFF => self.memory[address as usize],
        //     _ => 0,
        // }
        self.memory[address as usize]
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

#[cfg(test)]
mod test {
    use crate::mmu::MMU;
    #[test]
    fn mmu_test() {
        let mut mmu = MMU::new();
        mmu.write_byte(0x0010, 0xf2);
        println!("{:?}", mmu.read_byte(0x0010))
    }
}
