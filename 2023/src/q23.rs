use std::{collections::{HashMap, HashSet, VecDeque}, fs, cmp::max};

use phf::phf_map;

const DIRS: &[(isize, isize)] = &[
  (-1, 0),
  (0, 1),
  (1, 0),
  (0, -1)
];

const SLOPE_TO_DIR: phf::Map<char, (isize, isize)> = phf_map![
  '^' => (-1, 0),
  '>' => (0, 1),
  'v' => (1, 0),
  '<' => (0, -1)
];

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Point {
  i: usize,
  j: usize
}

fn is_in_bounds(i: isize, j: isize, h: isize, w: isize) -> bool {
  return i >= 0 && i < h && j >= 0 && j < w;
}

fn get_neighbours(grid: &Vec<Vec<char>>, start: &Point, junctions: &Vec<Point>) -> Vec<(Point, u32)> {
  let h = grid.len() as isize;
  let w = grid[0].len() as isize;
  let mut neighbours = Vec::new();
  let mut to_be_checked = VecDeque::new();
  to_be_checked.push_back((start.i as isize, start.j as isize, 0));
  let mut visited = vec![vec![false; w as usize]; h as usize];

  while !to_be_checked.is_empty() {
    let (i, j, steps) = to_be_checked.pop_front().unwrap();
    let curr_i = i as usize;
    let curr_j = j as usize;
    visited[curr_i][curr_j] = true;

    let next_junction = Point{i: curr_i, j: curr_j};
    if next_junction != *start && junctions.contains(&next_junction) {
      neighbours.push((next_junction, steps));
      continue;
    }

    for ref dir@(di, dj) in DIRS {
      let next_i = i + di;
      let next_j = j + dj;
      if is_in_bounds(next_i, next_j, h, w) {
        if visited[next_i as usize][next_j as usize] { continue; }
        match grid[next_i as usize][next_j as usize] {
          '.' => to_be_checked.push_back((next_i, next_j, steps + 1)),

          '^' | '>' | 'v' | '<' => {
            let slope_dir = SLOPE_TO_DIR.get(&grid[next_i as usize][next_j as usize]).unwrap();
            if slope_dir == *dir { 
              visited[next_i as usize][next_j as usize] = true;
              to_be_checked.push_back((next_i + di, next_j + dj, steps + 2)); 
            }
          },

          _ => ()
        }
      }
    }
  }

  return neighbours;
}

fn to_graph(grid: &Vec<Vec<char>>, start: &Point, end: &Point) -> HashMap<Point, Vec<(Point, u32)>> {
  let mut junctions = vec![start.clone(), end.clone()];
  for i in 1..grid.len() - 1 {
    for j in 1..grid[0].len() - 1 {
      if grid[i][j] == '#' { continue; }
      let mut edges = 0;
      for (di, dj) in DIRS.iter() {
        let next_i = i as isize + di;
        let next_j = j as isize + dj;
        if grid[next_i as usize][next_j as usize] != '#' { edges += 1; }
      }
      if edges >= 3 { junctions.push(Point{i, j}); }
    }
  }

  let graph = junctions.iter().map(|junction| (junction.clone(), get_neighbours(grid, junction, &junctions)))
    .collect::<HashMap<Point, Vec<(Point, u32)>>>();
  return graph;
}

fn find_max_path_length(graph: &HashMap<Point, Vec<(Point, u32)>>, visited: &mut HashSet<Point>,
  pos: Point, end: &Point) -> Option<u32> 
{
  if pos == *end { return Some(0); }
  visited.insert(pos.clone());

  let mut max_steps = 0;
  let mut found = false;
  let neighbours = graph.get(&pos).unwrap();

  for (neighbour, steps) in neighbours.iter() {
    let mut next_visited = visited.clone();
    if visited.contains(neighbour) { continue; }
    match find_max_path_length(graph, &mut next_visited, neighbour.clone(), end) {
      Some(next_steps) => {
        found = true;
        max_steps = max(max_steps, next_steps + steps);
      },

      None => ()
    }
  }
  if !found { return None; }
  return Some(max_steps);
}

fn main() {
  let contents = fs::read_to_string("./data/q23.txt")
    .expect("Should have been able to read file");
  let grid = contents.split('\n').map(|row| row.chars().collect()).collect::<Vec<Vec<char>>>();

  let mut start = Point{i: 0, j: 0};
  for j in 0..grid[0].len() {
    if grid[0][j] == '.' { 
      start = Point{i: 0, j};
      break;
    }
  }
  let mut end = Point{i: 0, j: 0};
  for j in 0..grid[0].len() {
    if grid[grid.len() - 1][j] == '.' { 
      end = Point{i: grid.len() - 1, j};
      break;
    }
  }

  let graph = to_graph(&grid, &start, &end);
  let mut grid_no_slope = grid.clone();
  for row in grid_no_slope.iter_mut() {
    for c in row.iter_mut() {
      match *c {
        '#' | '.' => (),
        _ => *c = '.'
      }
    }
  }
  let graph_no_slope = to_graph(&grid_no_slope, &start, &end);

  let mut prev_end = end.clone();
  let mut back_excess = 0;
  let mut found = false;
  for (p, neighbours) in graph_no_slope.iter() {
    for (neighbour, steps) in neighbours.iter() {
      if *neighbour == end { 
        prev_end = p.clone();
        back_excess = *steps;
        found = true;
        break; 
      }
    }
    if found { break; }
  }

  let (succ_start, front_excess) = &graph_no_slope.get(&start).unwrap()[0];

  println!("Part 1: {}", find_max_path_length(&graph, &mut HashSet::new(), start.clone(), &end).unwrap());
  println!("Part 2: {}", find_max_path_length(&graph_no_slope, &mut HashSet::new(), succ_start.clone(), &prev_end).unwrap()
    + front_excess
    + back_excess
  );
}