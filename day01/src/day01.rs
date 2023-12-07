use std::fs::File;
use std::io::{BufRead, BufReader};

use day01_lib::CalibrationEntry;

fn main() {
  let file = File::open("day01/assets/input.txt").unwrap();
  let reader = BufReader::new(file);
  let mut result_part1: u32 = 0;
  let mut result_part2: u32 = 0;

  for line in reader.lines() {
    let line = line.unwrap();
    let entry = CalibrationEntry::new(&line);
    result_part1 += entry.get_digit_only_value();
    result_part2 += entry.get_word_or_digit_value();
  }

  println!("{}", result_part1);
  println!("{}", result_part2);
}
