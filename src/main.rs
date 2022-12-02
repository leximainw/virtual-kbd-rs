// based on https://codegolf.stackexchange.com/q/77510

use std::error::Error;

fn main() {
    println!("Hello, world!");
}

const UPPER_CHARS: [char; 40] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
    'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P',
    'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', '-',
    'Z', 'X', 'C', 'V', 'B', 'N', 'M', '_', '@', '.',
];

const LOWER_CHARS: [char; 40] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
    'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p',
    'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', '-',
    'z', 'x', 'c', 'v', 'b', 'n', 'm', '_', '@', '.',
];

fn kbd_input(commands: &str) -> Result<String, Box<dyn Error>> {
    let mut cursor = (0, 0);
    let mut caps_lock = false;
    let mut pointer = 0;
    let mut text = String::new();

    for c in commands.chars() {
        match c {
            'L' | 'l' => cursor.0 = (cursor.0 + 9) % 10,
            'R' | 'r' => cursor.0 = (cursor.0 + 1) % 10,
            'U' | 'u' => cursor.1 = (cursor.1 + 3) % 4,
            'D' | 'd' => cursor.1 = (cursor.1 + 1) % 4,
            'Y' | 'y' => text.insert(pointer, ' '),
            'B' | 'b' => pointer = std::cmp::max(pointer - 1, 0),
            'F' | 'f' => pointer = std::cmp::min(pointer + 1, text.len()),
            'C' | 'c' => caps_lock = !caps_lock,
            'A' | 'a' => {
                text.insert(pointer,
                    if caps_lock {
                        UPPER_CHARS
                    } else {
                        LOWER_CHARS
                    }[cursor.0 + cursor.1 * 10]);
                pointer = pointer + 1;
            },
            _ => return Err(Box::from(format!("unexpected command {c}"))),
        }
    }
    Ok(text)
}

#[cfg(test)]
mod tests {
    use crate::kbd_input;

    #[test]
    fn test_kbd_input() {
        assert_eq!(kbd_input("").unwrap(), "");
        assert_eq!(kbd_input("A").unwrap(), "1");
        assert_eq!(kbd_input("DCACA").unwrap(), "Qq");
        assert_eq!(kbd_input("ADDDDAUUUUARRRRRRRRRRALLLLLLLLLLA").unwrap(), "11111");
    }
}
