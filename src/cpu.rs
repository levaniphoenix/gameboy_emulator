use crate::mmu::MMU;
use crate::registers::CpuFlag;
use crate::registers::Registers;

pub struct CPU {
    pub reg: Registers,
    pub mmu: MMU,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg: Registers::new(),
            mmu: MMU::new(),
        }
    }

    fn fetchbyte(&mut self) -> u8 {
        let byte = self.mmu.read_byte(self.reg.pc);
        self.reg.pc += 1;
        byte
    }

    pub fn call(&mut self, opcode: u8) -> u32 {
        match opcode {
            0x80 => {
                self.alu_add(self.reg.b, false);
                1
            }
            0x88 => {
                self.alu_add(self.reg.b, true);
                1
            }
            0xC6 => {
                let n = self.fetchbyte();
                self.alu_add(n, false);
                2
            }
            0xCE => {
                let n = self.fetchbyte();
                self.alu_add(n, true);
                2
            }
            other => panic!("Instruction {:2X} is not implemented", other),
        }
    }
    fn alu_add(&mut self, byte: u8, usec: bool) {
        let c = if usec && self.reg.getflag(CpuFlag::C) {
            1
        } else {
            0
        };
        let a = self.reg.a;
        let r = a.wrapping_add(byte).wrapping_add(c);

        self.reg.flag(CpuFlag::Z, r == 0);
        self.reg.flag(CpuFlag::N, false);
        self.reg
            .flag(CpuFlag::H, (a & 0xF) + (byte & 0xF) + c > 0xF);
        self.reg
            .flag(CpuFlag::C, (a as u16) + (byte as u16) + (c as u16) > 0xFF);

        self.reg.a = r;
    }
}
#[cfg(test)]
mod test {
    use super::CPU;

    #[test]
    fn add_b_to_a() {
        let mut cpu = CPU::new();
        cpu.reg.a = 0x3A;
        cpu.reg.b = 0xC6;
        let cycle = cpu.call(0x80);
        assert_eq!(cpu.reg.a, 0);
        assert_eq!(cpu.reg.f, 0b10110000)
    }
    #[test]
    fn add_immediate_to_a() {
        let mut cpu = CPU::new();
        cpu.mmu.memory[0] = 0xFF;
        cpu.reg.a = 0x3C;
        let cycle = cpu.call(0xC6);
        assert_eq!(cpu.reg.a, 0x3B);
        assert_eq!(cpu.reg.f, 0b00110000);
    }
}
