use std::fs;
use std::cmp::max;

fn get_max_calories(groups: &Vec<Vec<u32>>) -> u32 {
  return groups.iter().fold(0, |acc, s| max(acc, s.iter().sum()));
}

fn get_sum_top_three_calories(groups: &Vec<Vec<u32>>) -> u32 {
  let mut sums = groups.iter().map(|group| group.iter().sum()).collect::<Vec<u32>>();
  sums.sort_by(|a, b| b.cmp(a));
  return sums.iter().take(3).sum();
}

fn main() {
  let contents = fs::read_to_string("./data/q1.txt")
    .expect("Should have been able to read file");
  let lines = contents.split('\n').collect::<Vec<&str>>();
  let mut groups = Vec::new();
  let mut curr_group = Vec::new();
  for line in lines.iter() {
    if *line == "" { 
      groups.push(curr_group.clone());
      curr_group.clear();
    } else {
      curr_group.push(line.parse::<u32>().unwrap());
    }
  }

  println!("Part 1: {}", get_max_calories(&groups));
  println!("Part 2: {}", get_sum_top_three_calories(&groups));
}