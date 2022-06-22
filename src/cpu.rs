use crate::registers::Registers;
pub struct CPU {
    pub reg: Registers,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg: Registers::new(),
        }
    }
    pub fn call(&mut self, opcode: u8) -> u32 {
        match opcode {
            0x80 => {
                self.alu_add(self.reg.b);
                1
            }
            other => panic!("Instruction {:2X} is not implemented", other),
        }
    }
    fn alu_add(&mut self, byte: u8) {
        let a = self.reg.a;
        let r = a.wrapping_add(byte);
        self.reg.a = r;
    }
}
