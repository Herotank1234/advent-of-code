use std::fs;

struct MixList {
  list: Vec<(i64, usize)>,
}

impl MixList {
  fn new(list: Vec<i64>) -> Self {
    let mut index = 0;
    let id_list = list.iter().map(|val| {
      let pair = (*val, index);
      index += 1;
      pair
    }).collect::<Vec<(i64, usize)>>();
    return MixList{list: id_list};
  }

  fn mix(&mut self) {
    let mut next_list = self.list.clone();
    for id in 0..self.list.len() {
      let mut index = next_list.iter().position(|(_, other_id)| id == *other_id)
        .unwrap() as isize;
      let (value, _) = next_list[index as usize];
      next_list.remove(index as usize);

      index += value as isize;
      index %= next_list.len() as isize;
      if index < 0 {
        index += next_list.len() as isize;
      }
      if index > next_list.len() as isize {
        index -= next_list.len() as isize;
      }

      next_list.insert(index as usize, (value, id));
    }
    self.list = next_list;
  }

  fn get_after_0(&self, index: usize) -> i64 {
    let zero_index = self.list.iter().position(|(val, _)| *val == 0)
      .unwrap();
    return self.list[(zero_index + index) % self.list.len()].0;
  }
}

fn mix_and_sum_vals(nums: &Vec<i64>) -> i64 {
  let mut mix_list = MixList::new(nums.clone());
  mix_list.mix();
  return mix_list.get_after_0(1000) + mix_list.get_after_0(2000) +
    mix_list.get_after_0(3000);
}

fn mix_with_decrypt(nums: &Vec<i64>) -> i64 {
  let decrypt_key = 811589153;
  let next_nums = nums.iter().map(|num| num * decrypt_key).collect::<Vec<i64>>();
  let mut mix_list = MixList::new(next_nums);
  for _ in 0..10 {
    mix_list.mix();
  }
  return mix_list.get_after_0(1000) + mix_list.get_after_0(2000) +
    mix_list.get_after_0(3000);
}

fn main() {
  let contents = fs::read_to_string("./data/q20.txt")
    .expect("Should have been able to read file");
  let nums = contents.split('\n').map(|line|
    line.parse::<i64>().unwrap()
  ).collect::<Vec<i64>>();

  println!("Part 1: {}", mix_and_sum_vals(&nums));
  println!("Part 2: {}", mix_with_decrypt(&nums));
}