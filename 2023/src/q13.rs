use std::cmp::min;
use std::fs;

fn is_reflection_col(grid: &Vec<Vec<char>>, j: usize) -> bool {
  let min_dist = min(j + 1, grid[0].len() - 1 - j);
  for i in 0..grid.len() {
    for dj in 0..min_dist {
      if grid[i][j - dj] != grid[i][j + 1 + dj] { return false; }
    }
  }
  return true;
}

fn is_reflection_row(grid: &Vec<Vec<char>>, i: usize) -> bool {
  let min_dist = min(i + 1, grid.len() - 1 - i);
  for j in 0..grid[0].len() {
    for di in 0..min_dist {
      if grid[i - di][j] != grid[i + 1 + di][j] { return false; }
    }
  }
  return true;
}

fn sum_reflection_vals(grids: &Vec<Vec<Vec<char>>>) -> usize {
  let mut total = 0;
  for grid in grids {
    let mut found = false;
    for j in 0..grid[0].len() - 1 {
      if is_reflection_col(grid, j) { 
        total += j + 1;
        found = true;
        break;
      }
    }
    if found { continue; }
    for i in 0..grid.len() - 1 {
      if is_reflection_row(grid, i) {
        total += (i + 1) * 100;
        break;
      }
    }
  }
  return total;
}

fn has_one_smudge_on_reflection_col(grid: &Vec<Vec<char>>, j: usize) -> bool {
  let min_dist = min(j + 1, grid[0].len() - 1 - j);
  let mut found = false;
  for i in 0..grid.len() {
    for dj in 0..min_dist {
      if grid[i][j - dj] != grid[i][j + 1 + dj] { 
        if !found {
          found = true;
        } else {
          return false;
        }
      }
    }
  }
  return found;
}

fn has_one_smudge_on_reflection_row(grid: &Vec<Vec<char>>, i: usize) -> bool {
  let min_dist = min(i + 1, grid.len() - 1 - i);
  let mut found = false;
  for j in 0..grid[0].len() {
    for di in 0..min_dist {
      if grid[i - di][j] != grid[i + 1 + di][j] {
        if !found {
          found = true;
        } else {
          return false; 
        }
      }
    }
  }
  return found;
}

fn sum_reflection_vals_with_smudge(grids: &Vec<Vec<Vec<char>>>) -> usize {
  let mut total = 0;
  for grid in grids {
    let mut found = false;
    for j in 0..grid[0].len() - 1 {
      if has_one_smudge_on_reflection_col(grid, j) { 
        total += j + 1;
        found = true;
        break;
      }
    }
    if found { continue; }
    for i in 0..grid.len() - 1 {
      if has_one_smudge_on_reflection_row(grid, i) {
        total += (i + 1) * 100;
        break;
      }
    }
  }
  return total;
}

fn main() {
  let contents = fs::read_to_string("./data/q13.txt")
    .expect("Should have been able to read file");
  let lines = contents.split('\n').collect::<Vec<&str>>();
  let mut grids = Vec::new();
  let mut curr_grid = Vec::new();
  for line in lines {
    if line == "" {
      grids.push(curr_grid.clone());
      curr_grid.clear();
    } else {
      curr_grid.push(line.chars().collect::<Vec<char>>());
    }
  }
  grids.push(curr_grid);

  println!("Part 1: {}", sum_reflection_vals(&grids));
  println!("Part 2: {}", sum_reflection_vals_with_smudge(&grids));
}