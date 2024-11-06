use macroquad::input::get_last_key_pressed;

pub fn get_hexkey_pressed() -> u8 {
    if let Some(key_code) = get_last_key_pressed() {
        let key = key_code as u16;
        if key < 58 && key > 47 {
            return (key - 48) as u8;
        } else if key > 64 && key < 71 {
            return (key - 55) as u8;
        }
    }
    return 0xFF;
}
