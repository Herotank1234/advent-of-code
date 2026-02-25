use std::{
  fs,
  cmp::Ordering
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug)]
enum ListType {
  Value(u32),
  List(Vec<ListType>)
}

impl Ord for ListType {
  fn cmp(&self, other: &Self) -> Ordering {
    match self {
      ListType::Value(val1) => {
        match other {
          ListType::Value(val2) => return val1.cmp(val2),
          ListType::List(_) => return ListType::List(vec![self.clone()]).cmp(other)
        }
      },

      ListType::List(list1) => {
        match other {
          ListType::Value(_) => return self.cmp(&ListType::List(vec![other.clone()])),
          ListType::List(list2) => {
            let mut index = 0;
            while index < list1.len() && index < list2.len() {
              let res = list1[index].cmp(&list2[index]);
              if res != Ordering::Equal {
                return res;
              }
              index += 1;
            }
            if index == list1.len() && index != list2.len() { return Ordering::Less; }
            if index != list1.len() && index == list2.len() { return Ordering::Greater; }
            return Ordering::Equal
          }
        }
      }
    }
  }
}

fn parse_list_str(line: &Vec<char>, index: &mut usize) -> ListType {
  let mut list = Vec::new();
  assert_eq!(line[*index], '[');
  *index += 1;

  while *index < line.len() {
    if line[*index] == '[' {
      let part = parse_list_str(line, index);
      list.push(part);
    } else if line[*index].is_numeric() {
      let mut val_str = String::new();
      while line[*index].is_numeric() { 
        val_str.push(line[*index]);
        *index += 1;
      }
      list.push(ListType::Value(val_str.parse::<u32>().unwrap()));
    } else {
      assert_eq!(line[*index], ']');
      *index += 1;
      break;
    } 

    if line[*index] == ',' { *index += 1; } 
  }
  return ListType::List(list);
}

fn sum_correct_pair_indexes(pairs: &Vec<(ListType, ListType)>) -> u32 {
  let mut total = 0;
  for i in 0..pairs.len() {
    let (left, right) = &pairs[i];
    if left.cmp(&right) == Ordering::Less { total += i as u32 + 1; }
  }
  return total;
}

fn mul_divider_packet_indexes(pairs: &Vec<(ListType, ListType)>) -> u32 {
  let mut packets = Vec::new();
  for (left, right) in pairs.iter() {
    packets.push(left.clone());
    packets.push(right.clone());
  }
  let divider1 = ListType::List(vec![ListType::List(vec![ListType::Value(2)])]);
  let divider2 = ListType::List(vec![ListType::List(vec![ListType::Value(6)])]);
  packets.push(divider1.clone());
  packets.push(divider2.clone());
  packets.sort_by(|a, b| a.cmp(b));

  let mut result = 1;
  for i in 0..packets.len() {
    if packets[i] == divider1 { result *= i as u32 + 1; }
    if packets[i] == divider2 { result *= i as u32 + 1; }
  }
  return result;
}

fn main() {
  let contents = fs::read_to_string("./data/q13.txt")
    .expect("Should have been able to read file");
  let lines = contents.split('\n').map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
  let mut pairs = Vec::new();

  for i in (0..lines.len()).step_by(3) {
    let left = parse_list_str(&lines[i], &mut 0);
    let right = parse_list_str(&lines[i + 1], &mut 0);
    pairs.push((left, right));
  }

  println!("Part 1: {}", sum_correct_pair_indexes(&pairs));
  println!("Part 2: {}", mul_divider_packet_indexes(&pairs));
}