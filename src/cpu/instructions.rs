use once_cell::sync::Lazy;
use super::{CPU, cpu_addr, cpu_instr};

// Mnemonics for all 6502 CPU instructions
// Ref: http://www.thealmightyguru.com/Games/Hacking/Wiki/index.php/6502_Opcodes
#[derive(Debug)]
pub enum Mnemonic {
    LDA, LDX, LDY, STA, STX, STY, TAX, TAY, TSX, TXA, TXS, TYA,     // Storage
    ADC, DEC, DEX, DEY, INC, INX, INY, SBC,                         // Math
    AND, ASL, BIT, EOR, LSR, ORA, ROL, ROR,                         // Bitwise
    BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS,                         // Branch
    JMP, JSR, RTI, RTS,                                             // Jump
    CLC, CLD, CLI, CLV, CMP, CPX, CPY, SEC, SED, SEI,               // Registers
    PHA, PHP, PLA, PLP,                                             // Stack
    BRK, NOP,                                                       // System
    XXX,
}

// All possible 6502 addressing modes
// Addressing modes define how the CPU fetched the required operands for an instructions
// Ref: http://www.thealmightyguru.com/Games/Hacking/Wiki/index.php?title=Addressing_Modes
#[derive(Debug)]
pub enum AddressingMode {
    ZP0,        // ZeroPage             Operand is an address and only the low byte is used,         ex: LDA $EE
    ZPX,        // Indexed ZeroPage X   Operand is 1-byte address, X register is added to it         eg: STA $00,X
    ZPY,        // Indexed ZeroPage Y   Operand is 1-byte address, Y register is added to it         eg: STA $00,Y
    ABS,        // Absolute             Operand is an address and and both bytes are used,           ex: LDA $16A0
    ABX,        // Indexed Absolute X   Operand is 2-byte address, X register is added to it         eg: STA $1000,X
    ABY,        // Indexed Absolute Y   Operand is 2-byte address, Y register is added to it         eg: STA $1000,Y
    IND,        // Indirect             Memory location is 2-byte pointer at adjacent locations      eg: JMP ($0020)
    IMP,        // Implied              No operands, addressing is implied by the instruction,       eg: TAX
    ACC,        // Accumulator          No operands, accumulator is implied,                         eg: ASL
    IMM,        // Immediate            Operand value is contained in instruction itself,            ex: LDA #$07
    REL,        // Relative             1-byte signed operand is added to the program counter        eg: BEQ $04
    IZX,        // Indexed Indirect     2-byte pointer from 1-byte address and adding X register     eg: LDA ($40, X)
    IZY,        // Indirect Indexed     2-byte pointer from 1-byte address and adding Y after read   eg: LDA ($46), Y
}

pub type OpCode = u8;

pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub op_exec: fn(&mut CPU) -> u8,
    pub mode: AddressingMode,
    pub mode_exec: fn(&mut CPU) -> u8,
    pub cycles: u8,
}

impl Instruction {
    pub fn decode(opcode: OpCode) -> &'static Instruction {
        &INSTRUCTION_MAP[opcode as usize]
    }
}

