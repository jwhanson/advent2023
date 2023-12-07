use std::{char::from_digit, ops::Add};

pub struct CalibrationEntry {
  digit_only_value: u32,
  word_or_digit_value: u32,
}

impl CalibrationEntry {
  pub fn new(input: &str) -> Self {
    Self {
      digit_only_value: Self::parse_digit_only_value(input),
      word_or_digit_value: Self::parse_word_or_digit_value(input),
    }
  }

  pub fn get_digit_only_value(&self) -> u32 {
    self.digit_only_value
  }

  pub fn get_word_or_digit_value(&self) -> u32 {
    self.word_or_digit_value
  }

  fn parse_digit_only_value(input: &str) -> u32 {
    let first_numeric_char = input.chars().find(|c| c.is_numeric()).unwrap();
    let last_numeric_char = input.chars().rfind(|c| c.is_numeric()).unwrap();
    format!("{}{}", first_numeric_char, last_numeric_char)
      .parse()
      .unwrap()
  }

  fn parse_word_or_digit_value(input: &str) -> u32 {
    let words = &[
      "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let first_numeral = input.char_indices().find(|(_, ch)| ch.is_numeric());

    let first_number_word = input.char_indices().find_map(|(idx, _)| {
      words
        .iter()
        .enumerate()
        .find(|(_, word)| {
          idx + word.len() <= input.len()
            && word.chars().cmp(input[idx..idx + word.len()].chars()) == std::cmp::Ordering::Equal
        })
        .map(|(num, _)| (idx, from_digit(num.add(1).try_into().unwrap(), 10).unwrap()))
    });

    let first_output = match (first_numeral, first_number_word) {
      (None, None) => panic!("failed to find first"),
      (Some(numeral), None) => numeral,
      (None, Some(word)) => word,
      (Some(numeral), Some(word)) => std::cmp::min(numeral, word),
    };

    let last_numeral = input.char_indices().rev().find(|(_, ch)| ch.is_numeric());

    let last_number_word = input.char_indices().rev().find_map(|(idx, _)| {
      words
        .iter()
        .enumerate()
        .find(|(_, word)| {
          idx + word.len() <= input.len()
            && word.chars().cmp(input[idx..idx + word.len()].chars()) == std::cmp::Ordering::Equal
        })
        .map(|(num, _)| (idx, from_digit(num.add(1).try_into().unwrap(), 10).unwrap()))
    });

    let last_output = match (last_numeral, last_number_word) {
      (None, None) => panic!("failed to find last"),
      (Some(numeral), None) => numeral,
      (None, Some(word)) => word,
      (Some(numeral), Some(word)) => std::cmp::max(numeral, word),
    };

    format!("{}{}", first_output.1, last_output.1)
      .parse()
      .unwrap()
  }
}

#[cfg(test)]
mod day01_tests {
  use super::*;

  #[test]
  fn no_words_in_input() {
    let entry = CalibrationEntry::new("1abc2");
    assert_eq!(entry.get_digit_only_value(), 12);
    assert_eq!(entry.get_word_or_digit_value(), 12);
  }

  #[test]
  fn words_at_ends_of_input() {
    let entry = CalibrationEntry::new("two1nine");
    assert_eq!(entry.get_digit_only_value(), 11);
    assert_eq!(entry.get_word_or_digit_value(), 29);
  }
}
