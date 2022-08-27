pub mod cpu_addr;
pub mod cpu_instr;
pub mod disassemble;
pub mod instructions;

use bitflags::bitflags;
use crate::bus::Bus;
use self::instructions::{AddressingMode, Instruction};

/// Base location of the stack to which we can add the stack pointer offset.
pub const STACK_BASE: u16 = 0x0100;
/// Base location of the value in memory pointing to the location of the initial
/// value of the program counter on reset.
pub const PC_POINTER: u16 = 0xFFFC;
/// Base location of the value in memory pointing to the location of the initial
/// value of the program counter when an IRQ occurs.
pub const IRQ_POINTER: u16 = 0xFFFE;
/// Base location of the value in memory pointing to the location of the initial
/// value of the program counter when an NMI occurs.
pub const NMI_POINTER: u16 = 0xFFFA;

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
        self.a = 0;
        self.x = 0;
        self.y = 0;

        self.sp = 0xFD;
        self.status = StatusFlags::U;

        // Locating set by the programmer pointing to the location of the 
        // program counter on reset.
        let lo = self.read(PC_POINTER);
        let hi = self.read(PC_POINTER + 1);
        self.pc = u16::from_le_bytes([lo, hi]);

        self.addr_rel = 0x0000;
        self.addr_abs = 0x0000;
        self.fetched = 0x00;

        // Resets and interrupts actually consume cycles
        self.cycles_remaining = 8;
    }

    fn fetch(&mut self) -> u8 {
        let instr = Instruction::decode(self.opcode);

        // Use the absolute address to fetch from memory unless we're in implied
        // addressing mode.
        if instr.mode != AddressingMode::IMP {
            self.fetched = self.read(self.addr_abs);
        }

        self.fetched
    }

    /// Simulates the passing of a single clock cycle
    fn clock(&mut self) {
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

            // If the previous two actions indicated that they both require additional cycles
            // we add those to the total need to complete for this instruction.
            self.cycles_remaining += more_cycles1 & more_cycles2;
        }

        // Each call of the `clock` function, we decrement a single one of our remaining cycles
        self.cycles_remaining -= 1;
    }

    /// Simulate an interrupt request signal 
    pub fn irq(&mut self) {
        // Only run the interrupt if the interrupt disable flag is not set
        if !self.status.contains(StatusFlags::I) {
            // On interrupt, we write data to the stack so we can resume out program later. First
            // is the current program counter.
            self.write(STACK_BASE + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
            self.write(STACK_BASE + self.sp as u16 - 1, (self.pc & 0x00FF) as u8);
            self.sp -= 2;

            // Next we set the correct status flags and push those unto the stack as well
            self.status.set(StatusFlags::B, false); // Set to 0 when pushing to the stack during IRQ/NMI, 1 during PHP/BRK
            self.status.set(StatusFlags::U, true);  // Always set to 1 when pushed to the stack during IRQ
            self.status.set(StatusFlags::I, true);  // Disable interrupts during an interrupt
            self.write(STACK_BASE + self.sp as u16, self.status.bits);
            self.sp -= 1;

            // We look up the value of the interrupt handler we're supposed to execute at `IRQ_POINTER` and set the
            // program counter there.
            let lo = self.read(IRQ_POINTER);
            let hi = self.read(IRQ_POINTER + 1);
            self.pc = u16::from_be_bytes([lo, hi]);

            // Resets and interrupts actually consume cycles
            self.cycles_remaining = 7;
        }
    }

    /// Simulate a non-maskable interrupt request signal. Cannot be stopped from ocurring.
    pub fn nmi(&mut self) {
        // On interrupt, we write data to the stack so we can resume out program later. First
        // is the current program counter.
        self.write(STACK_BASE + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.write(STACK_BASE + self.sp as u16 - 1, (self.pc & 0x00FF) as u8);
        self.sp -= 2;

        // Next we set the correct status flags and push those unto the stack as well
        self.status.set(StatusFlags::B, false); // Set to 0 when pushing to the stack during IRQ/NMI, 1 during PHP/BRK
        self.status.set(StatusFlags::U, true);  // Always set to 1 when pushed to the stack during IRQ
        self.status.set(StatusFlags::I, true);  // Disable interrupts during an interrupt
        self.write(STACK_BASE + self.sp as u16, self.status.bits);
        self.sp -= 1;

        // We look up the value of the interrupt handler we're supposed to execute at `IRQ_POINTER` and set the
        // program counter there.
        let lo = self.read(NMI_POINTER);
        let hi = self.read(NMI_POINTER + 1);
        self.pc = u16::from_be_bytes([lo, hi]);

        // Resets and interrupts actually consume cycles
        self.cycles_remaining = 8;
    }
}