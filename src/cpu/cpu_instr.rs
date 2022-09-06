use std::net::AddrParseError;

use super::{CPU, STACK_BASE, StatusFlags, instructions::{AddressingMode, Instruction}};

/// Add with carry in. Allows us to add a value to the accumulator and a carry bit. 
/// If the result is > 255 there is an overflow setting the carry bit. Ths allows you 
/// to chain together ADC instructions to add numbers larger than 8-bits. 
pub fn adc(cpu: &mut CPU) -> u8 {
    let fetched: u16 = cpu.fetch().into();

    // Add is performed in 16-bit domain for emulation to capture any carry bit, 
    // which will exist in bit 8 of the 16-bit word
    let result = fetched + cpu.a as u16 + cpu.status.contains(StatusFlags::C) as u16;

    // We need to determine the signed overflow flag using the following fomula
    let v = !((cpu.a as u16) ^ fetched) & ((cpu.a as u16) ^ result) & 0x0080;

    // Set all the required
    cpu.status.set(StatusFlags::C, result > 255);
    cpu.status.set(StatusFlags::Z, result & 0x00FF == 0);
    cpu.status.set(StatusFlags::N, result & 0b1000_0000 != 0);
    cpu.status.set(StatusFlags::V, v != 0);

    // Load the result back into the accumulator, but as a u8 of course!
    cpu.a = (result & 0x00FF) as u8;

    1
}

/// Subtraction with Borrow In. Given the explanation for ADC above, we can reorganise our data
/// to use the same computation for addition, for subtraction by multiplying the data by -1, 
/// i.e. make it negative.
pub fn sbc(cpu: &mut CPU) -> u8 {
    // Fetch the datea and invert the lo bits (this is a u8 stored in a u16, so this is all of them) 
    let fetched: u16 = (cpu.fetch() as u16) ^ 0x00FF;

    // Add is performed in 16-bit domain for emulation to capture any carry bit, 
    // which will exist in bit 8 of the 16-bit word
    let result = fetched + cpu.a as u16 + cpu.status.contains(StatusFlags::C) as u16;

    // We need to determine the signed overflow flag using the following fomula
    let v = !((cpu.a as u16) ^ fetched) & ((cpu.a as u16) ^ result) & 0x0080;

    // Set all the required
    cpu.status.set(StatusFlags::C, result > 255);
    cpu.status.set(StatusFlags::Z, result & 0x00FF == 0);
    cpu.status.set(StatusFlags::N, result & 0b1000_0000 != 0);
    cpu.status.set(StatusFlags::V, v != 0);

    // Load the result back into the accumulator, but as a u8 of course!
    cpu.a = (result & 0x00FF) as u8;

    1
}

/// Logical AND on the value in the accumulator.
pub fn and(cpu: &mut CPU) -> u8 {
    cpu.a = cpu.a & cpu.fetch();
    cpu.status.set(StatusFlags::Z, cpu.a == 0x00);
    cpu.status.set(StatusFlags::N, cpu.a & 0b1000_0000 != 0);

    1
}

/// Arithmetic Shift Left.
pub fn asl(cpu: &mut CPU) -> u8 {
    let fetched = cpu.fetch() as u16;
    let shifted = fetched << 1;

    // Set the flags
    cpu.status.set(StatusFlags::C, (shifted & 0xFF00) > 0);
    cpu.status.set(StatusFlags::Z, (shifted & 0x00FF) == 0);
    cpu.status.set(StatusFlags::N, (shifted & 0x80) != 0);

    // Write the result based on the addressing mode
    if Instruction::decode(cpu.opcode).mode == AddressingMode::IMP  {
        cpu.a = (shifted & 0x00FF) as u8;
    } else {
        cpu.write(cpu.addr_abs, (shifted & 0x00FF) as u8);
    }

    0
}

/// Branch if carry bit is clear.
pub fn bcc(cpu: &mut CPU) -> u8 {
    // Check if the carry flag is clear
    if !cpu.status.contains(StatusFlags::C) {
        branch(cpu);
    }

    0
}

