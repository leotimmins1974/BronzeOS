pub struct KeyboardManager {
    keydown: [bool; 128],
    extended_keydown: [bool; 128],
    extend_next: bool,
}

impl KeyboardManager {
    pub fn new() -> Self {
        Self {
            keydown: [false; 128],
            extended_keydown: [false; 128],
            extend_next: false,
        }
    }

    pub fn register_scancode(&mut self, scancode: u8) {
        // Handle extended keys
        if scancode == 0xE0 {
            self.extend_next = true;
            return;
        }

        let is_down = (scancode & 0b10000000u8) == 0;
        let base = scancode & !0b10000000;

        if self.extend_next {
            self.extended_keydown[base as usize] = is_down;
        } else {
            self.keydown[base as usize] = is_down;
        }
    }
}