static INSTRUCTION_MAP: Lazy<[Instruction; 256]> = Lazy::new(|| {[
    Instruction { mnemonic: Mnemonic::BRK, op_exec: cpu_instr::brk, mode: AddressingMode::IMM, mode_exec: cpu_addr::imm, cycles: 7 },
    Instruction { mnemonic: Mnemonic::ORA, op_exec: cpu_instr::ora, mode: AddressingMode::IZX, mode_exec: cpu_addr::izx, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 8 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 3 },
    Instruction { mnemonic: Mnemonic::ORA, op_exec: cpu_instr::ora, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 3 },
    Instruction { mnemonic: Mnemonic::ASL, op_exec: cpu_instr::asl, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 5 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 5 },
    Instruction { mnemonic: Mnemonic::PHP, op_exec: cpu_instr::php, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 3 },
    Instruction { mnemonic: Mnemonic::ORA, op_exec: cpu_instr::ora, mode: AddressingMode::IMM, mode_exec: cpu_addr::imm, cycles: 2 },
    Instruction { mnemonic: Mnemonic::ASL, op_exec: cpu_instr::asl, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::ORA, op_exec: cpu_instr::ora, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 4 },
    Instruction { mnemonic: Mnemonic::ASL, op_exec: cpu_instr::asl, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::BPL, op_exec: cpu_instr::bpl, mode: AddressingMode::REL, mode_exec: cpu_addr::rel, cycles: 2 },
    Instruction { mnemonic: Mnemonic::ORA, op_exec: cpu_instr::ora, mode: AddressingMode::IZY, mode_exec: cpu_addr::izy, cycles: 5 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 8 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::ORA, op_exec: cpu_instr::ora, mode: AddressingMode::ZPX, mode_exec: cpu_addr::zpx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::ASL, op_exec: cpu_instr::asl, mode: AddressingMode::ZPX, mode_exec: cpu_addr::zpx, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::CLC, op_exec: cpu_instr::clc, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::ORA, op_exec: cpu_instr::ora, mode: AddressingMode::ABY, mode_exec: cpu_addr::aby, cycles: 4 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 7 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::ORA, op_exec: cpu_instr::ora, mode: AddressingMode::ABX, mode_exec: cpu_addr::abx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::ASL, op_exec: cpu_instr::asl, mode: AddressingMode::ABX, mode_exec: cpu_addr::abx, cycles: 7 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 7 },
    Instruction { mnemonic: Mnemonic::JSR, op_exec: cpu_instr::jsr, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 6 },
    Instruction { mnemonic: Mnemonic::AND, op_exec: cpu_instr::and, mode: AddressingMode::IZX, mode_exec: cpu_addr::izx, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 8 },
    Instruction { mnemonic: Mnemonic::BIT, op_exec: cpu_instr::bit, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 3 },
    Instruction { mnemonic: Mnemonic::AND, op_exec: cpu_instr::and, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 3 },
    Instruction { mnemonic: Mnemonic::ROL, op_exec: cpu_instr::rol, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 5 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 5 },
    Instruction { mnemonic: Mnemonic::PLP, op_exec: cpu_instr::plp, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::AND, op_exec: cpu_instr::and, mode: AddressingMode::IMM, mode_exec: cpu_addr::imm, cycles: 2 },
    Instruction { mnemonic: Mnemonic::ROL, op_exec: cpu_instr::rol, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::BIT, op_exec: cpu_instr::bit, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 4 },
    Instruction { mnemonic: Mnemonic::AND, op_exec: cpu_instr::and, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 4 },
    Instruction { mnemonic: Mnemonic::ROL, op_exec: cpu_instr::rol, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::BMI, op_exec: cpu_instr::bmi, mode: AddressingMode::REL, mode_exec: cpu_addr::rel, cycles: 2 },
    Instruction { mnemonic: Mnemonic::AND, op_exec: cpu_instr::and, mode: AddressingMode::IZY, mode_exec: cpu_addr::izy, cycles: 5 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 8 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::AND, op_exec: cpu_instr::and, mode: AddressingMode::ZPX, mode_exec: cpu_addr::zpx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::ROL, op_exec: cpu_instr::rol, mode: AddressingMode::ZPX, mode_exec: cpu_addr::zpx, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::SEC, op_exec: cpu_instr::sec, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::AND, op_exec: cpu_instr::and, mode: AddressingMode::ABY, mode_exec: cpu_addr::aby, cycles: 4 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 7 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::AND, op_exec: cpu_instr::and, mode: AddressingMode::ABX, mode_exec: cpu_addr::abx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::ROL, op_exec: cpu_instr::rol, mode: AddressingMode::ABX, mode_exec: cpu_addr::abx, cycles: 7 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 7 },
    Instruction { mnemonic: Mnemonic::RTI, op_exec: cpu_instr::rti, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::EOR, op_exec: cpu_instr::eor, mode: AddressingMode::IZX, mode_exec: cpu_addr::izx, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 8 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 3 },
    Instruction { mnemonic: Mnemonic::EOR, op_exec: cpu_instr::eor, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 3 },
    Instruction { mnemonic: Mnemonic::LSR, op_exec: cpu_instr::lsr, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 5 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 5 },
    Instruction { mnemonic: Mnemonic::PHA, op_exec: cpu_instr::pha, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 3 },
    Instruction { mnemonic: Mnemonic::EOR, op_exec: cpu_instr::eor, mode: AddressingMode::IMM, mode_exec: cpu_addr::imm, cycles: 2 },
    Instruction { mnemonic: Mnemonic::LSR, op_exec: cpu_instr::lsr, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::JMP, op_exec: cpu_instr::jmp, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 3 },
    Instruction { mnemonic: Mnemonic::EOR, op_exec: cpu_instr::eor, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 4 },
    Instruction { mnemonic: Mnemonic::LSR, op_exec: cpu_instr::lsr, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::BVC, op_exec: cpu_instr::bvc, mode: AddressingMode::REL, mode_exec: cpu_addr::rel, cycles: 2 },
    Instruction { mnemonic: Mnemonic::EOR, op_exec: cpu_instr::eor, mode: AddressingMode::IZY, mode_exec: cpu_addr::izy, cycles: 5 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 8 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::EOR, op_exec: cpu_instr::eor, mode: AddressingMode::ZPX, mode_exec: cpu_addr::zpx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::LSR, op_exec: cpu_instr::lsr, mode: AddressingMode::ZPX, mode_exec: cpu_addr::zpx, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::CLI, op_exec: cpu_instr::cli, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::EOR, op_exec: cpu_instr::eor, mode: AddressingMode::ABY, mode_exec: cpu_addr::aby, cycles: 4 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 7 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::EOR, op_exec: cpu_instr::eor, mode: AddressingMode::ABX, mode_exec: cpu_addr::abx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::LSR, op_exec: cpu_instr::lsr, mode: AddressingMode::ABX, mode_exec: cpu_addr::abx, cycles: 7 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 7 },
    Instruction { mnemonic: Mnemonic::RTS, op_exec: cpu_instr::rts, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::ADC, op_exec: cpu_instr::adc, mode: AddressingMode::IZX, mode_exec: cpu_addr::izx, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 8 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 3 },
    Instruction { mnemonic: Mnemonic::ADC, op_exec: cpu_instr::adc, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 3 },
    Instruction { mnemonic: Mnemonic::ROR, op_exec: cpu_instr::ror, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 5 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 5 },
    Instruction { mnemonic: Mnemonic::PLA, op_exec: cpu_instr::pla, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::ADC, op_exec: cpu_instr::adc, mode: AddressingMode::IMM, mode_exec: cpu_addr::imm, cycles: 2 },
    Instruction { mnemonic: Mnemonic::ROR, op_exec: cpu_instr::ror, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::JMP, op_exec: cpu_instr::jmp, mode: AddressingMode::IND, mode_exec: cpu_addr::ind, cycles: 5 },
    Instruction { mnemonic: Mnemonic::ADC, op_exec: cpu_instr::adc, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 4 },
    Instruction { mnemonic: Mnemonic::ROR, op_exec: cpu_instr::ror, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::BVS, op_exec: cpu_instr::bvs, mode: AddressingMode::REL, mode_exec: cpu_addr::rel, cycles: 2 },
    Instruction { mnemonic: Mnemonic::ADC, op_exec: cpu_instr::adc, mode: AddressingMode::IZY, mode_exec: cpu_addr::izy, cycles: 5 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 8 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::ADC, op_exec: cpu_instr::adc, mode: AddressingMode::ZPX, mode_exec: cpu_addr::zpx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::ROR, op_exec: cpu_instr::ror, mode: AddressingMode::ZPX, mode_exec: cpu_addr::zpx, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::SEI, op_exec: cpu_instr::sei, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::ADC, op_exec: cpu_instr::adc, mode: AddressingMode::ABY, mode_exec: cpu_addr::aby, cycles: 4 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 7 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::ADC, op_exec: cpu_instr::adc, mode: AddressingMode::ABX, mode_exec: cpu_addr::abx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::ROR, op_exec: cpu_instr::ror, mode: AddressingMode::ABX, mode_exec: cpu_addr::abx, cycles: 7 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 7 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::STA, op_exec: cpu_instr::sta, mode: AddressingMode::IZX, mode_exec: cpu_addr::izx, cycles: 6 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::STY, op_exec: cpu_instr::sty, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 3 },
    Instruction { mnemonic: Mnemonic::STA, op_exec: cpu_instr::sta, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 3 },
    Instruction { mnemonic: Mnemonic::STX, op_exec: cpu_instr::stx, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 3 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 3 },
    Instruction { mnemonic: Mnemonic::DEY, op_exec: cpu_instr::dey, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::TXA, op_exec: cpu_instr::txa, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::STY, op_exec: cpu_instr::sty, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 4 },
    Instruction { mnemonic: Mnemonic::STA, op_exec: cpu_instr::sta, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 4 },
    Instruction { mnemonic: Mnemonic::STX, op_exec: cpu_instr::stx, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 4 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::BCC, op_exec: cpu_instr::bcc, mode: AddressingMode::REL, mode_exec: cpu_addr::rel, cycles: 2 },
    Instruction { mnemonic: Mnemonic::STA, op_exec: cpu_instr::sta, mode: AddressingMode::IZY, mode_exec: cpu_addr::izy, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::STY, op_exec: cpu_instr::sty, mode: AddressingMode::ZPX, mode_exec: cpu_addr::zpx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::STA, op_exec: cpu_instr::sta, mode: AddressingMode::ZPX, mode_exec: cpu_addr::zpx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::STX, op_exec: cpu_instr::stx, mode: AddressingMode::ZPY, mode_exec: cpu_addr::zpy, cycles: 4 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::TYA, op_exec: cpu_instr::tya, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::STA, op_exec: cpu_instr::sta, mode: AddressingMode::ABY, mode_exec: cpu_addr::aby, cycles: 5 },
    Instruction { mnemonic: Mnemonic::TXS, op_exec: cpu_instr::txs, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 5 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 5 },
    Instruction { mnemonic: Mnemonic::STA, op_exec: cpu_instr::sta, mode: AddressingMode::ABX, mode_exec: cpu_addr::abx, cycles: 5 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 5 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 5 },
    Instruction { mnemonic: Mnemonic::LDY, op_exec: cpu_instr::ldy, mode: AddressingMode::IMM, mode_exec: cpu_addr::imm, cycles: 2 },
    Instruction { mnemonic: Mnemonic::LDA, op_exec: cpu_instr::lda, mode: AddressingMode::IZX, mode_exec: cpu_addr::izx, cycles: 6 },
    Instruction { mnemonic: Mnemonic::LDX, op_exec: cpu_instr::ldx, mode: AddressingMode::IMM, mode_exec: cpu_addr::imm, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::LDY, op_exec: cpu_instr::ldy, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 3 },
    Instruction { mnemonic: Mnemonic::LDA, op_exec: cpu_instr::lda, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 3 },
    Instruction { mnemonic: Mnemonic::LDX, op_exec: cpu_instr::ldx, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 3 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 3 },
    Instruction { mnemonic: Mnemonic::TAY, op_exec: cpu_instr::tay, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::LDA, op_exec: cpu_instr::lda, mode: AddressingMode::IMM, mode_exec: cpu_addr::imm, cycles: 2 },
    Instruction { mnemonic: Mnemonic::TAX, op_exec: cpu_instr::tax, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::LDY, op_exec: cpu_instr::ldy, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 4 },
    Instruction { mnemonic: Mnemonic::LDA, op_exec: cpu_instr::lda, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 4 },
    Instruction { mnemonic: Mnemonic::LDX, op_exec: cpu_instr::ldx, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 4 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::BCS, op_exec: cpu_instr::bcs, mode: AddressingMode::REL, mode_exec: cpu_addr::rel, cycles: 2 },
    Instruction { mnemonic: Mnemonic::LDA, op_exec: cpu_instr::lda, mode: AddressingMode::IZY, mode_exec: cpu_addr::izy, cycles: 5 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 5 },
    Instruction { mnemonic: Mnemonic::LDY, op_exec: cpu_instr::ldy, mode: AddressingMode::ZPX, mode_exec: cpu_addr::zpx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::LDA, op_exec: cpu_instr::lda, mode: AddressingMode::ZPX, mode_exec: cpu_addr::zpx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::LDX, op_exec: cpu_instr::ldx, mode: AddressingMode::ZPY, mode_exec: cpu_addr::zpy, cycles: 4 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::CLV, op_exec: cpu_instr::clv, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::LDA, op_exec: cpu_instr::lda, mode: AddressingMode::ABY, mode_exec: cpu_addr::aby, cycles: 4 },
    Instruction { mnemonic: Mnemonic::TSX, op_exec: cpu_instr::tsx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::LDY, op_exec: cpu_instr::ldy, mode: AddressingMode::ABX, mode_exec: cpu_addr::abx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::LDA, op_exec: cpu_instr::lda, mode: AddressingMode::ABX, mode_exec: cpu_addr::abx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::LDX, op_exec: cpu_instr::ldx, mode: AddressingMode::ABY, mode_exec: cpu_addr::aby, cycles: 4 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::CPY, op_exec: cpu_instr::cpy, mode: AddressingMode::IMM, mode_exec: cpu_addr::imm, cycles: 2 },
    Instruction { mnemonic: Mnemonic::CMP, op_exec: cpu_instr::cmp, mode: AddressingMode::IZX, mode_exec: cpu_addr::izx, cycles: 6 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 8 },
    Instruction { mnemonic: Mnemonic::CPY, op_exec: cpu_instr::cpy, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 3 },
    Instruction { mnemonic: Mnemonic::CMP, op_exec: cpu_instr::cmp, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 3 },
    Instruction { mnemonic: Mnemonic::DEC, op_exec: cpu_instr::dec, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 5 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 5 },
    Instruction { mnemonic: Mnemonic::INY, op_exec: cpu_instr::iny, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::CMP, op_exec: cpu_instr::cmp, mode: AddressingMode::IMM, mode_exec: cpu_addr::imm, cycles: 2 },
    Instruction { mnemonic: Mnemonic::DEX, op_exec: cpu_instr::dex, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::CPY, op_exec: cpu_instr::cpy, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 4 },
    Instruction { mnemonic: Mnemonic::CMP, op_exec: cpu_instr::cmp, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 4 },
    Instruction { mnemonic: Mnemonic::DEC, op_exec: cpu_instr::dec, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::BNE, op_exec: cpu_instr::bne, mode: AddressingMode::REL, mode_exec: cpu_addr::rel, cycles: 2 },
    Instruction { mnemonic: Mnemonic::CMP, op_exec: cpu_instr::cmp, mode: AddressingMode::IZY, mode_exec: cpu_addr::izy, cycles: 5 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 8 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::CMP, op_exec: cpu_instr::cmp, mode: AddressingMode::ZPX, mode_exec: cpu_addr::zpx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::DEC, op_exec: cpu_instr::dec, mode: AddressingMode::ZPX, mode_exec: cpu_addr::zpx, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::CLD, op_exec: cpu_instr::cld, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::CMP, op_exec: cpu_instr::cmp, mode: AddressingMode::ABY, mode_exec: cpu_addr::aby, cycles: 4 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 7 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::CMP, op_exec: cpu_instr::cmp, mode: AddressingMode::ABX, mode_exec: cpu_addr::abx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::DEC, op_exec: cpu_instr::dec, mode: AddressingMode::ABX, mode_exec: cpu_addr::abx, cycles: 7 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 7 },
    Instruction { mnemonic: Mnemonic::CPX, op_exec: cpu_instr::cpx, mode: AddressingMode::IMM, mode_exec: cpu_addr::imm, cycles: 2 },
    Instruction { mnemonic: Mnemonic::SBC, op_exec: cpu_instr::sbc, mode: AddressingMode::IZX, mode_exec: cpu_addr::izx, cycles: 6 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 8 },
    Instruction { mnemonic: Mnemonic::CPX, op_exec: cpu_instr::cpx, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 3 },
    Instruction { mnemonic: Mnemonic::SBC, op_exec: cpu_instr::sbc, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 3 },
    Instruction { mnemonic: Mnemonic::INC, op_exec: cpu_instr::inc, mode: AddressingMode::ZP0, mode_exec: cpu_addr::zp0, cycles: 5 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 5 },
    Instruction { mnemonic: Mnemonic::INX, op_exec: cpu_instr::inx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::SBC, op_exec: cpu_instr::sbc, mode: AddressingMode::IMM, mode_exec: cpu_addr::imm, cycles: 2 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::SBC, op_exec: cpu_instr::sbc, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::CPX, op_exec: cpu_instr::cpx, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 4 },
    Instruction { mnemonic: Mnemonic::SBC, op_exec: cpu_instr::sbc, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 4 },
    Instruction { mnemonic: Mnemonic::INC, op_exec: cpu_instr::inc, mode: AddressingMode::ABS, mode_exec: cpu_addr::abs, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::BEQ, op_exec: cpu_instr::beq, mode: AddressingMode::REL, mode_exec: cpu_addr::rel, cycles: 2 },
    Instruction { mnemonic: Mnemonic::SBC, op_exec: cpu_instr::sbc, mode: AddressingMode::IZY, mode_exec: cpu_addr::izy, cycles: 5 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 8 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::SBC, op_exec: cpu_instr::sbc, mode: AddressingMode::ZPX, mode_exec: cpu_addr::zpx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::INC, op_exec: cpu_instr::inc, mode: AddressingMode::ZPX, mode_exec: cpu_addr::zpx, cycles: 6 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 6 },
    Instruction { mnemonic: Mnemonic::SED, op_exec: cpu_instr::sed, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::SBC, op_exec: cpu_instr::sbc, mode: AddressingMode::ABY, mode_exec: cpu_addr::aby, cycles: 4 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 2 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 7 },
    Instruction { mnemonic: Mnemonic::NOP, op_exec: cpu_instr::nop, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 4 },
    Instruction { mnemonic: Mnemonic::SBC, op_exec: cpu_instr::sbc, mode: AddressingMode::ABX, mode_exec: cpu_addr::abx, cycles: 4 },
    Instruction { mnemonic: Mnemonic::INC, op_exec: cpu_instr::inc, mode: AddressingMode::ABX, mode_exec: cpu_addr::abx, cycles: 7 },
    Instruction { mnemonic: Mnemonic::XXX, op_exec: cpu_instr::xxx, mode: AddressingMode::IMP, mode_exec: cpu_addr::imp, cycles: 7 },
]});
