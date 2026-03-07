use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Monkey {
  Value(Option<u64>),
  Add(String, String),
  Sub(String, String),
  Mul(String, String),
  Div(String, String)
}

fn eval(name: &String, monkeys: &HashMap<String, Monkey>) -> Option<u64> {
  let monkey = monkeys.get(name).unwrap();
  match monkey {
    Monkey::Value(val) => return *val,
    Monkey::Add(lhs_name, rhs_name) => {
      let Some(lhs_res) = eval(lhs_name, monkeys) else { return None; };
      let Some(rhs_res) = eval(rhs_name, monkeys) else { return None; };
      return Some(lhs_res + rhs_res);
    }
    Monkey::Sub(lhs_name, rhs_name) => {
      let Some(lhs_res) = eval(lhs_name, monkeys) else { return None; };
      let Some(rhs_res) = eval(rhs_name, monkeys) else { return None; };
      return Some(lhs_res - rhs_res);
    }
    Monkey::Mul(lhs_name, rhs_name) => {
      let Some(lhs_res) = eval(lhs_name, monkeys) else { return None; };
      let Some(rhs_res) = eval(rhs_name, monkeys) else { return None; };
      return Some(lhs_res * rhs_res);
    }
    Monkey::Div(lhs_name, rhs_name) => {
      let Some(lhs_res) = eval(lhs_name, monkeys) else { return None; };
      let Some(rhs_res) = eval(rhs_name, monkeys) else { return None; };
      return Some(lhs_res / rhs_res);
    }
  }
}

fn get_root_value(monkeys: &HashMap<String, Monkey>) -> u64 {
  let root = String::from("root");
  let Some(result) = eval(&root, monkeys) else { panic!("Result is None"); };
  return result;
}

fn back_eval(name: &String, monkeys: &HashMap<String, Monkey>, answer: u64) -> u64 {
  if name == "humn" { return answer; }

  let monkey = monkeys.get(name).unwrap();
  let result = match monkey {
    Monkey::Add(lhs_name, rhs_name) => {
      let lhs_res = eval(lhs_name, monkeys);
      let rhs_res = eval(rhs_name, monkeys);
      if lhs_res == None {
        let Some(val) = rhs_res else { panic!("Only one side should be None"); };
        back_eval(lhs_name, monkeys, answer - val)
      } else {
        let Some(val) = lhs_res else { panic!("Only one side should be None"); };
        back_eval(rhs_name, monkeys, answer - val)
      }
    }
    Monkey::Sub(lhs_name, rhs_name) => {
      let lhs_res = eval(lhs_name, monkeys);
      let rhs_res = eval(rhs_name, monkeys);
      if lhs_res == None {
        let Some(val) = rhs_res else { panic!("Only one side should be None"); };
        back_eval(lhs_name, monkeys, answer + val)
      } else {
        let Some(val) = lhs_res else { panic!("Only one side should be None"); };
        back_eval(rhs_name, monkeys, val - answer)
      }
    }
    Monkey::Mul(lhs_name, rhs_name) => {
      let lhs_res = eval(lhs_name, monkeys);
      let rhs_res = eval(rhs_name, monkeys);
      if lhs_res == None {
        let Some(val) = rhs_res else { panic!("Only one side should be None"); };
        back_eval(lhs_name, monkeys, answer / val)
      } else {
        let Some(val) = lhs_res else { panic!("Only one side should be None"); };
        back_eval(rhs_name, monkeys, answer / val)
      }
    }
    Monkey::Div(lhs_name, rhs_name) => {
      let lhs_res = eval(lhs_name, monkeys);
      let rhs_res = eval(rhs_name, monkeys);
      if lhs_res == None {
        let Some(val) = rhs_res else { panic!("Only one side should be None"); };
        back_eval(lhs_name, monkeys, answer * val)
      } else {
        let Some(val) = lhs_res else { panic!("Only one side should be None"); };
        back_eval(rhs_name, monkeys, val / answer)
      }
    }
    _ => panic!("Back eval only on expressions")
  };
  return result;
}

fn get_humn_value(monkeys: &mut HashMap<String, Monkey>) -> u64 {
  *monkeys.get_mut(&String::from("humn")).unwrap() = Monkey::Value(None);
  let root = monkeys.get(&String::from("root")).unwrap();
  let result = match root {
    Monkey::Add(lhs_name, rhs_name) | Monkey::Sub(lhs_name, rhs_name) |
      Monkey::Mul(lhs_name, rhs_name) | Monkey::Div(lhs_name, rhs_name) =>
    {
      let lhs = eval(lhs_name, monkeys);
      let rhs = eval(rhs_name, monkeys);
      if lhs == None {
        let Some(answer) = rhs else { panic!("Only one side should be None"); };
        back_eval(lhs_name, monkeys, answer)
      } else {
        let Some(answer) = lhs else { panic!("Only one side should be None"); };
        back_eval(rhs_name, monkeys, answer)
      }
    },
    _ => panic!("Root should be an expression")
  };
  return result;
}

fn main() {
  let contents = fs::read_to_string("./data/q21.txt")
    .expect("Should have been able to read file");
  let mut monkeys = contents.split('\n').map(|line| {
    let parts = line.split(' ').collect::<Vec<&str>>();
    let name = parts[0][..parts[0].len() - 1].to_string();
    let monkey = if parts.len() == 2 {
      Monkey::Value(Some(parts[1].trim().parse::<u64>().unwrap()))
    } else {
      let lhs = parts[1].trim().to_string();
      let rhs = parts[3].to_string();
      match parts[2] {
        "+" => Monkey::Add(lhs, rhs),
        "-" => Monkey::Sub(lhs, rhs),
        "*" => Monkey::Mul(lhs, rhs),
        "/" => Monkey::Div(lhs, rhs),
        _ => panic!("Unrecognised operation: {}", parts[2])
      }
    };
    (name, monkey)
  }).collect::<HashMap<String, Monkey>>();

  println!("Part 1: {}", get_root_value(&monkeys));
  println!("Part 2: {}", get_humn_value(&mut monkeys));
}