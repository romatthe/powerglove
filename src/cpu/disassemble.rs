use crate::cpu::instructions::{AddressingMode, Instruction};

use super::CPU;

pub struct Disassembler;

impl Disassembler {
    /// Disassemble the program in memory from address start to address end.
    pub fn for_range(cpu: &CPU, start: u16, stop: u16) -> Vec<(u16, String)> {
        let mut current_addr = start as u32;
        let mut instr_lines = Vec::new();

        // Iteratre over all addresses as long as we have not reached the end
        while current_addr <= stop as u32 {
            let op_addr = current_addr as u16;
            let op = Instruction::decode(cpu.read(op_addr)); 
            let mut instr = format!("${}: {:?}", format!("{:04X}", current_addr), op.mnemonic);

            current_addr += 1;

            match op.mode {
                AddressingMode::IMP => {
                    instr += "  {IMP}";
                },
                AddressingMode::IMM => {
                    let fetched = cpu.read(current_addr as u16);
                    instr = format!("{} #${} {{IMP}}", instr, format!("{:02X}", fetched));
                    current_addr += 1;
                },
                AddressingMode::ZP0 => {
                    let lo = cpu.read(current_addr as u16);
                    instr = format!("{} ${} {{ZP0}}", instr, format!("{:02X}", lo));
                    current_addr += 1;
                },
                AddressingMode::ZPX => {
                    let lo = cpu.read(current_addr as u16);
                    instr = format!("{} ${}, X {{ZPX}}", instr, format!("{:02X}", lo));
                    current_addr += 1;
                },
                AddressingMode::ZPY => {
                    let lo = cpu.read(current_addr as u16);
                    instr = format!("{} ${}, Y {{ZPY}}", instr, format!("{:02X}", lo));
                    current_addr += 1;
                },
                AddressingMode::ABS => {
                    let lo = cpu.read(current_addr as u16);
                    let hi = cpu.read(current_addr as u16 + 1);
                    let val = u16::from_le_bytes([lo, hi]);
                    instr = format!("{} ${} {{ABS}}", instr, format!("{:04X}", val));
                    current_addr += 2;
                },
                AddressingMode::ABX => {
                    let lo = cpu.read(current_addr as u16);
                    let hi = cpu.read(current_addr as u16 + 1);
                    let val = u16::from_le_bytes([lo, hi]);
                    instr = format!("{} ${}, X {{ABX}}", instr, format!("{:04X}", val));
                    current_addr += 2;
                },
                AddressingMode::ABY => {
                    let lo = cpu.read(current_addr as u16);
                    let hi = cpu.read(current_addr as u16);
                    let val = u16::from_le_bytes([lo, hi]);
                    instr = format!("{} ${}, Y {{ABY}}", instr, format!("{:04X}", val));
                    current_addr += 2;
                },
                AddressingMode::IND => {
                    let lo = cpu.read(current_addr as u16);
                    let hi = cpu.read(current_addr as u16);
                    let val = u16::from_le_bytes([lo, hi]);
                    instr = format!("{} (${}) {{IND}}", instr, format!("{:04X}", val));
                    current_addr += 2;
                },
                AddressingMode::ACC => {
                    // No further formatting
                },
                AddressingMode::REL => {
                    let val = cpu.read(current_addr as u16);
                    current_addr += 1;
                    instr = format!("{} ${} [${}] {{REL}}", instr,
                        format!("{:02X}", val),
                        format!("{:04X}", current_addr.wrapping_add((val as i8) as u32)));
                },
                AddressingMode::IZX => {
                    let lo = cpu.read(current_addr as u16);
                    instr = format!("{} (${}, X) {{IZX}}", instr, format!("{:02X}", lo));
                    current_addr += 1;
                },
                AddressingMode::IZY => {
                    let lo = cpu.read(current_addr as u16);
                    instr = format!("{} (${}), Y {{IZY}}", instr, format!("{:02X}", lo));
                    current_addr += 1;
                },
            }

            instr_lines.push((op_addr, instr));
        }

        instr_lines
    }
}