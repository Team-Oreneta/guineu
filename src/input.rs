use crate::print;
use crate::keyboard::{
    install_keyboard_buffer_handler,
    uninstall_keyboard_buffer_handler
};
use crate::println;

static mut KEYBOARD_BUFFER_PTR: *mut u8 = core::ptr::null_mut();
static mut KEYBOARD_BUFFER_SIZE: usize = 0;
static mut BUFFER_INDEX: usize = 0;
static mut IS_NEWLINE: bool = false;

pub fn input_handler(c: char) {
    unsafe {
        if KEYBOARD_BUFFER_PTR.is_null() {
            return;
        }

        if KEYBOARD_BUFFER_SIZE == 0 {
            return;
        }

        if c == '\n' {
            println!();
            IS_NEWLINE = true;
            return;
        } else {
            print!("{}", c);
        }

        let buffer = core::slice::from_raw_parts_mut(KEYBOARD_BUFFER_PTR, KEYBOARD_BUFFER_SIZE);
        if BUFFER_INDEX < KEYBOARD_BUFFER_SIZE  {
            buffer[BUFFER_INDEX] = c as u8;
            BUFFER_INDEX += 1;
        }
    }
}

pub fn get_user_input(buffer: &mut [u8]) -> usize {
    unsafe {
        // One byte per character
        KEYBOARD_BUFFER_PTR = buffer.as_mut_ptr();
        KEYBOARD_BUFFER_SIZE = buffer.len();
        BUFFER_INDEX = 0;
        IS_NEWLINE = false;

        install_keyboard_buffer_handler(input_handler);

        while (BUFFER_INDEX < KEYBOARD_BUFFER_SIZE) && !IS_NEWLINE {
            // Wait for the user to press a key
        }

        uninstall_keyboard_buffer_handler(input_handler);

        BUFFER_INDEX
    }
}