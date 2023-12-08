use std::collections::HashMap;

pub struct CubeGame {
  id: u32,
  max_cube_counts: HashMap<String, u32>,
}

impl CubeGame {
  pub fn new(input: &str) -> Self {
    let id = input
      .split(':')
      .nth(0)
      .unwrap()
      .split(' ')
      .nth(1)
      .unwrap()
      .parse()
      .unwrap();

    let mut max_observed_counts: HashMap<String, u32> = HashMap::new();
    for handful in input.split(':').nth(1).unwrap().split(';') {
      for entry in handful.split(',').map(|s| s.trim()) {
        let count: u32 = entry.split(' ').nth(0).unwrap().parse().unwrap();
        let color = entry.split(' ').nth(1).unwrap();
        match max_observed_counts.contains_key(color) {
          true => {
            *max_observed_counts.get_mut(color).unwrap() =
              std::cmp::max(max_observed_counts[color], count);
          }
          false => {
            max_observed_counts.insert(color.to_string(), count);
          }
        }
      }
    }

    Self {
      id,
      max_cube_counts: max_observed_counts,
    }
  }

  pub fn get_id(&self) -> u32 {
    self.id
  }

  pub fn is_valid(&self) -> bool {
    self.max_cube_counts["red"] <= 12
      && self.max_cube_counts["green"] <= 13
      && self.max_cube_counts["blue"] <= 14
  }

  pub fn get_min_set_power(&self) -> u32 {
    self.max_cube_counts["red"] * self.max_cube_counts["green"] * self.max_cube_counts["blue"]
  }
}

#[cfg(test)]
mod day02_tests {
  use super::*;

  #[test]
  fn valid_game() {
    let good_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let good_game = CubeGame::new(good_input);
    assert_eq!(good_game.get_id(), 1);
    assert_eq!(good_game.is_valid(), true);
    assert_eq!(good_game.get_min_set_power(), 48);
  }

  #[test]
  fn invalid_game() {
    let bad_input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
    let bad_game = CubeGame::new(bad_input);
    assert_eq!(bad_game.get_id(), 3);
    assert_eq!(bad_game.is_valid(), false);
    assert_eq!(bad_game.get_min_set_power(), 1560);
  }
}
