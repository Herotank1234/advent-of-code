use std::fs;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::cmp::max;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
  North,
  East,
  South,
  West
}

fn translate_dir(d: &Direction) -> (isize, isize) {
  match d {
    Direction::North => return (-1, 0),
    Direction::East => return (0, 1),
    Direction::South => return (1, 0),
    Direction::West => return (0, -1)
  }
}

fn is_in_bounds(i: isize, j: isize, h: isize, w: isize) -> bool {
  return i >= 0 && i < h && j >= 0 && j < w;
}

fn count_visited_squares(grid: &Vec<Vec<char>>, start_i: isize, start_j: isize, start_dir: Direction) -> u32 {
  let h = grid.len() as isize;
  let w = grid[0].len() as isize;
  let mut lit_up = vec![vec![false; w as usize]; h as usize];
  let mut visited = HashSet::new();
  let mut to_be_visited: VecDeque<(isize, isize, Direction)> = VecDeque::new();
  to_be_visited.push_back((start_i, start_j, start_dir));

  while !to_be_visited.is_empty() {
    let state@(i, j, dir) = to_be_visited.pop_front().unwrap();
    if is_in_bounds(i, j, h, w) { lit_up[i as usize][j as usize] = true; }

    if visited.contains(&state) { continue; }
    visited.insert(state);

    let (di, dj) = translate_dir(&dir);
    let next_i = i + di;
    let next_j = j + dj;
    if !is_in_bounds(next_i, next_j, h, w) { continue; }
    match grid[next_i as usize][next_j as usize] {
      '.' => to_be_visited.push_back((next_i, next_j, dir)),
      '|' => {
        match dir {
          Direction::North | Direction::South => to_be_visited.push_back((next_i, next_j, dir)),
          _ => {
            to_be_visited.push_back((next_i, next_j, Direction::North));
            to_be_visited.push_back((next_i, next_j, Direction::South));
          }
        }
      },
      '-' => {
        match dir {
          Direction::East | Direction::West => to_be_visited.push_back((next_i, next_j, dir)),
          _ => {
            to_be_visited.push_back((next_i, next_j, Direction::East));
            to_be_visited.push_back((next_i, next_j, Direction::West));
          }
        }
      }
      '/' => {
        match dir {
          Direction::North => to_be_visited.push_back((next_i, next_j, Direction::East)), 
          Direction::East => to_be_visited.push_back((next_i, next_j, Direction::North)),
          Direction::South => to_be_visited.push_back((next_i, next_j, Direction::West)),
          Direction::West => to_be_visited.push_back((next_i, next_j, Direction::South)),
        }
      },
      '\\' => {
        match dir {
          Direction::North => to_be_visited.push_back((next_i, next_j, Direction::West)), 
          Direction::East => to_be_visited.push_back((next_i, next_j, Direction::South)),
          Direction::South => to_be_visited.push_back((next_i, next_j, Direction::East)),
          Direction::West => to_be_visited.push_back((next_i, next_j, Direction::North)),
        }
      },
      _ => println!("{} not recognised", grid[next_i as usize][next_j as usize])
    }
  }

  return lit_up.iter().map(|row| row.iter().map(|col| *col as u32).sum::<u32>()).sum();
}

fn count_visited_squares_top_left(grid: &Vec<Vec<char>>) -> u32 {
  return count_visited_squares(grid, 0, -1, Direction::East);
}

fn max_visited_squares(grid: &Vec<Vec<char>>) -> u32 {
  let mut max_visited = 0;
  
  for i in 0..grid.len() {
    max_visited = max(max_visited, count_visited_squares(grid, i as isize, -1, Direction::East));
    max_visited = max(max_visited, count_visited_squares(grid, i as isize, grid[0].len() as isize, Direction::West));
  }

  for j in 0..grid[0].len() {
    max_visited = max(max_visited, count_visited_squares(grid, -1, j as isize, Direction::South));
    max_visited = max(max_visited, count_visited_squares(grid, grid.len() as isize, j as isize, Direction::North));
  }

  return max_visited;
}

fn main() {
  let contents = fs::read_to_string("./data/q16.txt")
    .expect("Should have been able to read file");
  let grid = contents.split('\n').map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

  println!("Part 1: {}", count_visited_squares_top_left(&grid));
  println!("Part 2: {}", max_visited_squares(&grid));
}