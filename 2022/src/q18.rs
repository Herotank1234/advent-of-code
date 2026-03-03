use std::{cmp::max, collections::{HashSet, VecDeque}, fs, ops::Add};

const LOWER_BOUND: i32 = -5;
const UPPER_BOUND: i32 = 5;

const DIRS: &[Point; 6] = &[
  Point{x: 1, y: 0, z: 0},
  Point{x: 0, y: 1, z: 0},
  Point{x: 0, y: 0, z: 1},
  Point{x: -1, y: 0, z: 0},
  Point{x: 0, y: -1, z: 0},
  Point{x: 0, y: 0, z: -1}
];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
  x: i32,
  y: i32,
  z: i32
}

impl Add for Point {
  type Output = Point;
  fn add(self, rhs: Self) -> Self::Output {
    return Point{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z};
  }
}

fn get_surface_area(points: &HashSet<Point>) -> u32 {
  let mut exposed_faces = 0;
  for point in points.iter() {
    for dir in DIRS.iter() {
      let neighbour = *point + *dir;
      if !points.contains(&neighbour) { exposed_faces += 1; }
    }
  }
  return exposed_faces;
}

fn is_in_bounds(p: &Point, max_p: &Point) -> bool {
  return p.x >= LOWER_BOUND && p.x <= max_p.x && 
    p.y >= LOWER_BOUND && p.y <= max_p.y && 
    p.z >= LOWER_BOUND && p.z <= max_p.z;
}

fn get_outer_surface_area(points: &HashSet<Point>) -> u32 {
  let mut max_x = 0;
  let mut max_y = 0;
  let mut max_z = 0;
  for point in points.iter() {
    max_x = max(max_x, point.x);
    max_y = max(max_y, point.y);
    max_z = max(max_z, point.z);
  }
  let max_p = Point{x: max_x + UPPER_BOUND, y: max_y + UPPER_BOUND, 
    z: max_z + UPPER_BOUND};

  let mut visited = HashSet::new();
  let mut to_be_visited = VecDeque::new();
  to_be_visited.push_back(Point{x: LOWER_BOUND, y: LOWER_BOUND, 
    z: LOWER_BOUND});
  
  let mut outer_surface_area = 0;

  while !to_be_visited.is_empty() {
    let p = to_be_visited.pop_front().unwrap();
    if visited.contains(&p) { continue; }
    visited.insert(p);

    for dir in DIRS.iter() {
      let neighbour = p + *dir;
      if !is_in_bounds(&neighbour, &max_p) { continue; }
      if points.contains(&neighbour) {
        outer_surface_area += 1;
      } else {
        to_be_visited.push_back(neighbour);
      }
    }
  }

  return outer_surface_area;
}

fn main() {
  let contents = fs::read_to_string("./data/q18.txt")
    .expect("Should have been able to read file");
  let points = contents.split('\n').map(|line| {
    let parts = line.split(',').map(|part| part.parse::<i32>().unwrap())
      .collect::<Vec<i32>>();
    let [x, y, z] = parts[..] else { panic!("Line not recognised: {}", line); };
    Point{x, y, z}
  }).collect::<HashSet<Point>>();

  println!("Part 1: {}", get_surface_area(&points));
  println!("Part 2: {}", get_outer_surface_area(&points));
}