use std::fs;

struct Assignment {
  l_lower: u32,
  l_upper: u32,
  r_lower: u32,
  r_upper: u32
}

fn count_enveloping_assignments(assignments: &Vec<Assignment>) -> u32 {
  return assignments.iter().map(|a| {
    return (a.r_lower >= a.l_lower && a.r_upper <= a.l_upper || 
      a.l_lower >= a.r_lower && a.l_upper <= a.r_upper) as u32;
  }).sum();
}

fn count_overlapping_assignments(assignments: &Vec<Assignment>) -> u32 {
  return assignments.iter().map(|a| {
    return (!(a.l_upper < a.r_lower || a.r_upper < a.l_lower)) as u32;
  }).sum();
}

fn main() {
  let contents = fs::read_to_string("./data/q4.txt")
    .expect("Should have been able to read file");
  let assignments = contents.split('\n').map(|line| {
    let parts = line.split(',').collect::<Vec<&str>>();
    let left = parts[0].split('-').map(|part| part.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let right = parts[1].split('-').map(|part| part.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    return Assignment{l_lower: left[0], l_upper: left[1], r_lower: right[0], r_upper: right[1]};
  }).collect::<Vec<Assignment>>();

  println!("Part 1: {}", count_enveloping_assignments(&assignments));
  println!("Part 2: {}", count_overlapping_assignments(&assignments));
}