/// Branch if the carry bit has been set.
pub fn bcs(cpu: &mut CPU) -> u8 {
    // Check if the carry flag has been set
    if cpu.status.contains(StatusFlags::C) {
        branch(cpu);
    }

    0
}

/// Branch if equal.
pub fn beq(cpu: &mut CPU) -> u8 {
    // Check if the zero flag has been set
    if cpu.status.contains(StatusFlags::Z) {
        branch(cpu);
    }

    0
}

/// Test bits in memory with sccumulator
pub fn bit(cpu: &mut CPU) -> u8 {
    let fetched = cpu.fetch();
	let tested = cpu.a & fetched;

    // Set the flags
    cpu.status.set(StatusFlags::Z, (tested & 0x00FF) == 0);
    cpu.status.set(StatusFlags::N, fetched & (1 << 7) != 0);
    cpu.status.set(StatusFlags::V, fetched & (1 << 6) != 0);

    0
}

/// Branch if negative.
pub fn bmi(cpu: &mut CPU) -> u8 {
    // Check if the negative flag is clear
    if cpu.status.contains(StatusFlags::N) {
        branch(cpu);
    }

    0
}

/// Branch if not equal.
pub fn bne(cpu: &mut CPU) -> u8 {
    // Check if the zero flag is clear
    if !cpu.status.contains(StatusFlags::Z) {
        branch(cpu);
    }

    0
}

/// Branch if positive.
pub fn bpl(cpu: &mut CPU) -> u8 {
    // Check if the negative flag is clear
    if !cpu.status.contains(StatusFlags::N) {
        branch(cpu);
    }

    0
}

/// Break.
pub fn brk(cpu: &mut CPU) -> u8 {
    cpu.pc = cpu.pc.wrapping_add(1);
	
    cpu.status.set(StatusFlags::I, true);
    cpu.write(STACK_BASE + cpu.sp as u16, ((cpu.pc >> 8) & 0x00FF) as u8);
    cpu.write(STACK_BASE + (cpu.sp - 1) as u16, (cpu.pc & 0x00FF) as u8);
    cpu.sp = cpu.sp.wrapping_sub(2);

    // TODO: Why do we toggle the B flag here?
    cpu.status.set(StatusFlags::B, true);
    cpu.write(STACK_BASE + cpu.sp as u16, cpu.status.bits);
    cpu.sp = cpu.sp.wrapping_sub(1);
    cpu.status.set(StatusFlags::B, false);

    let lo = cpu.read(0xFFFE);
    let hi = cpu.read(0xFFFF);

    cpu.pc = u16::from_le_bytes([lo, hi]);

	0
}

/// Branch if overflow.
pub fn bvc(cpu: &mut CPU) -> u8 {
    // Check if the overflow flag is clear
    if !cpu.status.contains(StatusFlags::V) {
        branch(cpu);
    }

    0
}

/// Branch if not overflowed.
pub fn bvs(cpu: &mut CPU) -> u8 {
    // Check if the carry flag has been set
    if cpu.status.contains(StatusFlags::V) {
        branch(cpu);
    }

    0
}

/// Clear the "carry" flag.
pub fn clc(cpu: &mut CPU) -> u8 {
    cpu.status.set(StatusFlags::C, false);

    0
}

/// Clear the "decimal" flag.
pub fn cld(cpu: &mut CPU) -> u8 {
    cpu.status.set(StatusFlags::D, false);

    0
}

/// Clear the "disable interrupt" flag.
pub fn cli(cpu: &mut CPU) -> u8 {
    cpu.status.set(StatusFlags::I, false);

    0
}

/// Clear the "overflow" flag.
pub fn clv(cpu: &mut CPU) -> u8 {
    cpu.status.set(StatusFlags::V, false);

    0
}

/// Compare Accumulator.
pub fn cmp(cpu: &mut CPU) -> u8 {
    let fetched = cpu.fetch();
    let compared = (cpu.a as u16).wrapping_sub(fetched as u16);

    // Set flags
    cpu.status.set(StatusFlags::C, cpu.a >= fetched);
    cpu.status.set(StatusFlags::N, (compared & 0x0080) != 0);
    cpu.status.set(StatusFlags::Z, (compared & 0x00FF) == 0);

    1
}

