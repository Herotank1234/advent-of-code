use std::{
  collections::HashSet,
  fs
};

fn get_start_of_packet_pos(packet: &Vec<char>, win_size: usize) -> usize {
  let mut curr_letters = HashSet::new();
  let mut l = 0;
  let mut r = 0;
  while curr_letters.len() < win_size {
    let curr_letter = &packet[r];
    while curr_letters.contains(curr_letter) {
      curr_letters.remove(&packet[l]);
      l += 1;
    }
    curr_letters.insert(packet[r]);
    r += 1;
  }
  return r;
}

fn main() {
  let contents = fs::read_to_string("./data/q6.txt")
    .expect("Should have been able to read file");
  let packet = contents.chars().collect::<Vec<char>>();
  println!("Part 1: {}", get_start_of_packet_pos(&packet, 4));
  println!("Part 2: {}", get_start_of_packet_pos(&packet, 14));
}