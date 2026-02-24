use std::{
  collections::{
    HashMap,
    VecDeque
  },
  fs
};

#[derive(Clone, Debug)]
enum Ops {
  Add(u64),
  Mul(u64),
  Square
}

#[derive(Clone, Debug)]
struct Monkey {
  items: VecDeque<u64>,
  op: Ops,
  divisible: u64,
  if_true: u64,
  if_false: u64
}

fn simulate(mut monkeys: HashMap<u64, Monkey>, rounds: u64, relief: bool) -> u64 {
  let mut inspected = monkeys.keys().map(|key| (*key, 0_u64)).collect::<HashMap<u64, u64>>();
  let modulo = monkeys.values().fold(1, |acc, m| acc * m.divisible);
  
  for _ in 0..rounds {
    for monkey_index in 0..monkeys.len() as u64 {
      let curr_monkey = monkeys.get_mut(&monkey_index).unwrap();
      let mut throw_queue = VecDeque::new();

      while !curr_monkey.items.is_empty() {
        let mut item = curr_monkey.items.pop_front().unwrap();
        *inspected.get_mut(&monkey_index).unwrap() += 1;
        match curr_monkey.op {
          Ops::Add(val) => item += val,
          Ops::Mul(val) => item *= val,
          Ops::Square => item *= item
        }

        if relief {
          item /= 3;
        } else {
          item %= modulo;
        }

        if item % curr_monkey.divisible == 0 {
          throw_queue.push_back((curr_monkey.if_true, item));
        } else {
          throw_queue.push_back((curr_monkey.if_false, item));
        }
      }

      while !throw_queue.is_empty() {
        let (id, item) = throw_queue.pop_front().unwrap();
        monkeys.get_mut(&id).unwrap().items.push_back(item);
      }
    }
  }

  let mut busiest_two = inspected.values().collect::<Vec<&u64>>();
  busiest_two.sort_by(|a, b| b.cmp(a));
  return busiest_two.iter().take(2).fold(1, |acc, val| acc * *val);
}

fn main() {
  let contents = fs::read_to_string("./data/q11.txt")
    .expect("Should have been able to read file");
  let lines = contents.split('\n').collect::<Vec<&str>>();
  let mut monkeys = HashMap::new();

  for i in (0..lines.len()).step_by(7) {
    let id = lines[i].chars().nth_back(1).unwrap().to_digit(10).unwrap().into();

    let items = lines[i + 1].trim().split(' ').skip(2).map(|item| {
      let val_str = item.trim_end_matches(',');
      val_str.parse::<u64>().unwrap()
    }).collect::<VecDeque<u64>>();

    let op_parts = lines[i + 2].split(' ').collect::<Vec<&str>>();
    let op = {
      let other_val_str = op_parts[op_parts.len() - 1];
      if op_parts[op_parts.len() - 2] == "+" {
        let other_val = other_val_str.parse::<u64>().unwrap();
        Ops::Add(other_val)
      } else {
        if other_val_str == "old" {
          Ops::Square
        } else {
          let other_val = other_val_str.parse::<u64>().unwrap();
          Ops::Mul(other_val)
        }
      }
    };

    let divisible = lines[i + 3].split(' ').last().unwrap().parse::<u64>().unwrap();
    let if_true = lines[i + 4].split(' ').last().unwrap().parse::<u64>().unwrap();
    let if_false = lines[i + 5].split(' ').last().unwrap().parse::<u64>().unwrap();
    monkeys.insert(id, Monkey{items, op, divisible, if_true, if_false});
  }

  println!("Part 1: {}", simulate(monkeys.clone(), 20, true));
  println!("Part 2: {}", simulate(monkeys.clone(), 10000, false));
}