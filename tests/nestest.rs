use std::{fs::File, io::Read};

use powerglove::cpu::CPU;

#[test]
fn test_rom_nestest() {
    let mut f = File::open("./test-roms/nestest.nes").unwrap();
    let mut buffer = [0; 24592];
    
    f.read(&mut buffer).unwrap();

    let mut cpu = CPU::new();
    
    // Rough loading of `nestest` since we don't actually support loading cartridges yet
    for (i, byte) in buffer[0x10..0x4010].into_iter().enumerate() {
        cpu.write(0x8000 + i as u16, *byte);
        cpu.write(0xC000 + i as u16, *byte);
    }

    // Set CPU vector and reset
    cpu.bus.ram[0xFFFC] = 0x00;
    cpu.bus.ram[0xFFFD] = 0x80;
    cpu.reset();

    // Setting the PC to 0xC000 allows nestest to run in `auto` mode.
    cpu.pc = 0xC000;

    loop {
        cpu.clock();

        if cpu.pc == 0x004 && cpu.cycles_remaining == 0 {
            break;
        }
    }

    let lo = cpu.read(0x0002); 
    let hi = cpu.read(0x0003);
    let result = u16::from_le_bytes([lo, hi]);
    
    assert_eq!(0x001, result);
}