/// Compare X Register
pub fn cpx(cpu: &mut CPU) -> u8 {
    let fetched = cpu.fetch();
    let compared = (cpu.x as u16).wrapping_sub(fetched as u16);

    // Set flags
    cpu.status.set(StatusFlags::C, cpu.a >= fetched);
    cpu.status.set(StatusFlags::N, (compared & 0x0080) != 0);
    cpu.status.set(StatusFlags::Z, (compared & 0x00FF) == 0);

    0
}

/// Compare X Register
pub fn cpy(cpu: &mut CPU) -> u8 {
    let fetched = cpu.fetch();
    let compared = (cpu.y as u16).wrapping_sub(fetched as u16);

    // Set flags
    cpu.status.set(StatusFlags::C, cpu.a >= fetched);
    cpu.status.set(StatusFlags::N, (compared & 0x0080) != 0);
    cpu.status.set(StatusFlags::Z, (compared & 0x00FF) == 0);

    0
}

/// Decrement value at memory location.
pub fn dec(cpu: &mut CPU) -> u8 {
    let fetched = cpu.fetch();
    let decrement = fetched.wrapping_sub(1);
    cpu.write(cpu.addr_abs, decrement & 0x00FF);

    // Set flags
    cpu.status.set(StatusFlags::N, (decrement & 0x0080) != 0);
    cpu.status.set(StatusFlags::Z, (decrement & 0x00FF) == 0);

    0
}

/// Decrement X register.
pub fn dex(cpu: &mut CPU) -> u8 {
    cpu.x = cpu.x.wrapping_sub(cpu.x);

    // Set flags
    cpu.status.set(StatusFlags::N, (cpu.x & 0x0080) != 0);
    cpu.status.set(StatusFlags::Z, cpu.x == 0);

    0
}

/// Decrement Y register.
pub fn dey(cpu: &mut CPU) -> u8 {
    cpu.y = cpu.y.wrapping_sub(cpu.y);

    // Set flags
    cpu.status.set(StatusFlags::N, (cpu.y & 0x0080) != 0);
    cpu.status.set(StatusFlags::Z, cpu.y == 0);
    
    0
}

/// Bitwise logic XOR.
pub fn eor(cpu: &mut CPU) -> u8 {
    let fetched = cpu.fetch();
    cpu.a = cpu.a ^ fetched;
    
    // Set flags
    cpu.status.set(StatusFlags::N, cpu.a & 0x80 != 0);
    cpu.status.set(StatusFlags::Z, cpu.a == 0);
    
    1
}

/// Increment Value at memory location.
pub fn inc(cpu: &mut CPU) -> u8 {
    let fetched = cpu.fetch();
    let increment = fetched + 1;
    cpu.write(cpu.addr_abs, increment & 0x00FF);

    // Set flags
    cpu.status.set(StatusFlags::N, (increment & 0x0080) != 0);
    cpu.status.set(StatusFlags::Z, (increment & 0x00FF) == 0);

    0
}

/// Increment X Register.
pub fn inx(cpu: &mut CPU) -> u8 {
    cpu.x = cpu.x.wrapping_add(1);
    cpu.status.set(StatusFlags::N, (cpu.x & 0x0080) != 0);
    cpu.status.set(StatusFlags::Z, cpu.x == 0);

    0
}

/// Increment Y Register.
pub fn iny(cpu: &mut CPU) -> u8 {
    cpu.y = cpu.y.wrapping_add(1);
    cpu.status.set(StatusFlags::N, (cpu.y & 0x0080) != 0);
    cpu.status.set(StatusFlags::Z, cpu.y == 0);

    0
}

/// Jump to location.
pub fn jmp(cpu: &mut CPU) -> u8 {
    cpu.pc = cpu.addr_abs;
    
    0
}

