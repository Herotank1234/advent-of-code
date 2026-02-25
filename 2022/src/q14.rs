use std::{cmp::{max, min}, collections::HashSet, fs};

const SOURCE_I: u32 = 0;
const SOURCE_J: u32 = 500;

fn count_sand_until_void(rocks: &HashSet<(u32, u32)>) -> u32 {
  let mut sands = HashSet::new();
  let max_i = rocks.iter().fold(0, |acc, (i, _)| acc.max(*i));
  let mut is_abyss = false;

  while !is_abyss {
    let mut i = SOURCE_I;
    let mut j = SOURCE_J;

    let mut movable = true;
    while movable {
      if i == max_i {
        is_abyss = true;
        break;
      }
      movable = false;

      let mut next_i = i + 1;
      let mut next_j = j;
      if !rocks.contains(&(next_i, next_j)) && !sands.contains(&(next_i, next_j)) {
        movable = true;
        i = next_i;
        j = next_j;
        continue;
      }

      next_i = i + 1;
      next_j = j - 1;
      if !rocks.contains(&(next_i, next_j)) && !sands.contains(&(next_i, next_j)) {
        movable = true;
        i = next_i;
        j = next_j;
        continue;
      }

      next_i = i + 1;
      next_j = j + 1;
      if !rocks.contains(&(next_i, next_j)) && !sands.contains(&(next_i, next_j)) {
        movable = true;
        i = next_i;
        j = next_j;
        continue;
      }
    }

    if !is_abyss { sands.insert((i, j)); }
  }

  return sands.len() as u32;
}

fn count_sand_until_source_blocked(rocks: &HashSet<(u32, u32)>) -> u32 {
  let mut sands = HashSet::new();
  let max_i = rocks.iter().fold(0, |acc, (i, _)| acc.max(*i)) + 1;
  let mut source_blocked = false;

  while !source_blocked {
    let mut i = SOURCE_I;
    let mut j = SOURCE_J;

    let mut movable = true;
    while movable {
      if i == max_i { break; }
      movable = false;

      let mut next_i = i + 1;
      let mut next_j = j;
      if !rocks.contains(&(next_i, next_j)) && !sands.contains(&(next_i, next_j)) {
        movable = true;
        i = next_i;
        j = next_j;
        continue;
      }

      next_i = i + 1;
      next_j = j - 1;
      if !rocks.contains(&(next_i, next_j)) && !sands.contains(&(next_i, next_j)) {
        movable = true;
        i = next_i;
        j = next_j;
        continue;
      }

      next_i = i + 1;
      next_j = j + 1;
      if !rocks.contains(&(next_i, next_j)) && !sands.contains(&(next_i, next_j)) {
        movable = true;
        i = next_i;
        j = next_j;
        continue;
      }
    }

    sands.insert((i, j));
    if i == SOURCE_I && j == SOURCE_J { source_blocked = true; }
  }

  return sands.len() as u32;
}

fn main() {
  let contents = fs::read_to_string("./data/q14.txt")
    .expect("Should have been able to read file");
  let rock_coords = contents.split('\n').map(|line| {
    line.split(' ').filter(|s| *s != "->").map(|s| {
      let parts = s.split(',').map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>();
      (parts[0], parts[1])
    }).collect()
  }).collect::<Vec<Vec<(u32, u32)>>>();

  let mut rocks = HashSet::new();
  for rock_coord in rock_coords.iter() {
    for index in 0..rock_coord.len() - 1 {
      let (j1, i1) = &rock_coord[index];
      let (j2, i2) = &rock_coord[index + 1];
      let smaller_j = *min(j1, j2);
      let bigger_j = *max(j1, j2);
      let smaller_i = *min(i1, i2);
      let bigger_i = *max(i1, i2);
      for i in smaller_i..=bigger_i {
        for j in smaller_j..=bigger_j {
          rocks.insert((i, j));
        }
      }
    }
  }

  println!("Part 1: {}", count_sand_until_void(&rocks));
  println!("Part 2: {}", count_sand_until_source_blocked(&rocks));
}