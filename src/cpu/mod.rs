pub mod instructions;

use bitflags::bitflags;
use crate::bus::Bus;

bitflags! {
    pub struct StatusFlags: u8 {
        /// Carry flag
        const C = 1 << 0;
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
    pub fn clock(&self) {

    }

    /// Simulate an interrupt request signal 
    pub fn irq(&self) {

    }

    /// Simulate a non-maskable interrupt request signal
    pub fn nmi(&self) {

    }
}