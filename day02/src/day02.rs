use std::fs::File;
use std::io::{BufRead, BufReader};

use day02_lib::CubeGame;

fn main() {
  let file = File::open("day02/assets/input.txt").unwrap();
  let reader = BufReader::new(file);
  let mut valid_game_id_sum: u32 = 0;
  let mut min_set_power_sum: u32 = 0;

  for line in reader.lines() {
    let line = line.unwrap();
    let game = CubeGame::new(&line);
    if game.is_valid() {
      valid_game_id_sum += game.get_id();
    }
    min_set_power_sum += game.get_min_set_power();
  }

  println!("{}", valid_game_id_sum);
  println!("{}", min_set_power_sum);
}
