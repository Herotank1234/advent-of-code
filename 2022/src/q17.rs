use std::{cmp::max, collections::{HashMap, HashSet}, fs};

#[derive(Clone, Debug)]
enum Dir {
  Left,
  Right
}
 
struct AirJetGenerator {
  air_moves: Vec<Dir>,
  index: usize
}

impl AirJetGenerator {
  fn new(air_moves: Vec<Dir>) -> Self {
    return AirJetGenerator{air_moves, index: 0};
  }

  fn next(&mut self) -> &Dir {
    let air_move = &self.air_moves[self.index];
    self.index += 1;
    if self.index == self.air_moves.len() { self.index = 0; }
    return air_move;
  }

  fn get_index(&self) -> usize {
    return self.index;
  }
}

struct RockGenerator {
  index: u32
}

impl RockGenerator {
  fn new() -> Self {
    return RockGenerator{index: 0};
  }

  fn next(&mut self, i: u32) -> Vec<(u32, u32)> {
    let rock = match self.index {
      0 => vec![(i, 3), (i, 4), (i, 5), (i, 6)],
      1 => vec![(i, 4), (i + 1, 3), (i + 1, 4), (i + 1, 5), (i + 2, 4)],
      2 => vec![(i, 3), (i, 4), (i, 5), (i + 1, 5), (i + 2, 5)],
      3 => vec![(i, 3), (i + 1, 3), (i + 2, 3), (i + 3, 3)],
      4 => vec![(i, 3), (i, 4), (i + 1, 3), (i + 1, 4)],
      _ => panic!("Rock Generator index not recognised: {}", self.index)
    };
    self.index += 1;
    if self.index == 5 { self.index = 0; }
    return rock;
  }
}

fn is_moveable_horizontal(rocks: &HashSet<(u32, u32)>, rock: &Vec<(u32, u32)>, dir: &Dir) -> bool {
  for (i, j) in rock.iter() {
    match dir {
      Dir::Left => {
        if *j == 1 { return false; }
        if rocks.contains(&(*i, *j - 1)) { return false; }
      },
      Dir::Right => {
        if *j == 7 { return false; }
        if rocks.contains(&(*i, *j + 1)) { return false; }
      }
    }
  }
  return true;
}

fn is_moveable_down(rocks: &HashSet<(u32, u32)>, rock: &Vec<(u32, u32)>) -> bool {
  for (i, j) in rock.iter() {
    if *i == 1 { return false; }
    if rocks.contains(&(*i - 1, *j)) { return false; }
  }
  return true;
}

fn simulate(air_gen: &mut AirJetGenerator, rock_gen: &mut RockGenerator,
  rocks: &mut HashSet<(u32, u32)>, highest_i: &mut u32, hash: &mut String)
{
  let mut rock = rock_gen.next(*highest_i + 4);
  let mut moveable = true;
  hash.push_str(&air_gen.get_index().to_string());
  hash.push(',');

  while moveable {
    let air_dir = air_gen.next();
    if is_moveable_horizontal(&rocks, &rock, air_dir) {
      match air_dir {
        Dir::Left => {
          rock.iter_mut().for_each(|(_, j)| *j -= 1);
          hash.push('<');
        },
        Dir::Right => {
          rock.iter_mut().for_each(|(_, j)| *j += 1);
          hash.push('>');
        }
      }
    }

    if is_moveable_down(&rocks, &rock) {
      rock.iter_mut().for_each(|(i, _)| *i -= 1);
      hash.push('v');
    } else {
      moveable = false;
    }
  }

  hash.push(',');

  *highest_i = max(*highest_i, rock.last().unwrap().0);
  for pos in rock {
    rocks.insert(pos);
  }
}

fn simulate_small(air_moves: &Vec<Dir>) -> u32 {
  let mut air_gen = AirJetGenerator::new(air_moves.clone());
  let mut rock_gen = RockGenerator::new();
  let mut rocks = HashSet::new();
  let mut highest_i = 0;
  for _ in 0..2022 {
    simulate(&mut air_gen, &mut rock_gen, &mut rocks, &mut highest_i, &mut String::from(""));
  }
  return highest_i;
}

fn simulate_big(air_moves: &Vec<Dir>) -> u64 {
  let mut air_gen = AirJetGenerator::new(air_moves.clone());
  let mut rock_gen = RockGenerator::new();
  let mut rocks = HashSet::new();
  let mut highest_i = 0;

  let mut hashes = HashMap::new();
  let mut seen = false;
  let mut rocks_fallen = 0;
  let mut hash = String::new();

  let mut prev_highest_i = highest_i;
  let mut prev_rocks_fallen = rocks_fallen;

  while !seen {
    simulate(&mut air_gen, &mut rock_gen, &mut rocks, &mut highest_i, &mut hash);
    rocks_fallen += 1;
    if rocks_fallen % 5 == 0 {
      if hashes.contains_key(&hash) {
        seen = true;
      } else {
        hashes.insert(hash.clone(), (prev_rocks_fallen, prev_highest_i));
        prev_highest_i = highest_i;
        prev_rocks_fallen = rocks_fallen;
        hash.clear();
      }
    }
  }

  let (first_rocks_fallen, first_highest_i) = hashes.get(&hash).unwrap();

  let cycle_length = prev_rocks_fallen - first_rocks_fallen;
  let cycle_height = (prev_highest_i - first_highest_i) as u64;

  let total_cycles: u64 = 1_000_000_000_000;
  let cycles = (total_cycles - first_rocks_fallen) / cycle_length;
  let rem_cycles = ((total_cycles - first_rocks_fallen) % cycle_length) - 5;

  for _ in 0..rem_cycles {
    simulate(&mut air_gen, &mut rock_gen, &mut rocks, &mut highest_i, &mut hash);
  }
  let rem_height = (highest_i - prev_highest_i) as u64;

  return *first_highest_i as u64 + cycles * cycle_height + rem_height;
}

fn main() {
  let contents = fs::read_to_string("./data/q17.txt")
    .expect("Should have been able to read file");
  let air_moves = contents.chars().map(|c| {
    if c == '<' { Dir::Left } else { Dir::Right }
  }).collect::<Vec<Dir>>();

  println!("Part 1: {}", simulate_small(&air_moves));
  println!("Part 2: {}", simulate_big(&air_moves));
}