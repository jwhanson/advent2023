use day04_lib::{solve, solve_2};

fn main() {
  let input =
    std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/input.txt")).unwrap();
  let part_1_solution = solve(&input);
  let part_2_solution = solve_2(&input);
  println!("p1: {part_1_solution}");
  println!("p2: {part_2_solution}");
}
