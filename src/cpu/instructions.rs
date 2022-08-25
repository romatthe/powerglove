use std::collections::HashMap;
use once_cell::sync::Lazy;

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
    UNKNOWN
}

// All possible 6502 addressing modes
// Addressing modes define how the CPU fetched the required operands for an instructions
// Ref: http://www.thealmightyguru.com/Games/Hacking/Wiki/index.php?title=Addressing_Modes
#[derive(Debug)]
pub enum AddressingMode {
    ZPG,        // ZeroPage             Operand is an address and only the low byte is used,         ex: LDA $EE
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
    IDX,        // Indexed Indirect     2-byte pointer from 1-byte address and adding X register     eg: LDA ($40, X)
    IDY,        // Indirect Indexed     2-byte pointer from 1-byte address and adding Y after read   eg: LDA ($46), Y
    UNKNOWN
}

pub type OpCode = u8;

#[derive(Debug)]
pub struct Instruction {
    pub opcode: OpCode,
    pub mnemonic: Mnemonic,
    pub mode: AddressingMode,
    pub length: u8,
    pub cycles: u8
}

impl Instruction {
    fn new(opcode: OpCode, mnemonic: Mnemonic, mode: AddressingMode, length: u8, cycles: u8) -> Self {
        Instruction { opcode, mnemonic, mode, length, cycles }
    }

    pub fn decode(opcode: OpCode) -> &'static Instruction {
        if let Some(instr) = INSTRUCTION_MAP.get(&opcode) {
            return instr
        }

        &INSTRUCTION_ILLEGAL
    }
}

static INSTRUCTION_ILLEGAL: Lazy<Instruction> = Lazy::new(|| {
    Instruction::new(0xff, Mnemonic::UNKNOWN, AddressingMode::UNKNOWN, 0, 0)
});

