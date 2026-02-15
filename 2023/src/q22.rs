use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
  x: u32,
  y: u32,
  z: u32
}

#[derive(Clone, Debug)]
struct Block {
  id: u32,
  points: Vec<Point>
}

fn has_point_under(p: &Point, occupied_points: &HashSet<Point>) -> bool {
  let mut point_under = p.clone();
  point_under.z -= 1;
  return occupied_points.contains(&point_under);
}

fn fall(blocks: &Vec<Block>) -> Vec<Block>{
  let mut settled_blocks = Vec::new();
  let mut occupied_points = HashSet::new();
  for i in 0..blocks.len() {
    let mut block = blocks.get(i).unwrap().clone();
    let mut moveable = true;

    while moveable {
      for point in block.points.iter_mut() {
        if point.z == 1 { 
          moveable = false;
          break;
        }
        if has_point_under(point, &occupied_points) {
          moveable = false;
          break;
        }
      }

      if moveable {
        for point in block.points.iter_mut() { point.z -= 1; }
      }
    }
    for point in block.points.iter() {
      occupied_points.insert(point.clone());
    }
    settled_blocks.push(block);
  }
  return settled_blocks;
}

fn count_disintegratable_blocks(blocks: &Vec<Block>) -> u32 {
  let mut supports = blocks.iter().map(|b| (b.id, HashSet::new())).collect::<HashMap<u32, HashSet<u32>>>();

  for i in 0..blocks.len() {
    let block = blocks.get(i).unwrap();
    for point in block.points.iter() {
      if point.z == 1 { break; }
      
      let mut point_under = point.clone();
      point_under.z -= 1;
      for other_block in &blocks[..i] {
        if other_block.points.contains(&point_under) { 
          supports.get_mut(&other_block.id).unwrap().insert(block.id);
        }
      }
    }
  }

  let mut disintegratable = 0;
  for (id, support) in supports.iter() {
    let curr_support = support.iter().collect::<Vec<&u32>>();
    let mut found = vec![false; support.len()];

    for (other_id, other_support) in supports.iter() {
      if id == other_id { continue; }
      for i in 0..support.len() {
        if other_support.contains(&curr_support[i]) { found[i] = true; }
      }
    }

    let is_all_found = found.iter().fold(true, |acc, b| acc && *b);
    if is_all_found { disintegratable += 1; }
  }
  return disintegratable;
}

fn count_chain_reaction_falls(id: u32, supports: &HashMap<u32, HashSet<u32>>,
  supported_by: &HashMap<u32, HashSet<u32>>) -> u32 
{
  let mut fallen = HashSet::new();
  fallen.insert(id);

  let mut to_be_checked = VecDeque::new();
  for s in supports.get(&id).unwrap() {
    to_be_checked.push_back(s);
  }

  let mut chain_reaction_falls = 0;
  while !to_be_checked.is_empty() {
    let curr_id = to_be_checked.pop_front().unwrap();

    let mut is_falling = true;
    for s in supported_by.get(curr_id).unwrap() {
      if !fallen.contains(s) {
        is_falling = false;
        break;
      }
    }

    if is_falling {
      fallen.insert(*curr_id);
      chain_reaction_falls += 1;
      for s in supports.get(curr_id).unwrap() {
        if !to_be_checked.contains(&s) {
          to_be_checked.push_back(s);
        }
      }
    }
  }
  return chain_reaction_falls;
}

fn sum_chain_reaction_falls(blocks: &Vec<Block>) -> u32 {
  let mut supports = blocks.iter().map(|b| (b.id, HashSet::new())).collect::<HashMap<u32, HashSet<u32>>>();

  for i in 0..blocks.len() {
    let block = blocks.get(i).unwrap();
    for point in block.points.iter() {
      if point.z == 1 { break; }
      
      let mut point_under = point.clone();
      point_under.z -= 1;
      for other_block in &blocks[..i] {
        if other_block.points.contains(&point_under) { 
          supports.get_mut(&other_block.id).unwrap().insert(block.id);
        }
      }
    }
  }

  let mut supported_by = blocks.iter().map(|b| (b.id, HashSet::new())).collect::<HashMap<u32, HashSet<u32>>>();
  for (id, supports) in supports.iter() {
    for s in supports {
      supported_by.get_mut(s).unwrap().insert(*id);
    }
  }
  return supports.keys().map(|id| count_chain_reaction_falls(*id, &supports, &supported_by)).sum();
}

fn main() {
  let contents = fs::read_to_string("./data/q22.txt")
    .expect("Should have been able to read file");
  let lines = contents.split('\n').collect::<Vec<&str>>();
  let mut blocks = Vec::new();

  let re = Regex::new(r"([\d]+),([\d]+),([\d]+)~([\d]+),([\d]+),([\d]+)").unwrap();
  let mut id = 0;
  for line in lines {
    let caps = re.captures(line).unwrap();
    let vals = caps.iter().skip(1).map(|cap| cap.unwrap().as_str().parse::<u32>().unwrap())
      .collect::<Vec<u32>>();

    let [x_l, y_l, z_l, x_u, y_u, z_u] = vals[..] else {
      panic!("Bad line {}", line);
    };
    let mut points = Vec::new();
    for z in z_l..=z_u {
      for y in y_l..=y_u {
        for x in x_l..=x_u {
          points.push(Point{x, y, z});
        }
      }
    }
    blocks.push(Block{id, points});
    id += 1;
  }

  blocks.sort_by(|a, b| a.points[0].z.cmp(&b.points[0].z));
  let settled_blocks = fall(&blocks);

  println!("Part 1: {}", count_disintegratable_blocks(&settled_blocks));
  println!("Part 2: {}", sum_chain_reaction_falls(&settled_blocks));
}