use day04_lib::solve;

fn main() {
  let input =
    std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/input.txt")).unwrap();
  let part_1_solution = solve(&input);
  println!("{part_1_solution}");
}