/// Jump to subroutine.
pub fn jsr(cpu: &mut CPU) -> u8 {
    cpu.pc = cpu.pc.wrapping_sub(1);
    cpu.write(STACK_BASE + cpu.sp as u16, ((cpu.pc >> 8) & 0x00FF) as u8);
    cpu.write(STACK_BASE + (cpu.sp - 1) as u16, (cpu.pc & 0x00FF) as u8);
    cpu.sp = cpu.sp.wrapping_sub(2);
    cpu.pc = cpu.addr_abs;

    0
}

/// Load the accumulator.
pub fn lda(cpu: &mut CPU) -> u8 {
    cpu.a = cpu.fetch();
    
    // Set flags
    cpu.status.set(StatusFlags::N, cpu.a & 0x80 != 0);
    cpu.status.set(StatusFlags::Z, cpu.a == 0);

    1
}

/// Load the X register.
pub fn ldx(cpu: &mut CPU) -> u8 {
    cpu.x = cpu.fetch();
    
    // Set flags
    cpu.status.set(StatusFlags::N, cpu.x & 0x80 != 0);
    cpu.status.set(StatusFlags::Z, cpu.x == 0);

    1
}

/// Load the Y register.
pub fn ldy(cpu: &mut CPU) -> u8 {
    cpu.y = cpu.fetch();
    
    // Set flags
    cpu.status.set(StatusFlags::N, cpu.y & 0x80 != 0);
    cpu.status.set(StatusFlags::Z, cpu.y == 0);

    1
}

/// Shift one bit right (memory or accumulator).
pub fn lsr(cpu: &mut CPU) -> u8 {
    let fetched = cpu.fetch();
    let shifted = fetched >> 1 as u16;

    // Set flags
    cpu.status.set(StatusFlags::C, fetched & 0x0001 != 0);
    cpu.status.set(StatusFlags::N, (shifted & 0x0080) != 0);
    cpu.status.set(StatusFlags::Z, (shifted & 0x00FF) == 0);

    // Write the result based on the addressing mode
    if Instruction::decode(cpu.opcode).mode == AddressingMode::IMP {
        cpu.a = shifted & 0x00FF;
    } else {
        cpu.write(cpu.addr_abs, shifted & 0x00FF);
    }

    0
}

/// No operation.
pub fn nop(cpu: &mut CPU) -> u8 {
    // Not all NOPs are actually the same, see 
    // https://wiki.nesdev.com/w/index.php/CPU_unofficial_opcodes
    match cpu.opcode {
        0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => 1,
        _ => 0,
    }
}

/// Bitwise logic OR.
pub fn ora(cpu: &mut CPU) -> u8 {
    let fetched = cpu.fetch();
    cpu.a = cpu.a | fetched;

    // Set flags
    cpu.status.set(StatusFlags::Z, cpu.a == 0);
    cpu.status.set(StatusFlags::N, cpu.a & 0x80 != 0);

    1
}

/// Push Accumulator to Stack.
pub fn pha(cpu: &mut CPU) -> u8 {
    cpu.write(STACK_BASE + cpu.sp as u16, cpu.a);
    cpu.sp -= 1;

    0
}

/// Push status register to stack.
pub fn php(cpu: &mut CPU) -> u8 {
    // TODO: Does the status flag manipulation here work?
    cpu.write(STACK_BASE + cpu.sp as u16, cpu.status.bits | StatusFlags::B.bits | StatusFlags::U.bits);
    cpu.sp = cpu.sp.wrapping_sub(1);

    // Set flags
    cpu.status.set(StatusFlags::B, false);
    cpu.status.set(StatusFlags::U, false);

    0
}

/// Pop Accumulator off Stack.
pub fn pla(cpu: &mut CPU) -> u8 {
    cpu.sp += 1;
    cpu.a = cpu.read(STACK_BASE + cpu.sp as u16);

    // Set flags
    cpu.status.set(StatusFlags::Z, cpu.a == 0);
    cpu.status.set(StatusFlags::N, cpu.a & 0x80 != 0);

    0
}

/// Pop status register off stack.
pub fn plp(cpu: &mut CPU) -> u8 {
    cpu.sp = cpu.sp.wrapping_add(1);
    cpu.status.bits = cpu.read(STACK_BASE + cpu.sp as u16);
    
    // Set flags
    cpu.status.set(StatusFlags::U, true);

    0
}

