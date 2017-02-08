pub mod headers;

use std::ascii::AsciiExt;

/// Lower bound for ascii control characters
const ASCII_CONTROL_LOWER_BOUND: u8 = 0;
/// Upper bound for ascii control characters
const ASCII_CONTROL_UPPER_BOUND: u8 = 31;
/// Ascii del character
const ASCII_DEL_CHAR: u8 = 127;
/// Ascii tab character (\t)
const ASCII_TAB: u8 = 9;
/// Ascii space character
const ASCII_SPACE: u8 = 32;

/// Checks if a given byte is a "token" character as defined in [`RFC2616 Section 2.2`]:
///
/// Any ascii character
///
/// - excluding control characters (0 - 31 and 127)
/// - excluding "separators"
///
/// [`RFC2616 Section 2.2`]: https://tools.ietf.org/html/rfc2616#section-2.2
pub fn is_token(byte: u8) -> bool {
    if byte >= ASCII_CONTROL_LOWER_BOUND && byte <= ASCII_CONTROL_UPPER_BOUND {
        return false;
    }

    if byte == ASCII_DEL_CHAR {
        return false;
    }

    let char: char = byte.into();

    if !char.is_ascii() {
        return false;
    }

    match char {
        // excludes "separators"
        '(' | ')' | '<' | '>' | '@' | ',' | ';' | ':' | '\\' | '"' | '/' | '[' | ']' | '?' |
        '=' | '{' | '}' | ' ' | '\t' => false,
        _ => true,
    }
}

pub fn is_whitespace(byte: u8) -> bool {
    byte == ASCII_SPACE || byte == ASCII_TAB
}

pub fn is_control(byte: u8) -> bool {
    match byte {
        ASCII_CONTROL_LOWER_BOUND...ASCII_CONTROL_UPPER_BOUND |
        ASCII_DEL_CHAR => true,
        _ => false,
    }
}
