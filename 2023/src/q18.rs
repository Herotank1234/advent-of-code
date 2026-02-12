use std::fs;
use regex::Regex;
use phf::phf_map;

#[derive(Debug)]
struct Instruction {
  dir: char,
  steps: i64,
  color: String
}

const VAL_TO_CHAR_DIR: phf::Map<char, char> = phf_map! {
  '0' => 'R',
  '1' => 'D',
  '2' => 'L',
  '3' => 'U'
};

const CHAR_DIR_TO_DIR: phf::Map<char, (i64, i64)> = phf_map! {
    'U' => (-1, 0),
    'R' => (0, 1),
    'D' => (1, 0),
    'L' => (0, -1)
};

fn find_lagoon_area(instructions: &Vec<Instruction>) -> u64 {
  let mut vertices = Vec::new();
  let mut i = 0;
  let mut j = 0;
  vertices.push((i, j));

  let mut trench_length = 0;
  for instruction in instructions {
    let (di, dj) = CHAR_DIR_TO_DIR.get(&instruction.dir).unwrap();
    i += *di * instruction.steps;
    j += *dj * instruction.steps;
    trench_length += instruction.steps;
    vertices.push((i, j));
  }

  let mut left_sum = 0;
  let mut right_sum = 0;
  for i in 0.. vertices.len() - 1 {
    let (i1, j1) = &vertices[i];
    let (i2, j2) = &vertices[i + 1];
    left_sum += j1 * i2;
    right_sum += j2 * i1;
  }

  let total_area = (i64::abs(left_sum - right_sum) / 2) + (trench_length / 2) + 1;
  return total_area as u64;
}

fn reinterpret_and_find_lagoon_area(instructions: &mut Vec<Instruction>) -> u64 {
  for instruction in instructions.iter_mut() {
    let dir = &instruction.color.chars().nth(5).unwrap();
    let steps = &instruction.color[0..5];
    instruction.dir = *VAL_TO_CHAR_DIR.get(&dir).unwrap();
    instruction.steps = i64::from_str_radix(steps, 16).unwrap();
  }
  return find_lagoon_area(instructions);
}

fn main() {
  let contents = fs::read_to_string("./data/q18.txt")
    .expect("Should have been able to read file");
  let lines = contents.split('\n').collect::<Vec<&str>>();
  let re = Regex::new(r"(U|R|D|L) ([\d]+) \(#([0-9a-f]{6})\)").unwrap();
  let mut instructions = lines.iter().map(|line| {
    let caps = re.captures(line).unwrap();
    return Instruction{
      dir: caps[1].to_string().chars().nth(0).unwrap(), 
      steps: caps[2].to_string().parse::<i64>().unwrap(), 
      color: caps[3].to_string()
    };
  }).collect::<Vec<Instruction>>();

  println!("Part 1: {}", find_lagoon_area(&instructions));
  println!("Part 2: {}", reinterpret_and_find_lagoon_area(&mut instructions));
}