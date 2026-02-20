use std::{
  fs,
  collections::HashSet
};

const LOWER_SCORING: u32 = 1;
const UPPER_SCORING: u32 = 27;

fn sum_priorities(backpacks: &Vec<Vec<char>>) -> u32 {
  let mut priorities = 0;

  for backpack in backpacks.iter() {
    let half_len = backpack.len() / 2;
    let letters: HashSet<&char> = HashSet::from_iter(&backpack[..half_len]);
    for letter in backpack[half_len..].iter() {
      if letters.contains(letter) {
        if letter.is_ascii_lowercase() {
          priorities += (*letter as u32) - ('a' as u32) + LOWER_SCORING;
          break;
        } else {
          priorities += (* letter as u32) - ('A' as u32) + UPPER_SCORING;
          break;
        }
      }
    }
  }

  return priorities;
}

fn sum_priorities_three_backpacks(backpacks: &Vec<Vec<char>>) -> u32 {
  let mut priorities = 0;

  for i in (0..backpacks.len()).step_by(3) {
    let b1 = &backpacks[i];
    let b2 = &backpacks[i + 1];
    let b3 = &backpacks[i + 2];

    let b1_letters: HashSet<&char> = HashSet::from_iter(&b1[..]);
    let b2_letters: HashSet<&char> = b2.iter().filter(|letter| {
      return b1_letters.contains(letter);
    }).collect::<HashSet<&char>>();
    for letter in b3.iter() {
      if b2_letters.contains(letter) {
        if letter.is_ascii_lowercase() {
          priorities += (*letter as u32) - ('a' as u32) + LOWER_SCORING;
          break;
        } else {
          priorities += (* letter as u32) - ('A' as u32) + UPPER_SCORING;
          break;
        }
      }
    }
  }

  return priorities;
}

fn main() {
  let contents = fs::read_to_string("./data/q3.txt")
    .expect("Should have been able to read file");
  let backpacks = contents.split('\n').map(|line| line.chars().collect())
    .collect::<Vec<Vec<char>>>();
  println!("Part 1: {}", sum_priorities(&backpacks));
  println!("Part 2: {}", sum_priorities_three_backpacks(&backpacks));
}