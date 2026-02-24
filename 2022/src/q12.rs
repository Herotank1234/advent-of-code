use std::{collections::VecDeque, fs, u32, cmp::min};

const DIRS: &[(isize, isize); 4] = &[
  (-1, 0),
  (0, 1),
  (1, 0),
  (0, -1)
];

fn is_in_bounds(i: isize, j: isize, h: isize, w: isize) -> bool {
  return i >= 0 && i < h && j >= 0 && j < w;
}

fn get_shortest_path(grid: &Vec<Vec<char>>, start: &(usize, usize), 
  end: &(usize, usize)) -> u32 
{
  let h = grid.len() as isize;
  let w = grid[0].len() as isize;

  let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
  let mut to_be_vistied = VecDeque::new();
  to_be_vistied.push_back((*start, 0));

  while !to_be_vistied.is_empty() {
    let (pos@(i, j), steps) = to_be_vistied.pop_front().unwrap();
    if pos == *end { return steps; }

    if visited[i][j] { continue; }
    visited[i][j] = true;
    
    let curr_char = &grid[i][j];
    for (di, dj) in DIRS.iter() {
      let next_i = i as isize + di;
      let next_j = j as isize + dj;
      if !is_in_bounds(next_i, next_j, h, w) { continue; }
      if grid[next_i as usize][next_j as usize] as u32 > *curr_char as u32 + 1 { continue; }
      to_be_vistied.push_back(((next_i as usize, next_j as usize), steps + 1));
    }
  }

  return u32::MAX;
}

fn get_shortest_path_from_all_start(grid: &Vec<Vec<char>>, end: &(usize, usize)) -> u32 {
  let mut starts = Vec::new();
  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      if grid[i][j] == 'a' { starts.push((i, j)); }
    }
  }
  let mut shortest_path = u32::MAX;
  for start in starts.iter() {
    shortest_path = min(shortest_path, get_shortest_path(grid, start, end));
  }
  return shortest_path;
}
fn main() {
  let contents = fs::read_to_string("./data/q12.txt")
    .expect("Should have been able to read file");
  let mut grid = contents.split('\n').map(|line| line.chars().collect())
    .collect::<Vec<Vec<char>>>();

  let mut start = (0, 0);
  let mut end = (0, 0);

  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      if grid[i][j] == 'S' { 
        start = (i, j); 
        grid[i][j] = 'a'; 
      }
      if grid[i][j] == 'E' {
        end = (i, j); 
        grid[i][j] = 'z'; 
      }
    }
  }

  println!("Part 1: {}", get_shortest_path(&grid, &start, &end));
  println!("Part 2: {}", get_shortest_path_from_all_start(&grid, &end));
}