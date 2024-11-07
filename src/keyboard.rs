use macroquad::input::KeyCode;

pub struct Keyboard {
    pub last_key_pressed: KeyCode,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            last_key_pressed: KeyCode::Z,
        }
    }

    pub fn get_hexkey_pressed(&mut self) -> u8 {
        let key = self.last_key_pressed as u16;
        if key < 58 && key > 47 {
            return (key - 48) as u8;
        } else if key > 64 && key < 71 {
            return (key - 55) as u8;
        }
        return 0xFF;
    }
}

