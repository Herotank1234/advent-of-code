use std::{cmp::max, collections::{HashMap, HashSet, VecDeque}, fs};
use regex::Regex;

#[derive(Debug)]
struct Valve {
  flow: u32,
  neighbours: Vec<String>
}

fn get_shortest_path_lens(start: &String, valves: &HashMap<String, Valve>) -> Vec<(String, u32)> {
  let mut visited = HashSet::new();
  let mut to_be_visited = VecDeque::new();
  to_be_visited.push_back((start, 0));

  let mut shortest_paths = Vec::new();

  while !to_be_visited.is_empty() {
    let (valve_str, time) = to_be_visited.pop_front().unwrap();
    if visited.contains(valve_str) { continue; }

    visited.insert(valve_str.clone());
    if valve_str != start && valves.get(valve_str).unwrap().flow != 0 {
      shortest_paths.push((valve_str.clone(), time));
    }

    let neighbours = &valves.get(valve_str).unwrap().neighbours;
    for neighbour in neighbours.iter() {
      to_be_visited.push_back((neighbour, time + 1));
    }
  }

  return shortest_paths;
}

fn get_max_flow(valves: &HashMap<String, Valve>, adj_list: &HashMap<String, Vec<(String, u32)>>,
  curr_valve: String, time: u32, total_flow: u32, mut opened: HashSet<String>) -> u32 
{
  if time == 0 { return total_flow; }

  let mut next_time = time;
  let mut next_flow = total_flow;
  let valve = valves.get(&curr_valve).unwrap();

  if valve.flow != 0 {
    opened.insert(curr_valve.clone());
    next_time -= 1;
    next_flow += next_time * valve.flow;
  }

  if next_time == 0 { return total_flow; }
  let mut max_flow = next_flow;

  let neighbours = adj_list.get(&curr_valve).unwrap();
  for (name, time_to_go) in neighbours.iter() {
    if *time_to_go >= next_time { continue; }
    if opened.contains(name) { continue; }
    let next_opened = opened.clone();
    max_flow = max(max_flow, get_max_flow(valves, adj_list, name.clone(), next_time - time_to_go,
      next_flow, next_opened));
  }

  return max_flow;
}

fn get_opened_valves(openable_valves: &Vec<String>, mut bitmap: u32, use_ones: bool) -> HashSet<String> {
  let mut opened_valves = HashSet::new();
  for i in 0..openable_valves.len() {
    let bit = bitmap & 1;
    bitmap = bitmap >> 1;
    if bit == use_ones as u32 { opened_valves.insert(openable_valves[i].clone()); }
  }
  return opened_valves;
}

fn get_max_flow_pair(valves: &HashMap<String, Valve>, adj_list: &HashMap<String, Vec<(String, u32)>>) -> u32 {
  let mut openable_valves = adj_list.keys().map(|k| k.clone()).collect::<Vec<String>>();
  openable_valves.remove(openable_valves.iter().position(|v| *v == "AA".to_string()).unwrap());
  let number_of_valves = openable_valves.len() as u32;

  let mut max_flow = 0;
  for i in 0..2_u32.pow(number_of_valves) / 2 {
    let opened_valves = get_opened_valves(&openable_valves, i, true);
    let other_opened_valves = get_opened_valves(&openable_valves, i, false);
    max_flow = max(max_flow, get_max_flow(valves, adj_list, String::from("AA"), 26, 0, opened_valves) +
      get_max_flow(valves, adj_list, String::from("AA"), 26, 0, other_opened_valves));
  }

  return max_flow;
}

fn main() {
  let contents = fs::read_to_string("./data/q16.txt")
    .expect("Should have been able to read file");
  let re = Regex::new(r"Valve ([A-Z]+) has flow rate=([\d]+); tunnel[s]? lead[s]? to valve[s]? (.*)")
    .unwrap();

  let valves = contents.split('\n').map(|line| {
    let caps = re.captures(line).unwrap();
    let name = caps[1].to_string();
    let flow = caps[2].parse::<u32>().unwrap();
    let neighbours = caps[3].split(',').map(|neighbour| neighbour.trim().to_string())
      .collect::<Vec<String>>();
    (name, Valve{flow, neighbours})
  }).collect::<HashMap<String, Valve>>();

  let mut adj_list = valves.iter().filter_map(|(name, v)| 
    if v.flow != 0 { 
      Some((name.clone(), get_shortest_path_lens(&name, &valves))) 
    } else { 
      None 
    }
  ).collect::<HashMap<String, Vec<(String, u32)>>>();
  adj_list.insert(String::from("AA"), get_shortest_path_lens(&String::from("AA"), &valves));

  println!("Part 1: {}", get_max_flow(&valves, &adj_list, String::from("AA"), 30, 0, HashSet::new()));
  println!("Part 2: {}", get_max_flow_pair(&valves, &adj_list));
}