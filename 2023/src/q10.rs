use std::collections::HashSet;
use std::fs;
use phf::phf_map;

#[derive(Clone, PartialEq)]
enum Direction {
  North = 0,
  East = 1,
  South = 2,
  West = 3
}

static CHAR_TO_PIPE: phf::Map<char, (Direction, Direction)> = phf_map! {
  '|' => (Direction::North, Direction::South),
  '-' => (Direction::East, Direction::West),
  'L' => (Direction::North, Direction::East),
  'J' => (Direction::North, Direction::West),
  '7' => (Direction::South, Direction::West),
  'F' => (Direction::South, Direction::East)
};

fn translate_dir(d: &Direction) -> (isize, isize) {
  match d {
    Direction::North => return (-1, 0),
    Direction::East => return (0, 1),
    Direction::South => return (1, 0),
    Direction::West => return (0, -1)
  }
}

fn get_opposite_dir(d: &Direction) -> Direction {
  match d {
    Direction::North => return Direction::South,
    Direction::East => return Direction::West,
    Direction::South => return Direction::North,
    Direction::West => return Direction::East
  }
}

fn get_starting_pos(grid: &Vec<Vec<char>>) -> (usize, usize) {
  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      if grid[i][j] == 'S' { return (i, j); }
    }
  }
  return (grid.len(), grid[0].len());
}

fn replace_starting_pipe(grid: &mut Vec<Vec<char>>, start: (usize, usize)) {
  let (mut curr_i, mut curr_j) = start;
  let h = grid.len();
  let w = grid[0].len();
  let mut open_dirs = Vec::new();

  if curr_i > 0 {
    curr_i -= 1;
    match grid[curr_i][curr_j] {
      '|' | '7' | 'F' => open_dirs.push(Direction::North),
      _ => ()
    }
    curr_i += 1;
  }
  if curr_j < w - 1 {
    curr_j += 1;
    match grid[curr_i][curr_j] {
      '-' | 'J' | '7' => open_dirs.push(Direction::East),
      _ => ()
    }
    curr_j -= 1;
  }
  if curr_i < h - 1 {
    curr_i += 1;
    match grid[curr_i][curr_j] {
      '|' | 'L' | 'J' => open_dirs.push(Direction::South),
      _ => ()
    }
    curr_i -= 1;
  }
  if curr_j > 0 {
    curr_j -= 1;
    match grid[curr_i][curr_j] {
      '-' | 'L' | 'F' => open_dirs.push(Direction::West),
      _ => ()
    }
  }

  assert!(open_dirs.len() == 2);
  for (k, (d1, d2)) in CHAR_TO_PIPE.entries() {
    if (open_dirs[0] == *d1 && open_dirs[1] == *d2) || 
      (open_dirs[0] == *d2 && open_dirs[1] == *d1) 
    {
      grid[start.0][start.1] = *k;
      break;
    }
  }
}

fn get_output_dir(grid: &Vec<Vec<char>>, pos: (usize, usize), dir: &Direction) -> Direction {
  let (d1, d2) = CHAR_TO_PIPE[&grid[pos.0][pos.1]].clone();
  let in_dir = get_opposite_dir(dir);
  if in_dir == d1 {
    return d2;
  } else {
    return d1;
  }
}

fn get_path(grid: &Vec<Vec<char>>, start: &(usize, usize)) -> HashSet<(usize, usize)> {
  let mut curr_i = start.0;
  let mut curr_j = start.1;
  let mut curr_dir = CHAR_TO_PIPE[&grid[curr_i][curr_j]].0.clone();
  let mut visited = HashSet::new();

  while !visited.contains(&(curr_i, curr_j)) {
    visited.insert((curr_i, curr_j));

    let (di, dj) = translate_dir(&curr_dir);
    curr_i = (curr_i as isize + di) as usize;
    curr_j = (curr_j as isize + dj) as usize;
    curr_dir = get_output_dir(grid, (curr_i, curr_j), &curr_dir);
  }

  return visited;
}

fn get_max_steps_from_start(grid: &Vec<Vec<char>>, start: &(usize, usize)) -> u32 {
  let path_length = get_path(grid, start).len();
  return ((path_length / 2) + (path_length & 1)) as u32;
}

fn count_enclosed_tiles(grid: &Vec<Vec<char>>, start: &(usize, usize)) -> u32 {
  let path = get_path(grid, start);
  let mut inside_tiles = 0;
  let mut inside = false;
  let h = grid.len();
  let w = grid[0].len();

  for i in 0..h {
    for j in 0..w {
      if path.contains(&(i, j)) {
        match grid[i][j] {
          '|' | 'J' | 'L' => inside = !inside,
          _ => ()
        }
      } else {
        inside_tiles += inside as u32
      }
    }
  }
  return inside_tiles;
}

fn main() {
  let contents = fs::read_to_string("./data/q10.txt")
    .expect("Should have been able to read file");
  let mut grid = contents.split('\n').map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

  let start = get_starting_pos(&grid);
  assert!(start != (grid.len(), grid[0].len()));

  replace_starting_pipe(&mut grid, start);

  println!("Part 1: {}", get_max_steps_from_start(&grid, &start));
  println!("Part 2: {}", count_enclosed_tiles(&grid, &start));
}