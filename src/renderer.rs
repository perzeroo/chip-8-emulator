use std::sync::{Arc, Mutex};

pub struct Renderer {
    pub pixels_mutex: Arc<Mutex<Vec<u8>>>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer { pixels_mutex: Arc::new(Mutex::new(vec![0u8; 64 * 32])), }
    }

    pub fn clear_pixels(&mut self) {
        let mut pixels = self.pixels_mutex.lock().unwrap();
        for pixel in pixels.iter_mut() {
            *pixel = 0;
        }
    }
}
