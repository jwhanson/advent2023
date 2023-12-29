use nom::{
  bytes::complete::tag,
  character::complete::{alpha1, line_ending, multispace1, not_line_ending, space0, space1, u32},
  combinator::{all_consuming, map},
  multi::separated_list1,
  sequence::{delimited, separated_pair, terminated, tuple},
  IResult,
};

fn parse_map_header(input: &str) -> IResult<&str, (&str, &str)> {
  terminated(separated_pair(alpha1, tag("-to-"), alpha1), not_line_ending)(input)
}

fn parse_map_row(input: &str) -> IResult<&str, (u32, u32, u32)> {
  tuple((
    delimited(space0, u32, space0),
    delimited(space0, u32, space0),
    delimited(space0, u32, space0),
  ))(input)
}

fn parse_map(input: &str) -> IResult<&str, (&str, &str, Vec<(u32, u32, u32)>)> {
  map(
    separated_pair(
      parse_map_header,
      line_ending,
      separated_list1(line_ending, parse_map_row),
    ),
    |((dst_name, src_name), rows)| (dst_name, src_name, rows),
  )(input)
}

fn parse_seeds(input: &str) -> IResult<&str, (&str, Vec<u32>)> {
  separated_pair(
    alpha1,
    tag(":"),
    delimited(space0, separated_list1(space1, u32), space0),
  )(input)
}

pub fn parse_input(
  input: &str,
) -> IResult<&str, ((&str, Vec<u32>), Vec<(&str, &str, Vec<(u32, u32, u32)>)>)> {
  all_consuming(separated_pair(
    parse_seeds,
    multispace1,
    separated_list1(multispace1, parse_map),
  ))(input)
}

#[cfg(test)]
mod day05_tests {
  use super::*;

  #[test]
  fn check_seeds() {
    let input = "seeds: 79 14 55 13";
    let (remainder, (name, values)) = parse_seeds(input).unwrap();
    assert_eq!(name, "seeds");
    assert_eq!(values, [79, 14, 55, 13]);
    assert_eq!(remainder, "");
  }

  #[test]
  fn check_map_header() {
    let input = "seed-to-soil map:";
    let (remainder, (dst, src)) = parse_map_header(input).unwrap();
    assert_eq!(dst, "seed");
    assert_eq!(src, "soil");
    assert_eq!(remainder, "");
  }

  #[test]
  fn check_map() {
    let input = r"seed-to-soil map:
50 98 2
52 50 48";
    let (remainder, (dst_name, src_name, rows)) = parse_map(input).unwrap();
    let expected_rows = vec![(50, 98, 2), (52, 50, 48)];
    assert_eq!(dst_name, "seed");
    assert_eq!(src_name, "soil");
    assert_eq!(rows, expected_rows);
    assert_eq!(remainder, "");
  }

  #[test]
  fn check_input() {
    let input =
      std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/example.txt")).unwrap();
    let (remainder, ((name, values), maps)) = parse_input(&input).unwrap();
    let expected_name = "seeds";
    let expected_values = vec![79, 14, 55, 13];
    let map_a = ("seed", "soil", vec![(50, 98, 2), (52, 50, 48)]);
    let map_b = (
      "soil",
      "fertilizer",
      vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)],
    );
    let map_c = (
      "fertilizer",
      "water",
      vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)],
    );
    let map_d = ("water", "light", vec![(88, 18, 7), (18, 25, 70)]);
    let map_e = (
      "light",
      "temperature",
      vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)],
    );
    let map_f = ("temperature", "humidity", vec![(0, 69, 1), (1, 0, 69)]);
    let map_g = ("humidity", "location", vec![(60, 56, 37), (56, 93, 4)]);

    assert_eq!(expected_name, name);
    assert_eq!(expected_values, values);
    assert_eq!(map_a, maps[0]);
    assert_eq!(map_b, maps[1]);
    assert_eq!(map_c, maps[2]);
    assert_eq!(map_d, maps[3]);
    assert_eq!(map_e, maps[4]);
    assert_eq!(map_f, maps[5]);
    assert_eq!(map_g, maps[6]);
  }
}
