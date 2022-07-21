#![allow(dead_code)]
#![allow(unused_imports)]
mod cpu;
mod mmu;
mod registers;
use crate::cpu::CPU;
use crate::mmu::MMU;
use crate::registers::Registers;
fn main() {
    // let reg = Registers::new();
    // println!("{:?}", reg);
    let mut cpu = CPU::new();
    cpu.call(0x80);
    println!("{:?}", cpu.reg)
}
