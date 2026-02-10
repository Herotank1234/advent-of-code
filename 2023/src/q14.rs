use std::collections::HashMap;
use std::fs;

fn roll_north(grid: &mut Vec<Vec<char>>) {
  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      if grid[i][j] == 'O' {
        let mut curr_i = i;
        while curr_i > 0 && grid[curr_i - 1][j] == '.' {
          grid[curr_i][j] = '.';
          grid[curr_i - 1][j] = 'O';
          curr_i -= 1;
        }
      }
    }
  }
}

fn roll_west(grid: &mut Vec<Vec<char>>) {
  for j in 0..grid[0].len() {
    for i in 0..grid.len() {
      if grid[i][j] == 'O' {
        let mut curr_j = j;
        while curr_j > 0 && grid[i][curr_j - 1] == '.' {
          grid[i][curr_j] = '.';
          grid[i][curr_j - 1] = 'O';
          curr_j -= 1;
        }
      }
    }
  }
}

fn roll_south(grid: &mut Vec<Vec<char>>) {
  for i in (0..grid.len()).rev() {
    for j in 0..grid[0].len() {
      if grid[i][j] == 'O' {
        let mut curr_i = i;
        while curr_i < grid.len() - 1 && grid[curr_i + 1][j] == '.' {
          grid[curr_i][j] = '.';
          grid[curr_i + 1][j] = 'O';
          curr_i += 1;
        }
      }
    }
  }
}

fn roll_east(grid: &mut Vec<Vec<char>>) {
  for j in (0..grid[0].len()).rev() {
    for i in 0..grid.len() {
      if grid[i][j] == 'O' {
        let mut curr_j = j;
        while curr_j < grid[0].len() - 1 && grid[i][curr_j + 1] == '.' {
          grid[i][curr_j] = '.';
          grid[i][curr_j + 1] = 'O';
          curr_j += 1;
        }
      }
    }
  }
}

fn cycle(grid: &mut Vec<Vec<char>>) {
  roll_north(grid);
  roll_west(grid);
  roll_south(grid);
  roll_east(grid);
}

fn get_hash(grid: &Vec<Vec<char>>) -> String {
  let row_strs = grid.iter().map(|row| row.iter().collect::<String>())
    .collect::<Vec<String>>();
  return row_strs.iter().fold("".to_string(), |acc, s| acc + s);
}

fn calculate_load(grid: &Vec<Vec<char>>) -> u32 {
  let mut load = 0;
  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      if grid[i][j] == 'O' { load += (grid.len() - i) as u32; }
    }
  }
  return load;
}

fn roll_north_and_calculate_load(grid: &Vec<Vec<char>>) -> u32 {
  let mut curr_grid = grid.clone();
  roll_north(&mut curr_grid);
  return calculate_load(&curr_grid);
}

fn cycle_and_calculate_load(grid: &Vec<Vec<char>>) -> u32 {
  let mut curr_grid = grid.clone();
  let mut cycles: u32 = 0;
  let mut hash = get_hash(&curr_grid);
  let mut visited = HashMap::new();

  while !visited.contains_key(&hash) {
    visited.insert(hash.clone(), cycles);
    cycle(&mut curr_grid);
    hash = get_hash(&curr_grid);
    cycles += 1;
  }

  let first_visited = visited.get(&hash).unwrap();
  let cycle_length = cycles - first_visited;
  let cycles_remaining = (1000000000 - first_visited) % cycle_length;
  
  for _ in 0..cycles_remaining {
    cycle(&mut curr_grid);
  }
  return calculate_load(&curr_grid);
}

fn main() {
  let contents = fs::read_to_string("./data/q14.txt")
    .expect("Should have been able to read file");
  let grid = contents.split('\n').map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();
  println!("Part 1: {}", roll_north_and_calculate_load(&grid));
  println!("Part 2: {}", cycle_and_calculate_load(&grid));
}