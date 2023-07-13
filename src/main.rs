use std::fs::File;
use std::io::prelude::*;

mod chip8;

fn main() {
    // Load the rom
    let mut file = File::open("roms/invaders")
        .expect("Unable to open file");

    let mut rom = Vec::<u8>::new();
    file.read_to_end(&mut rom)
        .expect("Unable to read file");

    println!("Loaded rom with {} bytes", rom.len());
    // print 2 bytes togheter
    // let mut i = 0;
    // while i < rom.len() {
    //     println!("{:02x}{:02x} ", rom[i], rom[i+1]);
    //     i += 2;
    // }

    // Create a new chip8
    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom(rom);

    chip8.run();
}
