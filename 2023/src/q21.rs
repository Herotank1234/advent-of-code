use std::fs;
use std::collections::HashSet;

const DIRS: &[(isize, isize)]= &[
  (-1, 0),
  (0, 1),
  (1, 0),
  (0, -1)
];

fn find_start(grid: &mut Vec<Vec<char>>) -> (usize, usize) {
  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      if grid[i][j] == 'S' {
        grid[i][j] = '.';
        return (i, j);
      }
    }
  }
  return (grid.len(), grid[0].len());
}

fn is_in_bounds(i: isize, j: isize, h: isize, w: isize) -> bool {
  return i >= 0 && i < h && j >= 0 && j < w;
}

fn count_reachable_squares_after_steps(grid: &Vec<Vec<char>>, start: (usize, usize), steps: u32) -> usize {
  let h = grid.len() as isize;
  let w = grid[0].len() as isize;

  let mut curr_points = HashSet::new();
  let s_i = start.0 as isize;
  let s_j = start.1 as isize;
  curr_points.insert((s_i, s_j));

  for _ in 0..steps {
    let mut next_points = HashSet::new();
    for (i, j) in curr_points {
      for (di, dj) in DIRS.iter() {
        let next_i = i + di;
        let next_j = j + dj;
        if is_in_bounds(next_i, next_j, h, w) && grid[next_i as usize][next_j as usize] == '.' {
          next_points.insert((next_i, next_j));
        }
      }
    }
    curr_points = next_points;
  }

  return curr_points.len();
}

fn count_reachable_squares_big(grid: &Vec<Vec<char>>, start: (usize, usize)) -> u64 {
  let grid_width = grid.len() as u64;
  let steps: u64 = 26501365;

  let grids_width = (steps / grid_width) - 1;
  let odd_parity_grids = grids_width.pow(2);
  let odd_parity_tiles = count_reachable_squares_after_steps(grid, start, grid_width as u32) as u64;

  let even_parity_grids = (grids_width + 1).pow(2);
  let even_partiy_tiles = count_reachable_squares_after_steps(grid, start, grid_width as u32 + 1) as u64;

  let top = count_reachable_squares_after_steps(grid, (grid_width as usize - 1, start.1),
    grid_width as u32 - 1) as u64;
  let right = count_reachable_squares_after_steps(grid, (start.0, 0),
    grid_width as u32 - 1) as u64;
  let down = count_reachable_squares_after_steps(grid, (0, start.1),
    grid_width as u32 - 1) as u64;
  let left = count_reachable_squares_after_steps(grid, (start.0,grid_width as usize - 1),
    grid_width as u32 - 1) as u64;

  let top_right = count_reachable_squares_after_steps(grid, (grid_width as usize - 1, 0),
    grid_width as u32 / 2 - 1) as u64;
  let top_left = count_reachable_squares_after_steps(grid, (grid_width as usize - 1, grid_width as usize - 1),
    grid_width as u32 / 2 - 1) as u64;
  let bottom_right = count_reachable_squares_after_steps(grid, (0, 0),
    grid_width as u32 / 2 - 1) as u64;
  let bottom_left = count_reachable_squares_after_steps(grid, (0, grid_width as usize - 1),
    grid_width as u32 / 2 - 1) as u64;

  let large_top_right = count_reachable_squares_after_steps(grid, (grid_width as usize - 1, 0),
    grid_width as u32 * 3 / 2 - 1) as u64;
  let large_top_left = count_reachable_squares_after_steps(grid, (grid_width as usize - 1, grid_width as usize - 1),
    grid_width as u32 * 3 / 2 - 1) as u64;
  let large_bottom_right = count_reachable_squares_after_steps(grid, (0, 0),
    grid_width as u32 * 3 / 2 - 1) as u64;
  let large_bottom_left = count_reachable_squares_after_steps(grid, (0, grid_width as usize - 1),
    grid_width as u32 * 3 / 2 - 1) as u64;

  let mut total = 0;
  total += odd_parity_grids * odd_parity_tiles;
  total += even_parity_grids * even_partiy_tiles;
  total += top + right + down+ left;
  total += (grids_width + 1) * (top_right + top_left + bottom_right + bottom_left);
  total += grids_width * (large_top_left + large_top_right + large_bottom_left + large_bottom_right);
  return total;
}

fn main() {
  let contents = fs::read_to_string("./data/q21.txt")
    .expect("Should have been able to read file");

  let mut grid = contents.split('\n').map(|row| row.chars().collect()).collect::<Vec<Vec<char>>>();
  let start = find_start(&mut grid);
  
  println!("Part 1: {}", count_reachable_squares_after_steps(&grid, start, 64));
  println!("Part 2: {}", count_reachable_squares_big(&grid, start));
}