static INSTRUCTION_MAP: Lazy<HashMap<OpCode, Instruction>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // LDA
    m.insert(0xa9, Instruction::new(0xa9, Mnemonic::LDA, AddressingMode::IMM, 2, 2));
    m.insert(0xa5, Instruction::new(0xa5, Mnemonic::LDA, AddressingMode::ZPG, 2, 3));
    m.insert(0xb5, Instruction::new(0xb5, Mnemonic::LDA, AddressingMode::ZPX, 2, 4));
    m.insert(0xad, Instruction::new(0xad, Mnemonic::LDA, AddressingMode::ABS, 3, 4));
    m.insert(0xbd, Instruction::new(0xbd, Mnemonic::LDA, AddressingMode::ABX, 3, 4));
    m.insert(0xb9, Instruction::new(0xb9, Mnemonic::LDA, AddressingMode::ABY, 3, 4));
    m.insert(0xa1, Instruction::new(0xa1, Mnemonic::LDA, AddressingMode::IDX, 2, 6));
    m.insert(0xb1, Instruction::new(0xb1, Mnemonic::LDA, AddressingMode::IDY, 2, 5));
    // LDX
    m.insert(0xa2, Instruction::new(0xa2, Mnemonic::LDX, AddressingMode::IMM, 2, 2));
    m.insert(0xa6, Instruction::new(0xa6, Mnemonic::LDX, AddressingMode::ZPG, 2, 3));
    m.insert(0xb6, Instruction::new(0xb6, Mnemonic::LDX, AddressingMode::ZPY, 2, 4));
    m.insert(0xae, Instruction::new(0xae, Mnemonic::LDX, AddressingMode::ABS, 3, 4));
    m.insert(0xbe, Instruction::new(0xbe, Mnemonic::LDX, AddressingMode::ABY, 3, 4));
    // LDY
    m.insert(0xa0, Instruction::new(0xa0, Mnemonic::LDY, AddressingMode::IMM, 2, 2));
    m.insert(0xa4, Instruction::new(0xa4, Mnemonic::LDY, AddressingMode::ZPG, 2, 3));
    m.insert(0xb4, Instruction::new(0xb4, Mnemonic::LDY, AddressingMode::ZPX, 2, 4));
    m.insert(0xac, Instruction::new(0xac, Mnemonic::LDY, AddressingMode::ABS, 3, 4));
    m.insert(0xbc, Instruction::new(0xbc, Mnemonic::LDY, AddressingMode::ABX, 3, 4));
    // STA
    m.insert(0x85, Instruction::new(0x85, Mnemonic::STA, AddressingMode::ZPG, 2, 3));
    m.insert(0x95, Instruction::new(0x95, Mnemonic::STA, AddressingMode::ZPX, 2, 4));
    m.insert(0x8d, Instruction::new(0x8d, Mnemonic::STA, AddressingMode::ABS, 3, 4));
    m.insert(0x9d, Instruction::new(0x9d, Mnemonic::STA, AddressingMode::ABX, 3, 5));
    m.insert(0x99, Instruction::new(0x99, Mnemonic::STA, AddressingMode::ABY, 3, 5));
    m.insert(0x81, Instruction::new(0x81, Mnemonic::STA, AddressingMode::IDX, 2, 6));
    m.insert(0x91, Instruction::new(0x91, Mnemonic::STA, AddressingMode::IDY, 2, 6));
    // STX
    m.insert(0x86, Instruction::new(0x86, Mnemonic::STX, AddressingMode::ZPG, 2, 3));
    m.insert(0x96, Instruction::new(0x96, Mnemonic::STX, AddressingMode::ZPY, 2, 4));
    m.insert(0x8e, Instruction::new(0x8e, Mnemonic::STX, AddressingMode::ABS, 3, 4));
    // STY
    m.insert(0x84, Instruction::new(0x84, Mnemonic::STY, AddressingMode::ZPG, 2, 3));
    m.insert(0x94, Instruction::new(0x94, Mnemonic::STY, AddressingMode::ZPX, 2, 4));
    m.insert(0x8c, Instruction::new(0x8c, Mnemonic::STY, AddressingMode::ABS, 3, 4));
    // TAX
    m.insert(0xaa, Instruction::new(0xaa, Mnemonic::TAX, AddressingMode::IMP, 1, 2));
    // TAY
    m.insert(0xa8, Instruction::new(0xa8, Mnemonic::TAY, AddressingMode::IMP, 1, 2));
    // TSX
    m.insert(0xba, Instruction::new(0xba, Mnemonic::TSX, AddressingMode::IMP, 1, 2));
    // TXA
    m.insert(0x8a, Instruction::new(0x8a, Mnemonic::TXA, AddressingMode::IMP, 1, 2));
    // TXS
    m.insert(0x9a, Instruction::new(0x9a, Mnemonic::TXS, AddressingMode::IMP, 1, 2));
    // TYA
    m.insert(0x98, Instruction::new(0x98, Mnemonic::TYA, AddressingMode::IMP, 1, 2));
    // ADC
    m.insert(0x69, Instruction::new(0x69, Mnemonic::ADC, AddressingMode::IMM, 2, 2));
    m.insert(0x65, Instruction::new(0x65, Mnemonic::ADC, AddressingMode::ZPG, 2, 3));
    m.insert(0x75, Instruction::new(0x75, Mnemonic::ADC, AddressingMode::ZPX, 2, 4));
    m.insert(0x6d, Instruction::new(0x6d, Mnemonic::ADC, AddressingMode::ABS, 3, 4));
    m.insert(0x7d, Instruction::new(0x7d, Mnemonic::ADC, AddressingMode::ABX, 3, 4));
    m.insert(0x79, Instruction::new(0x79, Mnemonic::ADC, AddressingMode::ABY, 3, 4));
    m.insert(0x61, Instruction::new(0x61, Mnemonic::ADC, AddressingMode::IDX, 2, 6));
    m.insert(0x71, Instruction::new(0x71, Mnemonic::ADC, AddressingMode::IDY, 2, 5));
    // DEC
    m.insert(0xc6, Instruction::new(0xc6, Mnemonic::DEC, AddressingMode::ZPG, 2, 5));
    m.insert(0xd6, Instruction::new(0xd6, Mnemonic::DEC, AddressingMode::ZPX, 2, 6));
    m.insert(0xce, Instruction::new(0xce, Mnemonic::DEC, AddressingMode::ABS, 3, 6));
    m.insert(0xde, Instruction::new(0xde, Mnemonic::DEC, AddressingMode::ABX, 3, 7));
    // DEX
    m.insert(0xca, Instruction::new(0xca, Mnemonic::DEX, AddressingMode::IMP, 1, 2));
    // DEY
    m.insert(0x88, Instruction::new(0x88, Mnemonic::DEY, AddressingMode::IMP, 1, 2));
    // INC
    m.insert(0xe6, Instruction::new(0xe6, Mnemonic::INC, AddressingMode::ZPG, 2, 5));
    m.insert(0xf6, Instruction::new(0xf6, Mnemonic::INC, AddressingMode::ZPX, 2, 6));
    m.insert(0xee, Instruction::new(0xee, Mnemonic::INC, AddressingMode::ABS, 3, 6));
    m.insert(0xfe, Instruction::new(0xfe, Mnemonic::INC, AddressingMode::ABX, 3, 7));
    // INX
    m.insert(0xe8, Instruction::new(0xe8, Mnemonic::INX, AddressingMode::IMP, 1, 2));
    // INY
    m.insert(0xc8, Instruction::new(0xc8, Mnemonic::INY, AddressingMode::IMP, 1, 2));
    // SBC
    m.insert(0xe9, Instruction::new(0xe9, Mnemonic::SBC, AddressingMode::IMM, 2, 2));
    m.insert(0xe5, Instruction::new(0xe5, Mnemonic::SBC, AddressingMode::ZPG, 2, 3));
    m.insert(0xf5, Instruction::new(0xf5, Mnemonic::SBC, AddressingMode::ZPX, 2, 4));
    m.insert(0xed, Instruction::new(0xed, Mnemonic::SBC, AddressingMode::ABS, 3, 5));
    m.insert(0xfd, Instruction::new(0xfd, Mnemonic::SBC, AddressingMode::ABX, 3, 5));
    m.insert(0xf9, Instruction::new(0xf9, Mnemonic::SBC, AddressingMode::ABY, 3, 5));
    m.insert(0xe1, Instruction::new(0xe1, Mnemonic::SBC, AddressingMode::IDX, 2, 6));
    m.insert(0xf1, Instruction::new(0xf1, Mnemonic::SBC, AddressingMode::IDY, 2, 5));
    // AND
    m.insert(0x29, Instruction::new(0x29, Mnemonic::AND, AddressingMode::IMM, 2, 2));
    m.insert(0x25, Instruction::new(0x25, Mnemonic::AND, AddressingMode::ZPG, 2, 3));
    m.insert(0x35, Instruction::new(0x35, Mnemonic::AND, AddressingMode::ZPX, 2, 4));
    m.insert(0x2d, Instruction::new(0x2d, Mnemonic::AND, AddressingMode::ABS, 3, 4));
    m.insert(0x3d, Instruction::new(0x3d, Mnemonic::AND, AddressingMode::ABX, 3, 4));
    m.insert(0x39, Instruction::new(0x39, Mnemonic::AND, AddressingMode::ABY, 3, 4));
    m.insert(0x21, Instruction::new(0x21, Mnemonic::AND, AddressingMode::IDX, 2, 6));
    m.insert(0x31, Instruction::new(0x31, Mnemonic::AND, AddressingMode::IDY, 2, 5));
    // ASL
    m.insert(0x0a, Instruction::new(0x0a, Mnemonic::ASL, AddressingMode::ACC, 1, 2));
    m.insert(0x06, Instruction::new(0x06, Mnemonic::ASL, AddressingMode::ZPG, 2, 5));
    m.insert(0x16, Instruction::new(0x16, Mnemonic::ASL, AddressingMode::ZPX, 2, 6));
    m.insert(0x0e, Instruction::new(0x0e, Mnemonic::ASL, AddressingMode::ABS, 3, 6));
    m.insert(0x1e, Instruction::new(0x1e, Mnemonic::ASL, AddressingMode::ABX, 3, 7));
    // BIT
    m.insert(0x24, Instruction::new(0x24, Mnemonic::BIT, AddressingMode::ZPG, 2, 3));
    m.insert(0x2c, Instruction::new(0x2c, Mnemonic::BIT, AddressingMode::ABS, 3, 4));
    // EOR
    m.insert(0x49, Instruction::new(0x49, Mnemonic::EOR, AddressingMode::IMM, 2, 2));
    m.insert(0x45, Instruction::new(0x45, Mnemonic::EOR, AddressingMode::ZPG, 2, 3));
    m.insert(0x55, Instruction::new(0x55, Mnemonic::EOR, AddressingMode::ZPX, 2, 4));
    m.insert(0x4d, Instruction::new(0x4d, Mnemonic::EOR, AddressingMode::ABS, 3, 4));
    m.insert(0x5d, Instruction::new(0x5d, Mnemonic::EOR, AddressingMode::ABX, 3, 4));
    m.insert(0x59, Instruction::new(0x59, Mnemonic::EOR, AddressingMode::ABY, 3, 4));
    m.insert(0x41, Instruction::new(0x41, Mnemonic::EOR, AddressingMode::IDX, 2, 6));
    m.insert(0x51, Instruction::new(0x51, Mnemonic::EOR, AddressingMode::IDY, 2, 5));
    // LSR
    m.insert(0x4a, Instruction::new(0x4a, Mnemonic::LSR, AddressingMode::ACC, 1, 2));
    m.insert(0x46, Instruction::new(0x46, Mnemonic::LSR, AddressingMode::ZPG, 2, 5));
    m.insert(0x56, Instruction::new(0x56, Mnemonic::LSR, AddressingMode::ZPX, 2, 6));
    m.insert(0x4e, Instruction::new(0x4e, Mnemonic::LSR, AddressingMode::ABS, 3, 6));
    m.insert(0x5e, Instruction::new(0x5e, Mnemonic::LSR, AddressingMode::ABX, 3, 7));
    // ORA
    m.insert(0x09, Instruction::new(0x09, Mnemonic::ORA, AddressingMode::IMM, 2, 2));
    m.insert(0x05, Instruction::new(0x05, Mnemonic::ORA, AddressingMode::ZPG, 2, 3));
    m.insert(0x15, Instruction::new(0x15, Mnemonic::ORA, AddressingMode::ZPX, 2, 4));
    m.insert(0x0d, Instruction::new(0x0d, Mnemonic::ORA, AddressingMode::ABS, 3, 4));
    m.insert(0x1d, Instruction::new(0x1d, Mnemonic::ORA, AddressingMode::ABX, 3, 4));
    m.insert(0x19, Instruction::new(0x19, Mnemonic::ORA, AddressingMode::ABY, 3, 4));
    m.insert(0x01, Instruction::new(0x01, Mnemonic::ORA, AddressingMode::IDX, 2, 6));
    m.insert(0x11, Instruction::new(0x11, Mnemonic::ORA, AddressingMode::IDY, 2, 5));
    // ROL
    m.insert(0x2a, Instruction::new(0x2a, Mnemonic::ROL, AddressingMode::ACC, 1, 2));
    m.insert(0x26, Instruction::new(0x26, Mnemonic::ROL, AddressingMode::ACC, 2, 5));
    m.insert(0x36, Instruction::new(0x36, Mnemonic::ROL, AddressingMode::ACC, 2, 6));
    m.insert(0x2e, Instruction::new(0x2e, Mnemonic::ROL, AddressingMode::ACC, 3, 6));
    m.insert(0x3e, Instruction::new(0x3e, Mnemonic::ROL, AddressingMode::ACC, 3, 7));
    // ROR
    m.insert(0x6a, Instruction::new(0x6a, Mnemonic::ROR, AddressingMode::ACC, 1, 2));
    m.insert(0x66, Instruction::new(0x66, Mnemonic::ROR, AddressingMode::ACC, 2, 5));
    m.insert(0x76, Instruction::new(0x76, Mnemonic::ROR, AddressingMode::ACC, 2, 6));
    m.insert(0x6e, Instruction::new(0x6e, Mnemonic::ROR, AddressingMode::ACC, 3, 6));
    m.insert(0x7e, Instruction::new(0x7e, Mnemonic::ROR, AddressingMode::ACC, 3, 7));
    // BPL
    m.insert(0x10, Instruction::new(0x10, Mnemonic::BPL, AddressingMode::REL, 2, 2));
    // MBI
    m.insert(0x30, Instruction::new(0x30, Mnemonic::BMI, AddressingMode::REL, 2, 2));
    // BVC
    m.insert(0x50, Instruction::new(0x50, Mnemonic::BVC, AddressingMode::REL, 2, 2));
    // BVS
    m.insert(0x70, Instruction::new(0x70, Mnemonic::BVS, AddressingMode::REL, 2, 2));
    // BCC
    m.insert(0x90, Instruction::new(0x90, Mnemonic::BCC, AddressingMode::REL, 2, 2));
    // BCS
    m.insert(0xB0, Instruction::new(0xB0, Mnemonic::BCS, AddressingMode::REL, 2, 2));
    // BNE
    m.insert(0xD0, Instruction::new(0xD0, Mnemonic::BNE, AddressingMode::REL, 2, 2));
    // BEQ
    m.insert(0xF0, Instruction::new(0xF0, Mnemonic::BEQ, AddressingMode::REL, 2, 2));
    // JMP
    m.insert(0x4c, Instruction::new(0x4c, Mnemonic::JMP, AddressingMode::ABS, 3, 3));
    m.insert(0x6c, Instruction::new(0x6c, Mnemonic::JMP, AddressingMode::IND, 3, 5));
    // JSR
    m.insert(0x20, Instruction::new(0x20, Mnemonic::JSR, AddressingMode::ABS, 3, 6));
    // RTI
    m.insert(0x40, Instruction::new(0x40, Mnemonic::RTI, AddressingMode::IMP, 1, 6));
    // RTS
    m.insert(0x60, Instruction::new(0x60, Mnemonic::RTS, AddressingMode::IMP, 1, 6));
    // CLC
    m.insert(0x18, Instruction::new(0x18, Mnemonic::CLC, AddressingMode::IMP, 1, 2));
    // SEC
    m.insert(0x38, Instruction::new(0x38, Mnemonic::SEC, AddressingMode::IMP, 1, 2));
    // CLI
    m.insert(0x58, Instruction::new(0x58, Mnemonic::CLI, AddressingMode::IMP, 1, 2));
    // SEI
    m.insert(0x78, Instruction::new(0x78, Mnemonic::SEI, AddressingMode::IMP, 1, 2));
    // CLV
    m.insert(0xb8, Instruction::new(0xb8, Mnemonic::CLV, AddressingMode::IMP, 1, 2));
    // CLD
    m.insert(0xd8, Instruction::new(0xd8, Mnemonic::CLD, AddressingMode::IMP, 1, 2));
    // SED
    m.insert(0xf8, Instruction::new(0xf8, Mnemonic::SED, AddressingMode::IMP, 1, 2));
    // CMP
    m.insert(0xc9, Instruction::new(0xc9, Mnemonic::CMP, AddressingMode::IMM, 2, 2));
    m.insert(0xc5, Instruction::new(0xc5, Mnemonic::CMP, AddressingMode::ZPG, 2, 3));
    m.insert(0xd5, Instruction::new(0xd5, Mnemonic::CMP, AddressingMode::ZPX, 2, 4));
    m.insert(0xcd, Instruction::new(0xcd, Mnemonic::CMP, AddressingMode::ABS, 3, 4));
    m.insert(0xdd, Instruction::new(0xdd, Mnemonic::CMP, AddressingMode::ABX, 3, 4));
    m.insert(0xd9, Instruction::new(0xd9, Mnemonic::CMP, AddressingMode::ABY, 3, 4));
    m.insert(0xc1, Instruction::new(0xc1, Mnemonic::CMP, AddressingMode::IDX, 2, 6));
    m.insert(0xd1, Instruction::new(0xd1, Mnemonic::CMP, AddressingMode::IDY, 2, 5));
    // CPX
    m.insert(0xe0, Instruction::new(0xe0, Mnemonic::CPX, AddressingMode::IMM, 2, 2));
    m.insert(0xe4, Instruction::new(0xe4, Mnemonic::CPX, AddressingMode::ZPG, 2, 3));
    m.insert(0xec, Instruction::new(0xec, Mnemonic::CPX, AddressingMode::ABS, 3, 4));
    // CPX
    m.insert(0xc0, Instruction::new(0xc0, Mnemonic::CPY, AddressingMode::IMM, 2, 2));
    m.insert(0xc4, Instruction::new(0xc4, Mnemonic::CPY, AddressingMode::ZPG, 2, 3));
    m.insert(0xcc, Instruction::new(0xcc, Mnemonic::CPY, AddressingMode::ABS, 3, 4));
    // PHA
    m.insert(0x48, Instruction::new(0x48, Mnemonic::PHA, AddressingMode::IMP, 1, 3));
    // PHP
    m.insert(0x08, Instruction::new(0x08, Mnemonic::PHP, AddressingMode::IMP, 1, 3));
    // PLA
    m.insert(0x68, Instruction::new(0x68, Mnemonic::PLA, AddressingMode::IMP, 1, 4));
    // PLP
    m.insert(0x28, Instruction::new(0x28, Mnemonic::PLP, AddressingMode::IMP, 1, 4));
    // BRK
    m.insert(0x00, Instruction::new(0x00, Mnemonic::BRK, AddressingMode::IMP, 1, 7));
    // NOP
    m.insert(0xea, Instruction::new(0xea, Mnemonic::NOP, AddressingMode::IMP, 1, 2));

    m
});
