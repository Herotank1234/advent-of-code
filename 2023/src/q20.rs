use std::fs;
use std::collections::{HashMap, VecDeque};
use regex::Regex;
 
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pulse {
  Low,
  High
}

struct Message {
  src: String,
  dst: String,
  pulse: Pulse,
}

trait Module {
  fn process_pulse(&mut self, msg: Message) -> Vec<Message>;
}

struct Broadcast {
  name: String,
  dsts: Vec<String>
}

impl Module for Broadcast {
  fn process_pulse(&mut self, msg: Message) -> Vec<Message> {
    return self.dsts.iter().map(|dst| Message{src: self.name.clone(), dst: dst.clone(), pulse: msg.pulse})
      .collect::<Vec<Message>>();
  }
}

struct FlipFlop {
  name: String,
  is_on: bool,
  dsts: Vec<String>
}

impl Module for FlipFlop {
  fn process_pulse(&mut self, msg: Message) -> Vec<Message> {
    match msg.pulse {
      Pulse::Low => {
        let output_pulse = if self.is_on { Pulse::Low } else { Pulse::High };
        self.is_on = !self.is_on;
        return self.dsts.iter().map(|dst| Message{src: self.name.clone(), dst: dst.clone(), pulse: output_pulse})
          .collect::<Vec<Message>>();
      }
      Pulse::High => return vec![]
    }
  }
}

struct Conjunction {
  name: String,
  prev_pulses: HashMap<String, Pulse>,
  dsts: Vec<String>
}

impl Module for Conjunction {
  fn process_pulse(&mut self, msg: Message) -> Vec<Message> {
    let prev_pulse = self.prev_pulses.entry(msg.src).or_insert(Pulse::Low);
    *prev_pulse = msg.pulse;
    let mut is_all_high = true;
    for pulse in self.prev_pulses.values() {
      if *pulse == Pulse::Low {
        is_all_high = false;
        break;
      }
    }
    let output_pulse = if is_all_high { Pulse::Low } else { Pulse::High };
    return self.dsts.iter().map(|dst| Message{src: self.name.clone(), dst: dst.clone(), pulse: output_pulse})
      .collect::<Vec<Message>>();
  }
}

fn press_button(modules: &mut HashMap<String, Box<dyn Module>>, high: &mut u64, low: &mut u64) {
  let mut to_be_processed = VecDeque::new();
  to_be_processed.push_back(Message{src: "".to_string(), dst: "broadcaster".to_string(), pulse: Pulse::Low});

  while !to_be_processed.is_empty() {
    let curr_msg = to_be_processed.pop_front().unwrap();
    match curr_msg.pulse {
      Pulse::Low => *low += 1,
      Pulse::High => *high += 1,
    }

    if modules.contains_key(&curr_msg.dst) {
      let next_msgs = modules.get_mut(&curr_msg.dst).unwrap().process_pulse(curr_msg);
      for msg in next_msgs { to_be_processed.push_back(msg); }
    }
  }
}

fn press_and_count_pulses(modules: &mut HashMap<String, Box<dyn Module>>, iterations: u64) -> u64 {
  let mut high = 0;
  let mut low = 0;
  for _ in 0..iterations {
    press_button(modules, &mut high, &mut low);
  }
  return high * low;
}

fn is_all_non_zero(m: &HashMap<String, u64>) -> bool {
  for val in m.values() {
    if *val == 0 { return false; }
  }
  return true;
}

fn press_and_update_map(modules: &mut HashMap<String, Box<dyn Module>>, pulse_map: &mut HashMap<String, u64>,
  presses: u64) -> bool 
{
  let mut to_be_processed = VecDeque::new();
  to_be_processed.push_back(Message{src: "".to_string(), dst: "broadcaster".to_string(), pulse: Pulse::Low});

  while !to_be_processed.is_empty() {
    let curr_msg = to_be_processed.pop_front().unwrap();
    if pulse_map.contains_key(&curr_msg.dst) && curr_msg.pulse == Pulse::Low {
      let curr_presses = pulse_map.get_mut(&curr_msg.dst).unwrap();
      if *curr_presses == 0 { *curr_presses = presses; }
    }

    if modules.contains_key(&curr_msg.dst) {
      let next_msgs = modules.get_mut(&curr_msg.dst).unwrap().process_pulse(curr_msg);
      for msg in next_msgs { to_be_processed.push_back(msg); }
    }
  }
  return false;
}

