use std::fs;
use std::collections::{HashSet, BinaryHeap};
use std::cmp::Ordering;

#[derive(Hash, PartialEq, Eq, Debug)]
struct State {
  i: usize,
  j: usize,
  di: isize,
  dj: isize,
  steps: u32,
  heat_loss: u32
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> Ordering {
    return other.heat_loss.cmp(&self.heat_loss);
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    return Some(self.cmp(other));
  }
}

const DIRS: &[(isize, isize)] = &[
  (-1, 0),
  (0, 1),
  (1, 0),
  (0, -1)
];

fn is_in_bounds(i: isize, j: isize, h: isize, w: isize) -> bool {
  return i >= 0 && i < h && j >= 0 && j < w;
}

fn get_min_heat_loss(grid: &Vec<Vec<u32>>) -> u32 {
  let h = grid.len() as isize;
  let w = grid[0].len() as isize;
  let end_i = h as usize - 1;
  let end_j = w as usize - 1;
  let mut visited = HashSet::new();
  let mut to_be_visited = BinaryHeap::new();
  to_be_visited.push(State{i: 0, j: 0, di: 0, dj: 0, steps: 0, heat_loss: 0});


  while !to_be_visited.is_empty() {
    let State{i, j, di, dj, steps, heat_loss} = to_be_visited.pop().unwrap();
    if i == end_i && j == end_j { return heat_loss; }

    if visited.contains(&(i, j, di, dj, steps)) { continue; }
    visited.insert((i, j, di, dj, steps));

    for (next_di, next_dj) in DIRS {
      if (di as isize + *next_di == 0) && (dj as isize + *next_dj == 0) { continue; }
      let next_i = i as isize + next_di;
      let next_j = j as isize + next_dj;
      if !is_in_bounds(next_i, next_j, h, w) { continue; }
      let next_hl = heat_loss + grid[next_i as usize][next_j as usize];

      if di == *next_di && dj == *next_dj {
        if steps == 3 { continue; }
        let next_state = State{i: next_i as usize, j: next_j as usize, di, dj, steps: steps + 1, heat_loss: next_hl};
        to_be_visited.push(next_state);
      } else {
        let next_state = State{i: next_i as usize, j: next_j as usize, di: *next_di, dj: *next_dj, steps: 1, heat_loss: next_hl};
        to_be_visited.push(next_state);
      }
    }
  }
  return 0;
}

fn get_min_heat_loss_ultra(grid: &Vec<Vec<u32>>) -> u32 {
  let h = grid.len() as isize;
  let w = grid[0].len() as isize;
  let end_i = h as usize - 1;
  let end_j = w as usize - 1;
  let mut visited = HashSet::new();
  let mut to_be_visited = BinaryHeap::new();
  to_be_visited.push(State{i: 0, j: 0, di: 0, dj: 0, steps: 0, heat_loss: 0});

  while !to_be_visited.is_empty() {
    let State{i, j, di, dj, steps, heat_loss} = to_be_visited.pop().unwrap();
    if i == end_i && j == end_j { return heat_loss; }

    if visited.contains(&(i, j, di, dj, steps)) { continue; }
    visited.insert((i, j, di, dj, steps));

    for (next_di, next_dj) in DIRS {
      if (di as isize + *next_di == 0) && (dj as isize + *next_dj == 0) { continue; }
      if di == *next_di && dj == *next_dj { continue; }
      let mut next_i = i as isize;
      let mut next_j = j as isize;
      let mut next_hl = heat_loss;
      for step_in_dir in 0..10 {
        next_i += next_di;
        next_j += next_dj;
        if !is_in_bounds(next_i, next_j, h, w) { break; }
        next_hl += grid[next_i as usize][next_j as usize];
        if step_in_dir < 3 { continue; }

        let next_state = State{i: next_i as usize, j: next_j as usize, di: *next_di, dj: *next_dj, steps: step_in_dir, heat_loss: next_hl};
        to_be_visited.push(next_state);
      }
    }
  }
  return 0;
}

fn main() {
  let contents = fs::read_to_string("./data/q17.txt")
    .expect("Should have been able to read file");
  let grid = contents.split('\n').map(|line|
    line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
  ).collect::<Vec<Vec<u32>>>();

  println!("Part 1: {}", get_min_heat_loss(&grid));
  println!("Part 2: {}", get_min_heat_loss_ultra(&grid));
}