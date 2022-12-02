// based on https://codegolf.stackexchange.com/q/77510

mod solutions;

use solutions::{
    sol_naive_nonwrapping,
    sol_naive_wrapping,
};

use std::error::Error;

fn main() {
    let naive_cost = compute_cost(Box::new(sol_naive_wrapping));

    for contender in [
        ("Naive Non-Wrapping", Box::new(sol_naive_nonwrapping as fn(&str) -> Result<String, Box<dyn Error>>)),
        ("Naive Wrapping", Box::new(sol_naive_wrapping)),
    ] {
        let (name, solution) = contender;
        let cost = compute_cost(solution);
        println!("{name}: {:.2}%", cost / naive_cost * 100.0);
    }
}

fn compute_cost(solution: Box<dyn Fn(&str) -> Result<String, Box<dyn Error>>>) -> f64 {
    let mut cost = 0.0;

    for test_vector in TEST_VECTORS {
        match solution(*test_vector) {
            Ok(commands) => cost = cost + commands.len() as f64,
            Err(_) => return f64::NAN,
        }
    }

    cost
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

const TEST_VECTORS: &[&str] = [
    "101",
    "quip",
    "PPCG",
    "Mego",
    "Noob 5",
    "penguin",
    "867-5309",
    "2_sPoOkY_4_mE",
    "The Nineteenth Byte",
    "penguins@SouthPole.org",
    "8xM3R__5ltZgrkJ.-W b",
    "correcthorsebatterystaple",
    "verylongRUNOFCAPSandnocaps",
    "This is an English sentence.",
    "WNtza.akjzSP2GIOV9X .OepmUQ-mo",
    "Programming Puzzles and Code Golf",
].as_slice();

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
            'Y' | 'y' => {
                text.insert(pointer, ' ');
                pointer = pointer + 1;
            },
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
    use std::error::Error;

    use super::{
        TEST_VECTORS,
        kbd_input,
        sol_naive_nonwrapping,
    };

    #[test]
    fn test_kbd_input() {
        assert_eq!(kbd_input("").unwrap(), "");
        assert_eq!(kbd_input("A").unwrap(), "1");
        assert_eq!(kbd_input("DCACA").unwrap(), "Qq");
        assert_eq!(kbd_input("ADDDDAUUUUARRRRRRRRRRALLLLLLLLLLA").unwrap(), "11111");
    }

    #[test]
    fn test_naive_nonwrapping() {
        test_correctness(Box::new(sol_naive_nonwrapping));
    }

    fn test_correctness(solution: Box<dyn Fn(&str) -> Result<String, Box<dyn Error>>>) {
        for test_vector in TEST_VECTORS {
            assert_eq!(kbd_input(&solution(test_vector).unwrap()).unwrap(), *test_vector);
        }
    }
}
