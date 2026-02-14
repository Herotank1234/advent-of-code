use std::fs;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
enum Compare {
  LessThan,
  GreaterThan
}

#[derive(Debug)]
enum Rule {
  Comp(String, Compare, u64, String),
  Label(String)
}

fn parse_rule(rule_str: &str) -> Rule {
  let rule_vec = rule_str.chars().collect::<Vec<char>>();
  let has_lt = rule_vec.contains(&'<');
  let has_gt = rule_vec.contains(&'>');
  if has_lt || has_gt {
    let mut comp_index = 0;
    let mut semic_index = 0;
    for i in 0..rule_vec.len() {
      if rule_vec[i] == '<' || rule_vec[i] == '>' { comp_index = i; }
      if rule_vec[i] == ':' { semic_index = i; }
    }
    let part_name = rule_str[..comp_index].to_string();
    let comp = if has_lt { Compare::LessThan } else { Compare::GreaterThan };
    let comp_value = rule_str[comp_index + 1..semic_index].parse::<u64>().unwrap();
    let output_name = rule_str[semic_index + 1..].to_string();
    return Rule::Comp(part_name, comp, comp_value, output_name);
  }
  return Rule::Label(rule_str.to_string());
}

fn is_accepted(rules: &HashMap<String, Vec<Rule>>, part: &HashMap<String, u64>) -> bool {
  let mut label = "in";
  while label != "A" && label != "R" {
    let curr_rules = rules.get(label).unwrap();
    for rule in curr_rules.iter() {
      match rule {
        Rule::Comp(part_name, comp, comp_val, out_label) => {
          let part_val = part.get(part_name).unwrap();
          let comp_res;
          match comp {
            Compare::LessThan => comp_res = part_val < comp_val,
            Compare::GreaterThan => comp_res = part_val > comp_val,
          }
          if comp_res {
            label = out_label;
            break;
          }
        },
        Rule::Label(out_label) => {
          label = out_label;
          break;
        }
      }
    }
  }
  return label == "A";
}

fn sum_accepted_parts(rules: &HashMap<String, Vec<Rule>>, parts: &Vec<HashMap<String, u64>>) -> u64 {
  let mut total = 0;
  for part in parts.iter() {
    if is_accepted(rules, part) {
      total += part.values().sum::<u64>();
    }
  }
  return total;
}

fn get_combo_ranges(rules: &HashMap<String, Vec<Rule>>, mut state: HashMap<String, (u64, u64)>, label: String)
  -> Vec<HashMap<String, (u64, u64)>> 
{
  if label == "A" { return vec![state]; }
  if label == "R" { return vec![]; }

  let curr_rules = rules.get(&label).unwrap();
  let mut ranges = Vec::new();

  for rule in curr_rules.iter() {
    match rule {
      Rule::Comp(part_name, comp, comp_val, out_label) => {
        let mut next_state = state.clone();
        match comp {
          Compare::LessThan => {
            next_state.get_mut(part_name).unwrap().1 = *comp_val;
            let curr_ranges = get_combo_ranges(rules, next_state.clone(), out_label.clone());
            ranges.extend(curr_ranges);
            state.get_mut(part_name).unwrap().0 = *comp_val;
          },
          Compare::GreaterThan => {
            next_state.get_mut(part_name).unwrap().0 = *comp_val + 1;
            let curr_ranges = get_combo_ranges(rules, next_state.clone(), out_label.clone());
            ranges.extend(curr_ranges);
            state.get_mut(part_name).unwrap().1 = *comp_val + 1;
          }
        }
      },
      Rule::Label(out_label) => {
        let curr_ranges = get_combo_ranges(rules, state.clone(), out_label.clone());
        ranges.extend(curr_ranges);
      }
    }
  }

  return ranges;
}

fn count_accepted_combinations(rules: &HashMap<String, Vec<Rule>>) -> u64 {
  let initial_state = HashMap::from([
    ("x".to_string(), (1, 4001)),
    ("m".to_string(), (1, 4001)),
    ("a".to_string(), (1, 4001)),
    ("s".to_string(), (1, 4001)),
  ]);
  let combo_ranges = get_combo_ranges(rules, initial_state, "in".to_string());
  return combo_ranges.iter().map(|ranges_map| {
    let ranges = ranges_map.values().collect::<Vec<&(u64, u64)>>();
    return ranges.iter().fold(1, |acc, (l, u)| acc * (u - l));
  }).sum();
}

fn main() {
  let contents = fs::read_to_string("./data/q19.txt")
    .expect("Should have been able to read file");
  let lines = contents.split('\n').collect::<Vec<&str>>();

  let mut section_break_index = 0;
  while lines[section_break_index] != "" {
    section_break_index += 1;
  }

  let first_section = lines[..section_break_index].to_vec();
  let second_section = lines[section_break_index + 1..].to_vec();

  let mut rules = HashMap::new();
  let rule_re = Regex::new(r"([a-z]+)\{(.*)\}").unwrap();
  for line in first_section.iter() {
    let caps = rule_re.captures(line).unwrap();
    let name = caps[1].to_string();
    let rule_parts = caps[2].split(',').map(|rule_str| parse_rule(rule_str)).collect::<Vec<Rule>>();
    rules.insert(name, rule_parts);
  }
  
  let mut parts = Vec::new();
  let part_re = Regex::new(r"\{x=([\d]+),m=([\d]+),a=([\d]+),s=([\d]+)\}").unwrap();
  for line in second_section.iter() {
    let caps = part_re.captures(line).unwrap();
    let parts_map = HashMap::from([
      ("x".to_string(), caps[1].parse::<u64>().unwrap()),
      ("m".to_string(), caps[2].parse::<u64>().unwrap()),
      ("a".to_string(), caps[3].parse::<u64>().unwrap()),
      ("s".to_string(), caps[4].parse::<u64>().unwrap()),
    ]);
    parts.push(parts_map);
  }

  println!("Part 1: {}", sum_accepted_parts(&rules, &parts));
  println!("Part 2: {}", count_accepted_combinations(&rules));
}