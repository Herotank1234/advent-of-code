use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Lens {
  name: String,
  focus: u32
}

fn get_hash(line: &str) -> u32 {
  let mut hash = 0;
  for c in line.chars() {
    hash += c as u32;
    hash *= 17;
    hash %= 256;
  }
  return hash;
}

fn get_hash_sum(lines: &Vec<&str>) -> u32 {
  return lines.iter().map(|line| get_hash(line)).sum();
}

fn calculate_focusing_power(lines: &Vec<&str>) -> u32 {
  let mut boxes: HashMap<u32, Vec<Lens>> = HashMap::new();
  for i in 0..=255 {
    boxes.insert(i, Vec::new());
  }

  for line in lines {
    if line.contains('-') {
      let name = line[..line.len() - 1].to_string();
      let box_num = get_hash(&name);
      let curr_box = boxes.get_mut(&box_num).unwrap();
      for i in 0..curr_box.len() {
        if curr_box[i].name == name {
          curr_box.remove(i);
          break;
        }
      }
    } else {
      let parts = line.split('=').collect::<Vec<&str>>();
      let name = parts[0].to_string();
      let focus = parts[1].parse::<u32>().unwrap();
      let box_num = get_hash(&name);
      let curr_box = boxes.get_mut(&box_num).unwrap();
      let mut found = false;
      for i in 0..curr_box.len() {
        if curr_box[i].name == name {
          curr_box[i].focus = focus;
          found = true;
          break;
        }
      }
      if found { continue; }
      curr_box.push(Lens{name, focus});
    }
  }

  let mut total_focus = 0;
  for (box_num, lenses) in boxes.iter() {
    for i in 0..lenses.len() {
      total_focus += (box_num + 1) * (i as u32 + 1) * lenses[i].focus;
    }
  }
  return total_focus;
}

fn main() {
  let contents = fs::read_to_string("./data/q15.txt")
    .expect("Should have been able to read file");
  let lines = contents.split(',').collect::<Vec<&str>>();
  println!("Part 1: {}", get_hash_sum(&lines));
  println!("Part 2: {}", calculate_focusing_power(&lines));
}