/// Rotate one bit left (memory or accumulator).
pub fn rol(cpu: &mut CPU) -> u8 {
    let fetched: u16 = cpu.fetch().into();
    let rotated: u16 = (fetched << 1) | (cpu.status.contains(StatusFlags::C) as u16);

    // Set flags
    cpu.status.set(StatusFlags::C, (rotated & 0xFF00) != 0);
    cpu.status.set(StatusFlags::N, (rotated & 0x0080) != 0);
    cpu.status.set(StatusFlags::Z, (rotated & 0x00FF) == 0);

    // Write the result based on the addressing mode
    if Instruction::decode(cpu.opcode).mode == AddressingMode::IMP {
        cpu.a = (rotated & 0x00FF) as u8;
    } else {
        cpu.write(cpu.addr_abs, (rotated & 0x00FF) as u8);
    }

    0
}

/// Rotate one bit right (memory or accumulator).
pub fn ror(cpu: &mut CPU) -> u8 {
    let fetched: u16 = cpu.fetch().into();
    let rotated: u16 = ((cpu.status.contains(StatusFlags::C) as u16) << 7) | (fetched >> 1);

    // Set flags
    cpu.status.set(StatusFlags::C, (fetched & 0x0001) != 0);
    cpu.status.set(StatusFlags::N, (rotated & 0x0080) != 0);
    cpu.status.set(StatusFlags::Z, (rotated & 0x00FF) == 0);

    // Write the result based on the addressing mode
    if Instruction::decode(cpu.opcode).mode == AddressingMode::IMP {
        cpu.a = (rotated & 0x00FF) as u8;
    } else {
        cpu.write(cpu.addr_abs, (rotated & 0x00FF) as u8);
    }

    0
}

/// Returns from a BRK, IRQ or NMI.
pub fn rti(cpu: &mut CPU) -> u8 {
    // Restore the status register value from the stack
    let status_bits = cpu.read(STACK_BASE + cpu.sp as u16 + 1);
    cpu.status = StatusFlags::from_bits(status_bits).unwrap();
    cpu.status.set(StatusFlags::B, false);
    cpu.status.set(StatusFlags::U, false);
    cpu.sp += 1;

    let hi = cpu.read(STACK_BASE + cpu.sp as u16 + 1); 
    let lo = cpu.read(STACK_BASE + cpu.sp as u16 + 2);
    cpu.pc = u16::from_be_bytes([lo, hi]);
    cpu.sp += 2;

    0
}

pub fn rts(cpu: &mut CPU) -> u8 {
    0
}

pub fn sec(cpu: &mut CPU) -> u8 {
    0
}

pub fn sed(cpu: &mut CPU) -> u8 {
    0
}

pub fn sei(cpu: &mut CPU) -> u8 {
    0
}

pub fn sta(cpu: &mut CPU) -> u8 {
    0
}

pub fn stx(cpu: &mut CPU) -> u8 {
    0
}

pub fn sty(cpu: &mut CPU) -> u8 {
    0
}

pub fn tax(cpu: &mut CPU) -> u8 {
    0
}

pub fn tay(cpu: &mut CPU) -> u8 {
    0
}

pub fn tsx(cpu: &mut CPU) -> u8 {
    0
}

pub fn txa(cpu: &mut CPU) -> u8 {
    0
}

pub fn tya(cpu: &mut CPU) -> u8 {
    0
}

pub fn txs(cpu: &mut CPU) -> u8 {
    0
}

pub fn xxx(cpu: &mut CPU) -> u8 {
    0
}

/// Generic branch instruction
fn branch(cpu: &mut CPU) {
    cpu.cycles_remaining += 1;
    cpu.addr_abs = cpu.pc + cpu.addr_rel;

    // If this instruction crossed the page boundary, we
    // need to perform an additional clock cycle
    if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
        cpu.cycles_remaining += 1;
    }

    cpu.pc = cpu.addr_abs;
}
