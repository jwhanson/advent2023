#[derive(Debug, PartialEq)]
struct Card {
  winning_numbers: Vec<u32>,
  numbers: Vec<u32>,
}

pub fn solve(input: &str) -> u32 {
  let cards: Vec<Card> = parse_cards(input);

  cards.iter().map(evaluate_card).sum()
}

pub fn solve_2(input: &str) -> u32 {
  let cards: Vec<Card> = parse_cards(input);

  let number_of_wins_per_card = cards.iter().map(count_winners).collect::<Vec<u32>>();

  let mut card_counts = vec![1; cards.len()]; // start with one of each card
  for (id, winner_count) in number_of_wins_per_card
    .iter()
    .map(|val| *val as usize)
    .enumerate()
  {
    for i in 1..=winner_count {
      if id + i < card_counts.len() {
        card_counts[id + i] += card_counts[id];
      }
    }
  }

  card_counts.iter().sum()
}

fn parse_cards(input: &str) -> Vec<Card> {
  let mut cards = vec![];

  for line in input.lines() {
    cards.push(parse_card(line));
  }

  cards
}

fn parse_card(line: &str) -> Card {
  let mut winning_numbers = vec![];
  let mut numbers = vec![];

  let mut i = 0; // input mode; a lazy choice
  for word in line.split(' ') {
    if word == "|" {
      // switch modes once we find a '|'
      i += 1;
      continue;
    }

    if let Ok(val) = word.parse() {
      // if we have a number, add it based on input mode
      match i {
        0 => winning_numbers.push(val),
        1 => numbers.push(val),
        _ => panic!("unexpected input mode"),
      }
    }
  }

  Card {
    winning_numbers,
    numbers,
  }
}

fn evaluate_card(card: &Card) -> u32 {
  let win_count = count_winners(card);

  if win_count == 0 {
    0
  } else {
    2_u32.pow(win_count - 1)
  }
}

/// returns the count of winning numbers that appear in the card's numbers
fn count_winners(card: &Card) -> u32 {
  let mut win_count = 0;

  // Q: rust analyzer gave me the following & and I don't know why
  for winning_number in &card.winning_numbers {
    if card.numbers.contains(winning_number) {
      win_count += 1;
    }
  }

  win_count
}

#[cfg(test)]
mod day03_tests {
  use super::*;

  #[test]
  fn check_parse_card() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    let winning = vec![41, 48, 83, 86, 17];
    let numbers = vec![83, 86, 6, 31, 17, 9, 48, 53];
    let card = Card {
      winning_numbers: winning,
      numbers: numbers,
    };
    assert_eq!(parse_card(input), card);
  }

  #[test]
  fn check_eval_card() {
    // a scoring card
    let card = Card {
      winning_numbers: vec![41, 48, 83, 86, 17],
      numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
    };
    let score = 8;
    assert_eq!(evaluate_card(&card), score);

    // a non-scoring card
    let card = Card {
      winning_numbers: vec![1, 2, 3],
      numbers: vec![4, 5, 6],
    };
    let score = 0;
    assert_eq!(evaluate_card(&card), score);

    // a single-scoring card
    let card = Card {
      winning_numbers: vec![1, 2, 3],
      numbers: vec![3, 4, 5, 6, 7, 8, 9],
    };
    let score = 1;
    assert_eq!(evaluate_card(&card), score);
  }

  #[test]
  fn check_solve() {
    let input =
      std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/example.txt")).unwrap();
    let solution = 13;
    assert_eq!(solve(&input), solution);
  }

  #[test]
  fn check_solve_2() {
    let input =
      std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/example.txt")).unwrap();
    let solution = 30;
    assert_eq!(solve_2(&input), solution);
  }
}
