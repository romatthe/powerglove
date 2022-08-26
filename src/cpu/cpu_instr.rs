use super::{CPU, StatusFlags};

pub fn adc(cpu: &mut CPU) -> u8 {
    0
}

pub fn sbc(cpu: &mut CPU) -> u8 {
    0
}

/// Logical AND on the value in the accumulator.
pub fn and(cpu: &mut CPU) -> u8 {
    cpu.a = cpu.a & cpu.fetch();
    cpu.status.set(StatusFlags::Z, cpu.a == 0x00);
    cpu.status.set(StatusFlags::N, cpu.a & 0b1000_0000 != 0);

    1
}

pub fn asl(cpu: &mut CPU) -> u8 {
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

pub fn bit(cpu: &mut CPU) -> u8 {
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

pub fn brk(cpu: &mut CPU) -> u8 {
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

pub fn clc(cpu: &mut CPU) -> u8 {
    0
}

pub fn cld(cpu: &mut CPU) -> u8 {
    0
}

pub fn cli(cpu: &mut CPU) -> u8 {
    0
}

pub fn clv(cpu: &mut CPU) -> u8 {
    0
}

pub fn cmp(cpu: &mut CPU) -> u8 {
    0
}

pub fn cpx(cpu: &mut CPU) -> u8 {
    0
}

pub fn cpy(cpu: &mut CPU) -> u8 {
    0
}

pub fn dec(cpu: &mut CPU) -> u8 {
    0
}

pub fn dex(cpu: &mut CPU) -> u8 {
    0
}

pub fn dey(cpu: &mut CPU) -> u8 {
    0
}

pub fn eor(cpu: &mut CPU) -> u8 {
    0
}

pub fn inc(cpu: &mut CPU) -> u8 {
    0
}

pub fn inx(cpu: &mut CPU) -> u8 {
    0
}

pub fn iny(cpu: &mut CPU) -> u8 {
    0
}

pub fn jmp(cpu: &mut CPU) -> u8 {
    0
}

pub fn jsr(cpu: &mut CPU) -> u8 {
    0
}

pub fn lda(cpu: &mut CPU) -> u8 {
    0
}

pub fn ldx(cpu: &mut CPU) -> u8 {
    0
}

pub fn ldy(cpu: &mut CPU) -> u8 {
    0
}

pub fn lsr(cpu: &mut CPU) -> u8 {
    0
}

pub fn nop(cpu: &mut CPU) -> u8 {
    0
}

pub fn ora(cpu: &mut CPU) -> u8 {
    0
}

pub fn pha(cpu: &mut CPU) -> u8 {
    0
}

pub fn php(cpu: &mut CPU) -> u8 {
    0
}

pub fn pla(cpu: &mut CPU) -> u8 {
    0
}

pub fn plp(cpu: &mut CPU) -> u8 {
    0
}

pub fn rol(cpu: &mut CPU) -> u8 {
    0
}

pub fn ror(cpu: &mut CPU) -> u8 {
    0
}

pub fn rti(cpu: &mut CPU) -> u8 {
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
