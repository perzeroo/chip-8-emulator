use std::{sync::{Arc, Mutex}, usize};
use macroquad::prelude::*;

pub struct Renderer {
    pub pixels_mutex: Arc<Mutex<Vec<u8>>>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            pixels_mutex: Arc::new(Mutex::new(vec![0u8; 64 * 32])),
        }
    }

    pub fn clear_pixels(&mut self) {
        let mut pixels = self.pixels_mutex.lock().unwrap();

        let mut it = 0;
        for pixel in pixels.iter_mut() {
            *pixel = if it % 3 == 0 {1} else {0};
            it += 1;
        }
    }

    pub fn draw_pixel(&mut self, x: u8, y: u8) {
        
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: Vec<u8>) {
        let mut pixels = self.pixels_mutex.lock().unwrap();
        let mut current_y = y;
        for block in sprite.iter() {
            if current_y >= 32 {
                break;
            }
            for i in 0..8 {
                if (i + x as usize) >= 64 {
                    break;
                }
                let current_pixel: usize = (current_y as usize) * 64 + i + x as usize;
                pixels[current_pixel] = ((block << i) & 128) >> 7; // render the byte left to right
                // not sure if correct

            }
            current_y += 1;
        }
    }

    pub fn do_render(&mut self) {
        clear_background(BLACK);

        let pixel_size = screen_height() / 32.0;
        
        let mut current_x: f32;
        let mut current_y: f32;
        let mut it = 0;
        let pixels = self.pixels_mutex.lock().unwrap();
        for pixel in pixels.iter() {
            current_x = (it as f32 % 64.0) * pixel_size;
            current_y = ((it / 64) * pixel_size as i32) as f32;
            if *pixel == 1 {
                draw_rectangle(current_x, current_y, pixel_size, pixel_size, WHITE);
            }
            it += 1;
        }
    }
}


