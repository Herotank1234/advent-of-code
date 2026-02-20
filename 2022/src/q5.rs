use std::fs;
use regex::Regex;

const DUMMY: usize = 1;
const CRATE_SIZE: usize = 4;

struct Move {
  count: u32,
  src: usize,
  dst: usize
}

fn parse_crates(lines: &[&str]) -> Vec<Vec<char>> {
  let num_crates = (lines[0].len() + 1) / CRATE_SIZE + DUMMY;
  let mut crates = vec![vec![]; num_crates];
  for line in lines.iter().rev().skip(1) {
    for i in (0..line.len()).step_by(CRATE_SIZE) {
      let curr_line = line.chars().collect::<Vec<char>>();
      if curr_line[i] == ' ' {
        continue;
      } else {
        crates[i / CRATE_SIZE + DUMMY].push(curr_line[i + 1]);
      }
    }
  }
  return crates;
}

fn simulate_9000(mut crates: Vec<Vec<char>>, moves: &Vec<Move>) -> String {
  for m in moves.iter() {
    for _ in 0..m.count {
      let c = crates[m.src].pop().unwrap();
      crates[m.dst].push(c);
    }
  }

  let mut tops = String::new();
  for c in crates.iter() {
    if !c.is_empty() { tops.push(*c.last().unwrap()); }
  }
  return tops;
}

fn simulate_9001(mut crates: Vec<Vec<char>>, moves: &Vec<Move>) -> String {
  for m in moves.iter() {
    let mut popped = Vec::new();
    for _ in 0..m.count {
      let c = crates[m.src].pop().unwrap();
      popped.push(c)
    }
    for _ in 0..m.count {
      let c = popped.pop().unwrap();
      crates[m.dst].push(c);
    }
  }

  let mut tops = String::new();
  for c in crates.iter() {
    if !c.is_empty() { tops.push(*c.last().unwrap()); }
  }
  return tops;
}

fn main() {
  let contents = fs::read_to_string("./data/q5.txt")
    .expect("Should have been able to read file");
  let lines = contents.split('\n').collect::<Vec<&str>>();

  let mut empty_index = 0;
  for i in 0..lines.len() {
    if lines[i] == "" {
      empty_index = i;
      break;
    }
  }

  let crate_lines = &lines[..empty_index];
  let move_lines = &lines[empty_index + 1..];
  let re = Regex::new(r"move ([\d]+) from ([\d]+) to ([\d]+)").unwrap();

  let crates = parse_crates(crate_lines);
  let moves = move_lines.iter().map(|line| {
    let caps = re.captures(*line).unwrap();
    return Move{
      count: caps[1].to_string().parse::<u32>().unwrap(),
      src: caps[2].to_string().parse::<usize>().unwrap(),
      dst: caps[3].to_string().parse::<usize>().unwrap()
    };
  }).collect::<Vec<Move>>();
  println!("Part 1: {}", simulate_9000(crates.clone(), &moves));
  println!("Part 2: {}", simulate_9001(crates.clone(), &moves));
}