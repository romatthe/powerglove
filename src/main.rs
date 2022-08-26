use crate::cpu::CPU;

mod bus;
mod cpu;

fn main() {
    println!("Hello, world!");

    let cpu = CPU::new();

    println!("{:?}", cpu);
}
