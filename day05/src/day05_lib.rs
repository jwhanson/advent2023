use nom;

pub mod day05_parsing;

struct Almanac {
  seeds: Vec<u32>,
  maps: Vec<AlmanacMap>,
}

struct AlmanacMap {
  src_id: String,
  dst_id: String,
  ranges: Vec<AlmanacRange>,
}

struct AlmanacRange {
  src_start: u32,
  dst_start: u32,
  length: u32,
}

pub fn solve(input: &str) -> u32 {
  let almanac = parse(&input);

  0
}

fn parse(input: &str) -> () {
  let res = day05_parsing::parse_input(input).unwrap();
  // println!("{:?}", res);
  ()
}

#[cfg(test)]
mod day05_tests {
  use super::*;

  #[test]
  fn check_parse_almanac() {
    let input =
      std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/example.txt")).unwrap();
    parse(&input);
    assert!(true);
  }
}
