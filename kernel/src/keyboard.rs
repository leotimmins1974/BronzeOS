use lazy_static::lazy_static;
use spin::Mutex;

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

        /* print the key pressed - debugging */
        //if is_down {
        //    let c: u8 = if self.extend_next {
        //        match base {
        //            0x1C => b'\n', // enter - numpad
        //            0x35 => b'/',  // divide - numpad
        //            _ => 0,
        //        }
        //    } else {
        //        match base {
        //            0x02 => b'1',
        //            0x03 => b'2',
        //            0x04 => b'3',
        //            0x05 => b'4',
        //            0x06 => b'5',
        //            0x07 => b'6',
        //            0x08 => b'7',
        //            0x09 => b'8',
        //            0x0A => b'9',
        //            0x0B => b'0',
        //            0x0C => b'-',
        //            0x0D => b'=',
        //            0x0E => b'\x08', // backspace
        //            0x0F => b'\t',
        //            0x10 => b'q',
        //            0x11 => b'w',
        //            0x12 => b'e',
        //            0x13 => b'r',
        //            0x14 => b't',
        //            0x15 => b'y',
        //            0x16 => b'u',
        //            0x17 => b'i',
        //            0x18 => b'o',
        //            0x19 => b'p',
        //            0x1A => b'[',
        //            0x1B => b']',
        //            0x1C => b'\n',
        //            0x1E => b'a',
        //            0x1F => b's',
        //            0x20 => b'd',
        //            0x21 => b'f',
        //            0x22 => b'g',
        //            0x23 => b'h',
        //            0x24 => b'j',
        //            0x25 => b'k',
        //            0x26 => b'l',
        //            0x27 => b';',
        //            0x28 => b'\'',
        //            0x29 => b'`',
        //            0x2B => b'\\',
        //            0x2C => b'z',
        //            0x2D => b'x',
        //            0x2E => b'c',
        //            0x2F => b'v',
        //            0x30 => b'b',
        //            0x31 => b'n',
        //            0x32 => b'm',
        //            0x33 => b',',
        //            0x34 => b'.',
        //            0x35 => b'/',
        //            0x37 => b'*',  // * - keypad
        //            0x39 => b' ',
        //            0x4A => b'-',  // - - keypad
        //            0x4E => b'+',  // + - keypad
        //            0x53 => b'.',  // delete keypad
//
        //            // keypad digits
        //            0x47 => b'7',
        //            0x48 => b'8',
        //            0x49 => b'9',
        //            0x4B => b'4',
        //            0x4C => b'5',
        //            0x4D => b'6',
        //            0x4F => b'1',
        //            0x50 => b'2',
        //            0x51 => b'3',
        //            0x52 => b'0',
//
        //            _ => 0,
        //        }
        //    };
//
        //    if let Some(gm) = super::DISPLAY.lock().as_mut() {
        //        if c != 0 {
        //            gm.write_text(super::PURPLE, &[c]);
        //        }
        //    }
        //}
    }
}