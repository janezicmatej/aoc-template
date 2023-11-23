/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */

use std::env;
use std::fs;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        use aoc::template::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
        use std::fmt::Display;
        use std::time::Instant;

        fn print_result<T: Display>(func: impl FnOnce(&str) -> Option<T>, input: &str) {
            let timer = Instant::now();
            let result = func(input);
            let elapsed = timer.elapsed();
            match result {
                Some(result) => {
                    println!(
                        "{} {}(elapsed: {:.2?}){}",
                        result, ANSI_ITALIC, elapsed, ANSI_RESET
                    );
                }
                None => {
                    println!("not solved.")
                }
            }
        }

        fn main() {
            let input = aoc::template::read_file("inputs", 1);
            print!("{}Part {}{}: ", ANSI_BOLD, 1, ANSI_RESET);
            print_result(part_one, &input);
            print!("{}Part {}{}: ", ANSI_BOLD, 2, ANSI_RESET);
            print_result(part_two, &input);
        }
    };
}

#[must_use]
pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("data").join(folder).join(format!("{day:02}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file").trim().to_string()
}
