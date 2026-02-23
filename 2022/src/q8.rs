use std::{
  cmp::max,
  fs
};

fn row_visible_from_left(grid: &Vec<Vec<u32>>, visible: &mut Vec<Vec<bool>>, row: usize) {
  let w = grid[0].len();
  let mut max_height = grid[row][0];
  for j in 1..w - 1 {
    if grid[row][j] > max_height { 
      visible[row][j] = true;
      max_height = grid[row][j];
    }
  }
}

fn row_visible_from_right(grid: &Vec<Vec<u32>>, visible: &mut Vec<Vec<bool>>, row: usize) {
  let w = grid[0].len();
  let mut max_height = grid[row][w - 1];
  for j in (1..w - 1).rev() {
    if grid[row][j] > max_height { 
      visible[row][j] = true;
      max_height = grid[row][j];
    }
  }
}

fn row_visible_from_up(grid: &Vec<Vec<u32>>, visible: &mut Vec<Vec<bool>>, col: usize) {
  let h = grid.len();
  let mut max_height = grid[0][col];
  for i in 1..h - 1 {
    if grid[i][col] > max_height { 
      visible[i][col] = true;
      max_height = grid[i][col];
    }
  }
}

fn row_visible_from_down(grid: &Vec<Vec<u32>>, visible: &mut Vec<Vec<bool>>, col: usize) {
  let h = grid.len();
  let mut max_height = grid[h - 1][col];
  for i in (1..h - 1).rev() {
    if grid[i][col] > max_height { 
      visible[i][col] = true;
      max_height = grid[i][col];
    }
  }
}

fn count_visible_tree(grid: &Vec<Vec<u32>>) -> u32 {
  let h = grid.len();
  let w = grid[0].len();
  let mut visible = vec![vec![false; w]; h];

  for j in 0..w { visible[0][j] = true; }
  for j in 0..w { visible[h - 1][j] = true; }
  for i in 1..h - 1 { visible[i][0] = true; }
  for i in 1..h - 1 { visible[i][w - 1] = true; }
  
  for i in 1..h - 1 {
    row_visible_from_left(grid, &mut visible, i);
    row_visible_from_right(grid, &mut visible, i);
  }

  for j in 1..w - 1 {
    row_visible_from_up(grid, &mut visible, j);
    row_visible_from_down(grid, &mut visible, j);
  } 

  return visible.iter().flatten().map(|val| *val as u32).sum();
}

fn get_scenic_score(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
  let h = grid.len();
  let w = grid[0].len();

  let mut right_score = 1;
  let mut down_score = 1;
  let mut left_score = 1;
  let mut up_score = 1;
  let max_height = grid[row][col];

  let mut i = row;
  let mut j = col;
  while j < w - 2 && grid[i][j + 1] < max_height {
    right_score += 1;
    j += 1;
  }

  i = row;
  j = col;
  while i < h - 2 && grid[i + 1][j] < max_height {
    down_score += 1;
    i += 1;
  }

  i = row;
  j = col;
  while j > 1 && grid[i][j - 1] < max_height {
    left_score += 1;
    j -= 1;
  }
  
  i = row;
  j = col;
  while i > 1 && grid[i - 1][j] < max_height {
    up_score += 1;
    i -= 1;
  }

  return right_score * down_score * left_score * up_score;
}

fn get_max_scenic_score(grid: &Vec<Vec<u32>>) -> u32 {
  let h = grid.len();
  let w = grid[0].len();
  let mut max_scenic_score = 0;

  for i in 1..h - 1 {
    for j in 1..w - 1 {
      max_scenic_score = max(max_scenic_score, get_scenic_score(grid, i, j));
    }
  }

  return max_scenic_score;
}

fn main() {
  let contents = fs::read_to_string("./data/q8.txt")
    .expect("Should have been able to read file");
  let grid = contents.split('\n').map(|line| line.chars().map(|val|
    val.to_digit(10).unwrap()).collect()).collect::<Vec<Vec<u32>>>();

  println!("Part 1: {}", count_visible_tree(&grid));
  println!("Part 2: {}", get_max_scenic_score(&grid));
}