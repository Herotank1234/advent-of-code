use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
struct HotSpring {
  springs: Vec<char>,
  record: Vec<u32>
}

fn get_hash(spring: &Vec<char>, record: &Vec<u32>) -> String {
  let mut hash = spring.iter().collect::<String>();
  for r in record {
    hash += "-";
    hash += &r.to_string();
  }
  return hash;
}

fn count_possible_arrangements(spring: Vec<char>, record: Vec<u32>, memo: &mut HashMap<String, u64>) -> u64 {
  if spring.is_empty() { return record.is_empty() as u64; }
  if record.is_empty() { return !spring.contains(&'#') as u64 ; }

  let hash = get_hash(&spring, &record);
  if memo.contains_key(&hash) { return *memo.get(&hash).unwrap(); }

  let mut arrangements = 0;
  if spring[0] == '.' || spring[0] == '?' {
    arrangements += count_possible_arrangements(spring[1..].to_vec(), record.clone(), memo);
  }

  if spring[0] == '#' || spring[0] == '?' {
    if spring.len() >= record[0] as usize && !spring[..record[0] as usize].contains(&'.') {
      if spring.len() == record[0] as usize {
        arrangements += count_possible_arrangements(spring[record[0] as usize..].to_vec(), 
          record[1..].to_vec(), memo)
      } else if spring[record[0] as usize] != '#' {
        arrangements += count_possible_arrangements(spring[record[0] as usize + 1..].to_vec(), 
          record[1..].to_vec(), memo)
      }
    }
  }

  memo.insert(hash, arrangements);

  return arrangements;
}

fn count_all_possible_arrangements(hot_springs: &Vec<HotSpring>) -> u64 {
  let mut memo: HashMap<String, u64> = HashMap::new();
  return hot_springs.iter().map(|hot_spring|
    count_possible_arrangements(hot_spring.springs.clone(), hot_spring.record.clone(), &mut memo)
  ).sum();
}

fn extend(hot_springs: &mut Vec<HotSpring>) {
  for hs in hot_springs.iter_mut() {
    let mut new_springs = Vec::new();
    let mut new_records = Vec::new();
    for i in 0..5 {
      new_springs.extend(&hs.springs);
      new_records.extend(&hs.record);
      if i != 4 { new_springs.push('?'); }
    }
    hs.springs = new_springs;
    hs.record = new_records;
  }
}

fn count_all_possible_arrangements_extended(hot_springs: &Vec<HotSpring>) -> u64 {
  let mut extended_hot_springs = hot_springs.clone();
  extend(&mut extended_hot_springs);
  let mut memo: HashMap<String, u64> = HashMap::new();
  return extended_hot_springs.iter().map(|hot_spring|
    count_possible_arrangements(hot_spring.springs.clone(), hot_spring.record.clone(), &mut memo)
  ).sum();
}

fn main() {
  let contents = fs::read_to_string("./data/q12.txt")
    .expect("Should have been able to read file");
  let hot_springs = contents.split('\n').map(|line| {
    let parts = line.split(' ').collect::<Vec<&str>>();
    return HotSpring{
      springs: parts[0].chars().collect::<Vec<char>>(), 
      record: parts[1].split(',').map(|val| val.parse::<u32>().unwrap()).collect::<Vec<u32>>()
    };
  }).collect::<Vec<HotSpring>>();

  println!("Part 1: {}", count_all_possible_arrangements(&hot_springs));
  println!("Part 2: {}", count_all_possible_arrangements_extended(&hot_springs));
}