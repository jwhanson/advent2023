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
              None => {
                // well, it could be diagonal
                if col_idx > 0 {
                  if let Some(part_number) = self.check_for_part_number(col_idx - 1, row_idx - 1) {
                    part_numbers.push(part_number)
                  }
                }
                if col_idx < self.grid[row_idx].len() - 1 {
                  if let Some(part_number) = self.check_for_part_number(col_idx + 1, row_idx - 1) {
                    part_numbers.push(part_number)
                  }
                }
              }
            }
          }

          // check left
          if col_idx > 0 {
            if let Some(part_number) = self.check_for_part_number(col_idx - 1, row_idx) {
              part_numbers.push(part_number);
            }
          }

          // check right
          if col_idx < self.grid[row_idx].len() - 1 {
            if let Some(part_number) = self.check_for_part_number(col_idx + 1, row_idx) {
              part_numbers.push(part_number);
            }
          }

          // check down a row
          if row_idx < self.grid.len() - 1 {
            match self.check_for_part_number(col_idx, row_idx + 1) {
              Some(part_number) => part_numbers.push(part_number),
              None => {
                // well, it could be diagonal
                if col_idx > 0 {
                  if let Some(part_number) = self.check_for_part_number(col_idx - 1, row_idx + 1) {
                    part_numbers.push(part_number)
                  }
                }
                if col_idx < self.grid[row_idx].len() - 1 {
                  if let Some(part_number) = self.check_for_part_number(col_idx + 1, row_idx + 1) {
                    part_numbers.push(part_number)
                  }
                }
              }
            }
          }
        }
      }
    }

    part_numbers
  }

  pub fn find_gear_ratios(&self) -> Vec<u32> {
    let mut gear_ratios = vec![];

    for (row_idx, row) in self.grid.iter().enumerate() {
      for (col_idx, ch) in row.iter().enumerate() {
        if *ch == '*' {
          println!("gear found at ({},{})", col_idx, row_idx);
          let mut gears = vec![];

          // check up a row
          if row_idx > 0 {
            match self.check_for_part_number(col_idx, row_idx - 1) {
              Some(part_number) => gears.push(part_number),
              None => {
                // well, it could be diagonal
                if col_idx > 0 {
                  if let Some(part_number) = self.check_for_part_number(col_idx - 1, row_idx - 1) {
                    gears.push(part_number)
                  }
                }
                if col_idx < self.grid[row_idx].len() - 1 {
                  if let Some(part_number) = self.check_for_part_number(col_idx + 1, row_idx - 1) {
                    gears.push(part_number)
                  }
                }
              }
            }
          }

          // check left
          if col_idx > 0 {
            if let Some(part_number) = self.check_for_part_number(col_idx - 1, row_idx) {
              gears.push(part_number);
            }
          }

          // check right
          if col_idx < self.grid[row_idx].len() - 1 {
            if let Some(part_number) = self.check_for_part_number(col_idx + 1, row_idx) {
              gears.push(part_number);
            }
          }

          // check down a row
          if row_idx < self.grid.len() - 1 {
            match self.check_for_part_number(col_idx, row_idx + 1) {
              Some(part_number) => gears.push(part_number),
              None => {
                // well, it could be diagonal
                if col_idx > 0 {
                  if let Some(part_number) = self.check_for_part_number(col_idx - 1, row_idx + 1) {
                    gears.push(part_number)
                  }
                }
                if col_idx < self.grid[row_idx].len() - 1 {
                  if let Some(part_number) = self.check_for_part_number(col_idx + 1, row_idx + 1) {
                    gears.push(part_number)
                  }
                }
              }
            }
          }

          if gears.len() == 2 {
            gear_ratios.push(gears[0] * gears[1]);
          }
        }
      }
    }

    gear_ratios
  }

  fn check_for_part_number(&self, x: usize, y: usize) -> Option<u32> {
    if x >= self.grid[y].len() {
      // out of bounds
      return None;
    }

    if self.grid[y][x].is_numeric() {
      let value = self.find_surrounding_number(x, y).unwrap();
      return Some(value);
    }
    None
  }

  fn find_surrounding_number(&self, x: usize, y: usize) -> Option<u32> {
    if !self.grid[y][x].is_numeric() {
      // no surrounding number if first loc is not a number
      return None;
    }

    let mut i = 0;
    while x - i > 0 {
      if !self.grid[y][x - i].is_numeric() {
        i -= 1; // go back to last numeral
        break;
      }

      i += 1;
    }
    // awful, handling the off-by-one case where x-i is 0 but we don't know if 0 is a numeral yet
    let left = if self.grid[y][x - i].is_numeric() {
      x - i
    } else {
      x - i + 1
    };

    let mut i = 0;
    while x + i < self.grid[y].len() - 1 {
      if !self.grid[y][x + i].is_numeric() {
        i -= 1; // go back to last numeral
        break;
      }

      i += 1;
    }
    // awful, handling the off-by-one case where x+i is the final index but we don't know if the final index is a numeral yet
    let right = if self.grid[y][x + i].is_numeric() {
      x + i
    } else {
      x + i - 1
    };

    let res = self.grid[y][left..=right].iter().collect::<String>();
    println!("{res}");
    let res2 = res.parse().unwrap();
    Some(res2)
  }
}

#[cfg(test)]
mod day03_tests {
  use super::*;

  #[test]
  fn find_part_numbers() {
    let input = r"12..34
$.....
.56...";
    let schematic = Schematic::new(&input.as_bytes().to_vec());
    let expected_parts: Vec<u32> = vec![12, 56];
    assert_eq!(schematic.find_part_numbers(), expected_parts);
  }

  #[test]
  fn find_gear_ratios() {
    let input = r"12..34
*.....
.56...";
    let schematic = Schematic::new(&input.as_bytes().to_vec());
    let expected_parts: Vec<u32> = vec![12 * 56];
    assert_eq!(schematic.find_gear_ratios(), expected_parts);
  }
}
