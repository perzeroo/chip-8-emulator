mod processor;
mod memory;
mod emulator;
mod renderer;
mod keyboard;
use std::env;
use std::thread;
use std::process;
use macroquad::prelude::*;

#[macroquad::main("Chip-8 Emulator")]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path to ROM>", args[0]);
        process::exit(1);
    }

    let rom_path = &args[1];

    let mut emu: emulator::Emulator = emulator::Emulator::new();
    println!("Successfully created the CPU and Memory");

    println!("Loading ROM from: {}", rom_path);
    match emu.mem.load_rom(rom_path) {
        Ok(_) => println!("ROM successfully loaded into Memory"),
        Err(e) => {
            eprintln!("Failed to load ROM into Memory: {}", e);
            process::exit(1);
        }
    }

    match emu.mem.load_rom_at_location("./font-data.bin", 0x0) {
        Ok(_) => println!("ROM successfully loaded into Memory"),
        Err(e) => {
            eprintln!("Failed to load ROM into Memory: {}", e);
            process::exit(1);
        }
    }

    emu.prepare();
    emu.clock();
    let emulator_logic_thread = thread::spawn(move || {
        loop {
            //do clock
        }
    });

    loop {
        emu.renderer.do_render();
        next_frame().await
    }
}


