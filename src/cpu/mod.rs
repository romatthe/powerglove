pub mod cpu_addr;
pub mod cpu_instr;
pub mod instructions;

use bitflags::bitflags;
use crate::bus::Bus;

use self::instructions::Instruction;

bitflags! {
    pub struct StatusFlags: u8 {
        /// Carry flag
        const C = 1;
        /// Zero flag
        const Z = 1 << 1;
        /// Disable interrupts flag
        const I = 1 << 2;
        /// Decimal mode flag (Unused, as the NES's version of the 6502 does NOT support hardware decimal mode)
        const D = 1 << 3;
        /// Break flag
        const B = 1 << 4;
        /// (Unused)
        const U = 1 << 5;
        /// Overflow flag
        const V = 1 << 6;
        /// Negative flag
        const N = 1 << 7;
    }
}

#[derive(Debug)]
pub struct CPU {
    /// The memory bus
    pub bus: Bus,

    // Registers

    /// The status register
    pub status: StatusFlags,
    /// The accumulator register
    pub a: u8,
    /// The general purpose x register
    pub x: u8,
    /// The general purpose y register
    pub y: u8,
    /// The stack pointer
    pub sp: u8,
    /// The program counter
    pub pc: u16,

    // Current instruction execution

    /// Data that was fetched as part of the addressing
    pub fetched: u8,
    /// Location in memory read from as part of the addressing
    pub addr_abs: u16,
    /// Represents absolute address following a branch
    pub addr_rel: u16,   
    /// The amount of cycles remaining for the current instruction
    pub cycles_remaining: u8,
    /// The opcode that's currently being executed
    pub opcode: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU { 
            bus: Bus::new(),
            status: StatusFlags::empty(),
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            fetched: 0,
            addr_abs: 0,
            addr_rel: 0,
            cycles_remaining: 0,
            opcode: 0,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.bus.read(address)
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.bus.write(address, data);
    }

    /// Reset the CPU to its initial boot state
    pub fn reset(&mut self) {
        
    }

    /// Simulates the passing of a single clock cycle
    pub fn clock(&mut self) {
        // No more cycles are remaining in the currently executing instruction
        if self.cycles_remaining == 0 {
            // Set the next opcode to execute
            self.opcode = self.read(self.pc);
            self.pc = self.pc.wrapping_add(1);
            
            // Set how many clock cycles we need to execute
            self.cycles_remaining = Instruction::decode(self.opcode).cycles;

            // Fetch operands with the correct addressing mode and execute the instruction
            let more_cycles1 = (Instruction::decode(self.opcode).mode_exec)(self);
            let more_cycles2 = (Instruction::decode(self.opcode).op_exec)(self);

            // If the previous two actions indicated that there are more cycles to complete
            // than normal, we add those to the total cycle count for the current instruction.
            self.cycles_remaining += more_cycles1 & more_cycles2;
        }

        // Each call of the `clock` function, we decrement a single one of our remaining cycles
        self.cycles_remaining -= 1;
    }

    /// Simulate an interrupt request signal 
    pub fn irq(&self) {

    }

    /// Simulate a non-maskable interrupt request signal
    pub fn nmi(&self) {

    }
}