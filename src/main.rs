mod cpu;
mod registers;
use crate::cpu::CPU;
use crate::registers::Registers;

fn main() {
    // let reg = Registers::new();
    // println!("{:?}", reg);
    let mut cpu = CPU::new();
    cpu.call(0x80);
    println!("{:?}", cpu.reg)
}
