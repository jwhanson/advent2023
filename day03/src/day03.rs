use std::fs::File;
use std::io::{BufReader, Read};

use day03_lib::Schematic;

fn main() {
  let file = File::open("day03/assets/example.txt").unwrap();
  let mut reader = BufReader::new(file);
  let mut buffer: Vec<u8> = vec![];
  let res = reader.read_to_end(&mut buffer);

  match res {
    Ok(_) => {
      let schematic = Schematic::new(&buffer);
      let parts = schematic.find_part_numbers();
      println!("{}", parts.iter().sum::<u32>());
    }
    Err(err) => panic!("failed to read file to end: {}", err),
  }
}
