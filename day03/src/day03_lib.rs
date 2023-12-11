use std::io::BufRead;

pub struct Schematic {
  grid: Vec<Vec<char>>,
}

impl Schematic {
  pub fn new(input: &Vec<u8>) -> Self {
    let mut grid: Vec<Vec<char>> = vec![];

    for line in input.lines() {
      grid.push(line.unwrap().chars().collect());
    }

    Self { grid }
  }

  pub fn find_part_numbers(&self) -> Vec<u32> {
    let mut part_numbers: Vec<u32> = vec![];

    for (row_idx, row) in self.grid.iter().enumerate() {
      for (col_idx, ch) in row.iter().enumerate() {
        if *ch != '.' && !ch.is_numeric() {
          println!("symbol: '{}' at ({},{})", ch, col_idx, row_idx);

          // check up a row
          if row_idx > 0 {
            match self.check_for_part_number(col_idx, row_idx - 1) {
              Some(part_number) => part_numbers.push(part_number),
              None => continue,
            }
          }

          // check left

          // check right

          // check down a row
        }
      }
    }
    vec![]
  }

  fn check_for_part_number(&self, x: usize, y: usize) -> Option<u32> {
    if self.grid[y][x].is_numeric() {
      //todo: actually get full number lol
      let value = self.grid[y][x].to_digit(10).unwrap();
      return Some(value);
    }
    None
  }
}

#[cfg(test)]
mod day03_tests {
  use super::*;

  #[test]
  fn find_part_numbers() {
    let input = "12..34\n$.....\n.56...".as_bytes().to_vec();
    let schematic = Schematic::new(&input);
    let expected_parts: Vec<u32> = vec![12, 56];
    assert_eq!(schematic.find_part_numbers(), expected_parts);
  }
}
