use crate::{
    UPPER_CHARS,
    LOWER_CHARS,
};

use std::error::Error;

pub fn sol_naive_nonwrapping(input: &str) -> Result<String, Box<dyn Error>> {
    let mut curr_x = 0;
    let mut curr_y = 0;
    let mut caps = false;
    let mut commands = String::new();

    for c in input.chars() {
        if c == ' ' {
            commands.push('Y');
            continue;
        }

        let (curr_chars, flip_chars) = if caps {
            (UPPER_CHARS, LOWER_CHARS)
        } else {
            (LOWER_CHARS, UPPER_CHARS)
        };
        let (new_x, new_y, flip) = if let Some(index) = curr_chars.iter().position(|x| *x == c) {
            (index % 10, index / 10, false)
        } else if let Some(index) = flip_chars.iter().position(|x| *x == c) {
            (index % 10, index / 10, true)
        } else {
            return Err(Box::from("encountered out-of-range character {c}"));
        };

        if curr_y != new_y {
            if curr_y < new_y {
                commands.push_str(&"D".repeat(new_y - curr_y));
            } else {
                commands.push_str(&"U".repeat(curr_y - new_y));
            }
            curr_y = new_y;
        }

        if curr_x != new_x {
            if curr_x < new_x {
                commands.push_str(&"R".repeat(new_x - curr_x));
            } else {
                commands.push_str(&"L".repeat(curr_x - new_x));
            }
            curr_x = new_x;
        }

        if flip {
            caps = !caps;
            commands.push('C');
        }

        commands.push('A');
    }

    Ok(commands)
}
