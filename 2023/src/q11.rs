use std::fs;

fn get_galaxy_poss(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
  let mut galaxies = Vec::new();
  let h = grid.len();
  let w = grid[0].len();
  for i in 0..h {
    for j in 0..w {
      if grid[i][j] == '#' { galaxies.push((i, j)); }
    }
  }
  return galaxies;
}

fn is_row_clear(grid: &Vec<Vec<char>>, i: usize) -> bool {
  for c in 0..grid.len() {
    if grid[i][c] == '#' { return false; }
  }
  return true;
}

fn is_col_clear(grid: &Vec<Vec<char>>, i: usize) -> bool {
  for c in 0..grid.len() {
    if grid[c][i] == '#' { return false; }
  }
  return true;
}

fn expand_galaxies(grid: &Vec<Vec<char>>, galaxies: &Vec<(usize, usize)>, rate: usize) -> Vec<(usize, usize)> {
  let mut row_clear_indexes = Vec::new();
  let mut col_clear_indexes = Vec::new();
  for i in 0..grid.len() {
    if is_row_clear(grid, i) { row_clear_indexes.push(i); }
    if is_col_clear(grid, i) { col_clear_indexes.push(i); }
  }

  let mut new_galaxies = Vec::new();
  for galaxy in galaxies.iter() {
    let mut cols_less = 0;
    let mut rows_less = 0;
    let mut index = 0;
      while index < col_clear_indexes.len() && col_clear_indexes[index] < galaxy.1 {
      cols_less += 1;
      index += 1;
    }

    index = 0;
    while index < row_clear_indexes.len() && row_clear_indexes[index] < galaxy.0 {
      rows_less += 1;
      index += 1;
    }

    new_galaxies.push((galaxy.0 + (rows_less * rate), galaxy.1 + (cols_less * rate)));
  }

  return new_galaxies;
}

fn manhanttan_dist(g1: (usize, usize), g2: (usize, usize)) -> usize {
  return i32::abs(g1.0 as i32 - g2.0 as i32) as usize + i32::abs(g1.1 as i32 - g2.1 as i32) as usize;
}

fn sum_expanded_galaxies_dists(grid: &Vec<Vec<char>>, rate: usize) -> usize {
  let mut galaxies = get_galaxy_poss(grid);
  galaxies = expand_galaxies(grid, &galaxies, rate - 1);

  let mut total_dist = 0;
  for i in 0..galaxies.len() - 1 {
    for j in i + 1.. galaxies.len() {
      total_dist += manhanttan_dist(galaxies[i], galaxies[j]);
    }
  }

  return total_dist;
}

fn main() {
  let contents = fs::read_to_string("./data/q11.txt")
    .expect("Should have been able to read file");
  let grid = contents.split('\n').map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

  println!("Part 1: {}", sum_expanded_galaxies_dists(&grid, 2));
  println!("Part 2: {}", sum_expanded_galaxies_dists(&grid, 1000000));
}