use super::CPU;

/// Implied addressiong. No data is fetched with this addressing mode as it
/// is part of the actual instruction instead. Some implied instruction act
/// upon the accumulator value though, so we set `fetched` to that value.
#[inline]
pub fn imp(cpu: &mut CPU) -> u8 {
    cpu.fetched = cpu.a;
    0
}

/// Immediate mode addressing. This means the data is supplied as part of the
/// instruction (in other words, the next byte).
#[inline]
pub fn imm(cpu: &mut CPU) -> u8 {
    cpu.addr_abs = cpu.pc;
    cpu.pc = cpu.pc.wrapping_add(1);
    0
}

/// Zero page addressing. This means we're reading from page zero (address with high
/// byte being 0x00). In the 6502 world, working memory is often located at or around
/// page zero. Thus we can interact with working memory with instructions that require
/// less bytes (in other words, shorter instructions).
#[inline]
pub fn zp0(cpu: &mut CPU) -> u8 {
    cpu.addr_abs = cpu.read(cpu.pc).into();
    cpu.addr_abs = cpu.addr_abs & 0x00FF;
    cpu.pc = cpu.pc.wrapping_add(1);
    0
}

/// Zero page addressing with the offset of the X register added to it. Useful for iterating
/// through regions of working memory.
#[inline]
pub fn zpx(cpu: &mut CPU) -> u8 {
    cpu.addr_abs = (cpu.read(cpu.pc) + cpu.x).into();
    cpu.addr_abs = cpu.addr_abs & 0x00FF;
    cpu.pc = cpu.pc.wrapping_add(1);
    0
}

/// Zero page addressing with the offset of the Y register added to it. Useful for iterating
/// through regions of working memory.
#[inline]
pub fn zpy(cpu: &mut CPU) -> u8 {
    cpu.addr_abs = (cpu.read(cpu.pc) + cpu.y).into();
    cpu.addr_abs = cpu.addr_abs & 0x00FF;
    cpu.pc = cpu.pc.wrapping_add(1);
    0
}

/// Relative addressing. Only used in branching instructions.
#[inline]
pub fn rel(cpu: &mut CPU) -> u8 {
    cpu.addr_rel = cpu.read(cpu.pc).into();
    cpu.pc = cpu.pc.wrapping_add(1);

    // In order to use this address to jump back, it needs to be signed. Therefore,
    // we check if the first bit is set to 1.
    if cpu.addr_rel & 0b1000_0000 != 0 {
        // Set the high byte of the relative address to all 1s.
        cpu.addr_rel |= 0xFF00;
    }

    0
}

/// Absolute addressing. The entire address we need is located in the next two bytes from the
/// instruction.
#[inline]
pub fn abs(cpu: &mut CPU) -> u8 {
    let lo = cpu.read(cpu.pc);
    let hi = cpu.read(cpu.pc.wrapping_add(1));
    
    cpu.pc = cpu.pc.wrapping_add(2);
    cpu.addr_abs = u16::from_le_bytes([lo, hi]);

    0
}

/// Absolute addressing with the offset in the X register added to it. An extra cycle must be
/// elapsed if during the adding of the X register, a page is crossed.
#[inline]
pub fn abx(cpu: &mut CPU) -> u8 {
    let lo = cpu.read(cpu.pc);
    let hi = cpu.read(cpu.pc.wrapping_add(1));
    
    cpu.pc = cpu.pc.wrapping_add(2);
    cpu.addr_abs = u16::from_le_bytes([lo, hi]);
    cpu.addr_abs = cpu.addr_abs + cpu.x as u16;

    // If by incrementing the absolute address with the X register the whole
    // address has changed to a different page, we need to count an extra cycle.
    // We can do this by checking if the high byte has changed.
    if (cpu.addr_abs & 0xFF00) != (hi as u16) << 8 {
        1
    } else {
        0
    }
}

/// Absolute addressing with the offset in the Y register added to it. An extra cycle must be
/// elapsed if during the adding of the Y register, a page is crossed.
#[inline]
pub fn aby(cpu: &mut CPU) -> u8 {
    let lo = cpu.read(cpu.pc);
    let hi = cpu.read(cpu.pc.wrapping_add(1));
    
    cpu.pc = cpu.pc.wrapping_add(2);
    cpu.addr_abs = u16::from_le_bytes([lo, hi]);
    cpu.addr_abs = cpu.addr_abs + cpu.y as u16;

    // If by incrementing the absolute address with the Y register the whole
    // address has changed to a different page, we need to count an extra cycle.
    // We can do this by checking if the high byte has changed.
    if (cpu.addr_abs & 0xFF00) != (hi as u16) << 8 {
        1
    } else {
        0
    }
}

/// Indirect addressing. This is an assembly-level technique to implement pointer-like addressing, as
/// this reads from the address defined by the the value read through absolute addressing.
#[inline]
pub fn ind(cpu: &mut CPU) -> u8 {
    // First construct the "pointer"
    let ptr_lo = cpu.read(cpu.pc);
    let ptr_hi = cpu.read(cpu.pc.wrapping_add(1));
    let ptr = u16::from_le_bytes([ptr_lo, ptr_hi]);
    cpu.pc = cpu.pc.wrapping_add(2);

    // This simulates a hardware bug. If the lo byte is 0x00FF (aka, a page cross will occur), the 6502 
    // will not read from the correct memory location by reading from `ptr` twice, instead of 
    // `ptr + 1` and `ptr`. 
    if ptr_lo == 0x00FF {
        cpu.addr_abs = u16::from_le_bytes([cpu.read(ptr & 0xFF00), cpu.read(ptr + 1)]);
    } else {
        cpu.addr_abs = u16::from_le_bytes([cpu.read(ptr), cpu.read(ptr + 1)]);
    }

    0
}

/// Indirect addressing of the zero page with X offset.
#[inline]
pub fn izx(cpu: &mut CPU) -> u8 {
    let t: u16 = cpu.read(cpu.pc).into();
    let lo = cpu.read((t + cpu.x as u16) as u16 & 0x00FF);
    let hi = cpu.read((t + cpu.x as u16 + 1) as u16 & 0x00FF);
    
    cpu.pc = cpu.pc.wrapping_add(1);
    cpu.addr_abs = u16::from_be_bytes([lo, hi]);
    
    0
}


/// Indirect addressing of the zero page with Y offset after reading.
#[inline]
pub fn izy(cpu: &mut CPU) -> u8 {
    let t: u16 = cpu.read(cpu.pc).into();
    let lo = cpu.read(t & 0x00FF);
    let hi = cpu.read((t + 1)  & 0x00FF);
    
    cpu.pc = cpu.pc.wrapping_add(1);
    cpu.addr_abs = u16::from_le_bytes([lo, hi]);
    cpu.addr_abs = cpu.addr_abs + cpu.y as u16;

    // If by incrementing the indirect with the Y register the whole
    // address has changed to a different page, we need to count an extra cycle.
    // We can do this by checking if the high byte has changed.

    if (cpu.addr_abs & 0xFF00) != (hi as u16) << 8 {
        1
    } else {
        0
    }
}