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
            0x09 => {
                self.alu_add16(self.reg.bc());
                2
            }
            0x19 => {
                self.alu_add16(self.reg.de());
                2
            }
            0x29 => {
                self.alu_add16(self.reg.hl());
                2
            }
            0x39 => {
                self.alu_add16(self.reg.sp);
                2
            }
            0x80 => {
                self.alu_add(self.reg.b, false);
                1
            }
            0x81 => {
                self.alu_add(self.reg.c, false);
                1
            }
            0x82 => {
                self.alu_add(self.reg.d, false);
                1
            }
            0x83 => {
                self.alu_add(self.reg.e, false);
                1
            }
            0x84 => {
                self.alu_add(self.reg.h, false);
                1
            }
            0x85 => {
                self.alu_add(self.reg.l, false);
                1
            }
            0x86 => {
                let byte = self.mmu.read_byte(self.reg.hl());
                self.alu_add(byte, false);
                2
            }
            0x87 => {
                self.alu_add(self.reg.a, false);
                1
            }
            0x88 => {
                self.alu_add(self.reg.b, true);
                1
            }
            0x89 => {
                self.alu_add(self.reg.c, true);
                1
            }
            0x8A => {
                self.alu_add(self.reg.d, true);
                1
            }
            0x8B => {
                self.alu_add(self.reg.e, true);
                1
            }
            0x8C => {
                self.alu_add(self.reg.h, true);
                1
            }
            0x8D => {
                self.alu_add(self.reg.l, true);
                1
            }
            0x8E => {
                let byte = self.mmu.read_byte(self.reg.hl());
                self.alu_add(byte, true);
                2
            }
            0x8F => {
                self.alu_add(self.reg.a, true);
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
            0xE8 => {
                self.reg.sp = self.alu_add16imm(self.reg.sp);
                4
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
    fn alu_add16(&mut self, b: u16) {
        let a = self.reg.hl();
        let r = a.wrapping_add(b);
        self.reg
            .flag(CpuFlag::H, (a & 0x07FF) + (b & 0x07FF) > 0x07FF);
        self.reg.flag(CpuFlag::N, false);
        self.reg.flag(CpuFlag::C, a > 0xFFFF - b);
        self.reg.sethl(r);
    }
    fn alu_add16imm(&mut self, a: u16) -> u16 {
        let b = self.fetchbyte() as u16;
        self.reg.flag(CpuFlag::N, false);
        self.reg.flag(CpuFlag::Z, false);
        self.reg
            .flag(CpuFlag::H, (a & 0x000F) + (b & 0x000F) > 0x000F);
        self.reg
            .flag(CpuFlag::C, (a & 0x00FF) + (b & 0x00FF) > 0x00FF);
        return a.wrapping_add(b);
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

    #[test]
    fn bit16_add() {
        let mut cpu = CPU::new();
        cpu.reg.setbc(0x0605);
        cpu.reg.sethl(0x8A23);
        let cycle = cpu.call(0x09);
        assert_eq!(cpu.reg.hl(), 0x9028);
        assert_eq!(cpu.reg.f, 0b00100000);

        cpu.reg.sp = 0xFFFF;
        cpu.mmu.memory[0] = 2;
        let cycle = cpu.call(0xE8);
        assert_eq!(cpu.reg.sp, 0x0001);
        assert_eq!(cpu.reg.f, 0b00110000);
    }
}
