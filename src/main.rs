use crate::cpu::{CPU, disassemble::Disassembler};

mod bus;
mod cpu;

fn main() {
    let mut cpu = CPU::new();
    
    let prog = "A2 00 8E 00 00 A2 40 8E 01 00 A9 00 8D 10 00 8D 11 00 A2 08 38 AD 00 00 E9 40 A8 AD 11 00 ED 10 00 90 06 8C 00 00 8D 11 00 2E 10 00 0E 01 00 2E 00 00 2E 11 00 0E 01 00 2E 00 00 2E 11 00 CA D0 D3";
    let prog_array: Vec<_> = prog.split_whitespace().collect();
    for (index, prog_n) in prog_array.iter().enumerate() {
        cpu.bus.ram[0x8000 + index] = u8::from_str_radix(prog_n, 16).unwrap();
    }

    cpu.bus.ram[0xFFFC] = 0x00;
    cpu.bus.ram[0xFFFD] = 0x80;

    let code_map = Disassembler::for_range(&cpu, 0x8000, 0xFFFF);

    for (_, op_str) in code_map {
        println!("{}", op_str);
    }
}
