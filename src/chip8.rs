mod cpu;
mod memory;
mod display;

use cpu::CPU;
use memory::Memory;
use display::Display;

pub struct Chip8 {
    memory: Memory,
    cpu: CPU,
    display: Display
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: Memory::new(),
            cpu: CPU::new(),
            display: Display::new()
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        for (i, byte) in rom.iter().enumerate() {
            self.memory.write_to_address((i as u16) + 0x200, *byte);
        }
    }

    pub fn run(&mut self) {
        loop {
            self.display.display();
            self.cpu.execute_instruction(&mut self.memory, &mut self.display);
        }
    }
}
