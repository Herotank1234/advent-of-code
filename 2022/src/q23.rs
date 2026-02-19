use std::fs;

fn main() {
  let _contents = fs::read_to_string("./data/q_.txt")
    .expect("Should have been able to read file");
  println!("Part 1: {}", 0);
  println!("Part 2: {}", 0);
}