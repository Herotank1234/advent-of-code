use std::{fs, collections::HashMap, cmp::max};
use regex::Regex;

#[derive(Debug)]
struct Blueprint {
  id: u32,
  ore_cost: u32,
  clay_cost: u32,
  ob_ore_cost: u32,
  ob_clay_cost: u32,
  geo_ore_cost: u32,
  geo_ob_cost: u32
}

impl Blueprint {
  fn new(id: u32, ore_cost: u32, clay_cost: u32, ob_ore_cost: u32, ob_clay_cost: u32,
    geo_ore_cost: u32, geo_ob_cost: u32) -> Self
  {
    return Blueprint {id, ore_cost, clay_cost, ob_ore_cost, ob_clay_cost,
      geo_ore_cost, geo_ob_cost};
  }
}

fn get_max_geodes(blueprint: &Blueprint, memo: &mut HashMap<String, u32>, 
  max_robots: &[u32; 3], time: i32, robots: [u32; 4], resources: [u32; 4]) -> u32
{
  if time == 0 { return resources[3]; }

  let hash = format!("{},{},{},{},{},{},{},{},{}", time, robots[0], robots[1],
    robots[2], robots[3], resources[0], resources[1], resources[2], resources[3]);
  let in_memo = memo.get(&hash);
  match in_memo {
    Some(max_geodes) => return *max_geodes,
    _ => ()
  }

  let mut max_geodes = resources[3] + (robots[3] * time as u32);

  if robots[0] < max_robots[0] {
    let mut next_time = time;
    let mut next_resources = resources;
    while next_resources[0] < blueprint.ore_cost {
      next_time -= 1;
      for i in 0..next_resources.len() {
        next_resources[i] += robots[i];
      }
    }

    if next_time > 0 {
      let mut next_robots = robots;
      next_robots[0] += 1;
      next_resources[0] -= blueprint.ore_cost;
      for i in 0..next_resources.len() {
        next_resources[i] += robots[i];
      }
      next_time -= 1;
      max_geodes = max(max_geodes, get_max_geodes(blueprint, memo, max_robots,
        next_time, next_robots, next_resources))
    }
  }

  if robots[1] < max_robots[1] {
    let mut next_time = time;
    let mut next_resources = resources;
    while next_resources[0] < blueprint.clay_cost {
      next_time -= 1;
      for i in 0..next_resources.len() {
        next_resources[i] += robots[i];
      }
    }

    if next_time > 0 {
      let mut next_robots = robots;
      next_robots[1] += 1;
      next_resources[0] -= blueprint.clay_cost;
      for i in 0..next_resources.len() {
        next_resources[i] += robots[i];
      }
      next_time -= 1;
      max_geodes = max(max_geodes, get_max_geodes(blueprint, memo, max_robots,
        next_time, next_robots, next_resources))
    }
  }

  if robots[2] < max_robots[2] && robots[1] >= 1 {
    let mut next_time = time;
    let mut next_resources = resources;
    while next_resources[0] < blueprint.ob_ore_cost || 
      next_resources[1] < blueprint.ob_clay_cost 
    {
      next_time -= 1;
      for i in 0..next_resources.len() {
        next_resources[i] += robots[i];
      }
    }

    if next_time > 0 {
      let mut next_robots = robots;
      next_robots[2] += 1;
      next_resources[0] -= blueprint.ob_ore_cost;
      next_resources[1] -= blueprint.ob_clay_cost;
      next_time -= 1;
      for i in 0..next_resources.len() {
        next_resources[i] += robots[i];
      }
      max_geodes = max(max_geodes, get_max_geodes(blueprint, memo, max_robots,
        next_time, next_robots, next_resources))
    }
  }

  if robots[2] >= 1 {
    let mut next_time = time;
    let mut next_resources = resources;
    while next_resources[0] < blueprint.geo_ore_cost || 
      next_resources[2] < blueprint.geo_ob_cost
    {
      next_time -= 1;
      for i in 0..next_resources.len() {
        next_resources[i] += robots[i];
      }
    }

    if next_time > 0 {
      let mut next_robots = robots;
      next_robots[3] += 1;
      next_resources[0] -= blueprint.geo_ore_cost;
      next_resources[2] -= blueprint.geo_ob_cost;
      next_time -= 1;
      for i in 0..next_resources.len() {
        next_resources[i] += robots[i];
      }
      max_geodes = max(max_geodes, get_max_geodes(blueprint, memo, max_robots,
        next_time,next_robots, next_resources))
    }
  }
  
  memo.insert(hash, max_geodes);
  return max_geodes;
}

fn sum_quality_level(blueprints: &Vec<Blueprint>) -> u32 {
  return blueprints.iter().map(|blueprint| {
    let max_ore = max(max(max(blueprint.ore_cost, blueprint.clay_cost),
      blueprint.ob_ore_cost), blueprint.geo_ore_cost);
    let max_clay = blueprint.ob_clay_cost;
    let max_ob = blueprint.geo_ob_cost;
    let max_robots: [u32; 3] = [max_ore, max_clay, max_ob];
    blueprint.id * get_max_geodes(blueprint, &mut HashMap::new(), &max_robots, 
      24, [1, 0, 0, 0], [0, 0 ,0 ,0])
  }).sum();
}

fn mul_three_geodes(blueprints: &Vec<Blueprint>) -> u32 {
  return blueprints.iter().take(3).fold(1, |acc, blueprint| {
    let max_ore = max(max(max(blueprint.ore_cost, blueprint.clay_cost),
      blueprint.ob_ore_cost), blueprint.geo_ore_cost);
    let max_clay = blueprint.ob_clay_cost;
    let max_ob = blueprint.geo_ob_cost;
    let max_robots: [u32; 3] = [max_ore, max_clay, max_ob];
    acc * get_max_geodes(blueprint, &mut HashMap::new(), &max_robots, 
      32, [1, 0, 0, 0], [0, 0 ,0 ,0])
  });
}

fn main() {
  let contents = fs::read_to_string("./data/q19.txt")
    .expect("Should have been able to read file");
  let re = Regex::new("Blueprint ([\\d]+): Each ore robot costs ([\\d]+) ore. \
    Each clay robot costs ([\\d+]+) ore. \
    Each obsidian robot costs ([\\d]+) ore and ([\\d]+) clay. \
    Each geode robot costs ([\\d]+) ore and ([\\d]+) obsidian.").unwrap();

  let blueprints = contents.split('\n').map(|line| {
    let caps = re.captures(line).unwrap().iter().skip(1)
      .map(|val| val.unwrap().as_str().parse::<u32>().unwrap())
      .collect::<Vec<u32>>();
    Blueprint::new(caps[0], caps[1], caps[2], caps[3], caps[4], caps[5], caps[6])
  }).collect::<Vec<Blueprint>>();

  println!("Part 1: {}", sum_quality_level(&blueprints));
  println!("Part 2: {}", mul_three_geodes(&blueprints));
}