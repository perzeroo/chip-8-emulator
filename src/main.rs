mod processor;
mod memory;
mod emulator;
mod renderer;
mod keyboard;
use std::env;
use std::process;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;
use std::mem;
use emulator::Emulator;
use keyboard::Keyboard;
use macroquad::prelude::*;
use renderer::Renderer;

#[macroquad::main("Chip-8 Emulator")]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path to ROM>", args[0]);
        process::exit(1);
    }
    let mut emu: emulator::Emulator = emulator::Emulator::new();
    let renderer: Arc<Mutex<Renderer>> = Arc::new(Mutex::new(Renderer::new()));
    let renderer_copy: Arc<Mutex<Renderer>> = Arc::clone(&renderer);
    let keyboard_mutex: Arc<Mutex<Keyboard>> = Arc::new(Mutex::new(Keyboard::new()));
    let keyboard_mutex_copy: Arc<Mutex<Keyboard>> = Arc::clone(&keyboard_mutex);

    let emulator_logic_thread = spawn(move || {
        let rom_path = &args[1];

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
        loop {
            emu.clock(renderer_copy.clone(), keyboard_mutex_copy.clone());
        }
    });


    loop {
        let mut keyboard = keyboard_mutex.lock().unwrap(); 

        if let Some(key) = get_last_key_pressed() {
            (*keyboard).last_key_pressed = key;
        } else {
            if is_key_released((*keyboard).last_key_pressed) {
                (*keyboard).last_key_pressed = KeyCode::Z;
            }
        }
        mem::drop(keyboard);
        let mut rend = renderer.lock().unwrap();
        (*rend).do_render();
        mem::drop(rend);
        next_frame().await
    }
}

fn set_last_key_pressed(emu: &mut Emulator) {
}

