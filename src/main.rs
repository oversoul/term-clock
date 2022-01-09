use chrono::prelude::*;
use colored::*;
use std::collections::HashMap;
use std::time::Duration;

fn main() {
    let space = [[0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]];
    let one = [[0, 0, 1], [0, 1, 1], [1, 0, 1], [0, 0, 1], [0, 0, 1]];
    let two = [[1, 1, 1], [0, 0, 1], [1, 1, 1], [1, 0, 0], [1, 1, 1]];
    let three = [[1, 1, 1], [0, 0, 1], [1, 1, 1], [0, 0, 1], [1, 1, 1]];
    let four = [[1, 0, 1], [1, 0, 1], [1, 1, 1], [0, 0, 1], [0, 0, 1]];
    let five = [[1, 1, 1], [1, 0, 0], [1, 1, 1], [0, 0, 1], [1, 1, 1]];
    let six = [[1, 1, 1], [1, 0, 0], [1, 1, 1], [1, 0, 1], [1, 1, 1]];
    let seven = [[1, 1, 1], [0, 0, 1], [0, 1, 1], [0, 0, 1], [0, 0, 1]];
    let eight = [[1, 1, 1], [1, 0, 1], [1, 1, 1], [1, 0, 1], [1, 1, 1]];
    let nine = [[1, 1, 1], [1, 0, 1], [1, 1, 1], [0, 0, 1], [1, 1, 1]];
    let zero = [[1, 1, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 1, 1]];
    let dots = [[0, 0, 0], [0, 1, 0], [0, 0, 0], [0, 1, 0], [0, 0, 0]];

    let mut numbers: HashMap<char, [[u8; 3]; 5]> = HashMap::new();
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

    // println!("{:?} hour", hour_str.chars());

    loop {
        let current_time = chrono::offset::Local::now();
        clear_console();
        hide_cursor();
        let clock = format!(
            "{:0width$}:{:0width$}:{:0width$}",
            current_time.hour(),
            current_time.minute(),
            current_time.second(),
            width = 2
        );
        println!();
        let time: String = [0, 1, 2, 3, 4]
            .iter()
            .map(|row| {
                format!(
                    "{}\n",
                    clock
                        .chars()
                        .map(|c| numbers.get(&c).unwrap_or(&space))
                        .map(|num| format!("{}   ", draw_number_line(*num, *row)))
                        .collect::<String>()
                        .trim_end()
                )
            })
            .collect::<String>();

        print!("{}", time);

        /*
        println!( "{:?} {} {}", current_time.hour(), current_time.minute(), current_time.second());
        */
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

fn hide_cursor() {
    print!("");
}

fn draw_number_line(number: [[u8; 3]; 5], line: usize) -> String {
    number[line]
        .iter()
        .map(|c| {
            if *c == 0 {
                format!("{}", "   ".normal())
            } else {
                format!("{}", "   ".reverse())
            }
        })
        .collect::<String>()
}
