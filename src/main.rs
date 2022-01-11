use chrono::prelude::*;
use colored::*;
use std::collections::HashMap;
use std::time::Duration;

const BLOCK: &str = "  ";
type Font = [[u8; 5]; 7];

fn main() {
    let space = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let one = [
        [0, 0, 1, 0, 0],
        [0, 1, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
    ];
    let two = [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [1, 1, 1, 1, 1],
    ];
    let three = [
        [1, 1, 1, 1, 0],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [1, 1, 1, 1, 0],
    ];
    let four = [
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
    ];
    let five = [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 0],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [1, 1, 1, 1, 0],
    ];
    let six = [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ];
    let seven = [
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
    ];
    let eight = [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ];
    let nine = [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ];
    let zero = [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 1, 1],
        [1, 0, 1, 0, 1],
        [1, 1, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ];
    let dots = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut numbers: HashMap<char, Font> = HashMap::new();
    numbers.insert(':', dots);
    numbers.insert('0', zero);
    numbers.insert('1', one);
    numbers.insert('2', two);
    numbers.insert('3', three);
    numbers.insert('4', four);
    numbers.insert('5', five);
    numbers.insert('6', six);
    numbers.insert('7', seven);
    numbers.insert('8', eight);
    numbers.insert('9', nine);

    let mut empty_lines = 0;
    let mut padding_left = 0;
    let shape = "..... ..... ::::: ..... ..... ::::: ..... .....";

    if let Some((w, h)) = term_size::dimensions() {
        // println!("Width: {}\nHeight: {}", w, h);
        empty_lines = (h - 7) / 2;
        padding_left = ((w / BLOCK.len()) - shape.len()) / 2;
    }

    loop {
        let current_time = chrono::offset::Local::now();
        clear_console();
        for _ in 0..empty_lines {
            println!();
        }
        let clock = format!(
            "{:0width$}:{:0width$}:{:0width$}",
            current_time.hour(),
            current_time.minute(),
            current_time.second(),
            width = 2
        );
        let time: String = (0..7)
            .map(|row| {
                format!(
                    "{}{}\n",
                    BLOCK.repeat(padding_left),
                    clock
                        .chars()
                        .map(|c| numbers.get(&c).unwrap_or(&space))
                        .map(|num| format!("{}{}", draw_number_line(*num, row), BLOCK))
                        .collect::<String>()
                        .trim_end()
                )
            })
            .collect::<String>();

        print!("{}", time);

        std::thread::sleep(Duration::from_secs(1));
    }
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

fn draw_number_line(number: Font, line: usize) -> String {
    number[line]
        .iter()
        .map(|c| {
            if *c == 0 {
                format!("{}", BLOCK.normal())
            } else {
                format!("{}", BLOCK.reverse().blue())
            }
        })
        .collect::<String>()
}
