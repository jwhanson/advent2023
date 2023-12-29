use day05_lib;

fn main() {
  let input =
    std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/input.txt")).unwrap();
  let part_1_solution = day05_lib::solve(&input);
  println!("p1: {part_1_solution}");
}
