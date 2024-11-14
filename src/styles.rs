#![allow(dead_code)]

// Define style codes
pub const PROMPT_STYLE: &str = "\x1b[90m";     // Grey text
pub const ITALIC_STYLE: &str = "\x1b[3m";      // Italic text
pub const NOTE_STYLE: &str = "\x1b[90;3m";     // Dark grey and italic text
pub const KEY_STYLE: &str = "\x1b[1;34m";      // Blue and bold text
pub const VALUE_STYLE: &str = "\x1b[1;3;32m";  // Green, bold and italic text
pub const RESET_STYLE: &str = "\x1b[0m";       // Reset style

pub fn style_prompt(prompt_string: &str) -> String {
    PROMPT_STYLE.to_string() + prompt_string + RESET_STYLE
}

pub fn style_note(note_string: &str) -> String {
    NOTE_STYLE.to_string() + note_string + RESET_STYLE
}

pub fn style_key(key_string: &str) -> String {
    KEY_STYLE.to_string() + key_string + RESET_STYLE
}

pub fn style_value(value_string: &str) -> String {
    VALUE_STYLE.to_string() + value_string + RESET_STYLE
}

pub fn style_italic(italic_string: &str) -> String {
    ITALIC_STYLE.to_string() + italic_string + RESET_STYLE
}