fn count_presses_till_low_rx(modules: &mut HashMap<String, Box<dyn Module>>, srcs: &Vec<String>) -> u64 {
  let mut pulse_map = srcs.iter().map(|src| (src.clone(), 0)).collect::<HashMap<String, u64>>();
  let mut presses = 1;
  while !is_all_non_zero(&pulse_map) { 
    press_and_update_map(modules, &mut pulse_map, presses);
    presses += 1;
  }
  return pulse_map.values().fold(1, |acc, s| acc * s);
}
fn main() {
  let contents = fs::read_to_string("./data/q20.txt")
    .expect("Should have been able to read file");
  let lines = contents.split('\n').collect::<Vec<&str>>();

  let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
  let mut modules2: HashMap<String, Box<dyn Module>> = HashMap::new();
  let mut modules_dsts: Vec<(String, String, Vec<String>)> = Vec::new();
  let mut conj_modules: HashMap<String, Vec<String>> = HashMap::new();
  let re = Regex::new(r"(&|%)?([a-z]+) -> (.*)").unwrap();

  for line in lines.iter() {
    let caps = re.captures(line).unwrap();
    let module_name = caps[2].to_string();
    let dsts = caps[3].to_string().split(',').map(|dst| dst.trim().to_string()).collect::<Vec<String>>();
    match caps.get(1) {
      None => modules_dsts.push(("".to_string(), module_name, dsts)),
      Some(m) => {
        let char_ref = m.as_str();
        if char_ref == "&" {
          conj_modules.insert(module_name.clone(), Vec::new());
        }
        modules_dsts.push((m.as_str().to_owned(), module_name, dsts));
      }
    }
  }

  for (_, src, dsts) in modules_dsts.iter() {
    for (conj_name, conj_srcs) in conj_modules.iter_mut() {
      if dsts.contains(&conj_name) { conj_srcs.push(src.clone()); }
    }
  }

  for (char_ref, module_name, dsts) in modules_dsts.iter() {
    match char_ref.as_str() {
      "" => {
        modules.insert(module_name.clone(), Box::new(Broadcast{
          name: module_name.clone(), dsts: dsts.clone()
        }));
        modules2.insert(module_name.clone(), Box::new(Broadcast{
          name: module_name.clone(), dsts: dsts.clone()
        }));
      },
      "%" => {
        modules.insert(module_name.clone(), Box::new(FlipFlop{
          name: module_name.clone(), is_on: false, dsts: dsts.clone()
        }));
        modules2.insert(module_name.clone(), Box::new(FlipFlop{
          name: module_name.clone(), is_on: false, dsts: dsts.clone()
        }));
      },
      "&" => {
        let prev_pulses = conj_modules.get(module_name).unwrap().iter().map(|src|
          (src.clone(), Pulse::Low)
        ).collect::<HashMap<String, Pulse>>();
        modules.insert(module_name.clone(), Box::new(Conjunction{
          name: module_name.clone(), prev_pulses: prev_pulses.clone(), dsts: dsts.clone()
        }));
        modules2.insert(module_name.clone(), Box::new(Conjunction{
          name: module_name.clone(), prev_pulses: prev_pulses.clone(), dsts: dsts.clone()
        }));
      },
      _ => ()
    }
  }

  let mut writes_to_rx = "".to_string();
  for (_, module_name, dsts) in modules_dsts.iter() {
    if dsts.contains(&"rx".to_string()) {
      writes_to_rx = module_name.clone();
      break;
    }
  }

  let mut writes_to_rx_srcs = Vec::new();
  for (_, module_name, dsts) in modules_dsts.iter() {
    if dsts.contains(&writes_to_rx.to_string()) {
      writes_to_rx_srcs.push(module_name.clone());
    }
  }

  println!("Part 1: {}", press_and_count_pulses(&mut modules, 1000));
  println!("Part 2: {}", count_presses_till_low_rx(&mut modules2, &writes_to_rx_srcs));
}