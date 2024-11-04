mod processor;
mod memory;
mod emulator;
use std::env;
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
    emu.prepare();
    emu.clock();

    loop {
        clear_background(BLACK);
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello World!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

