use crate::text;

pub fn clear() {
    text::WRITER.lock().clear_screen();
    text::WRITER.lock().reset_cursor();
}