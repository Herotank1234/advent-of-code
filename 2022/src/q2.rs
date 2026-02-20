use std::fs;

fn get_score(round: &(char, char)) -> u32 {
  let move_score: u32;
  let outcome_score: u32;

  match round.1 {
    'X' => {
      move_score = 1;
      match round.0 {
        'A' => outcome_score = 3,
        'B' => outcome_score = 0,
        'C' => outcome_score = 6,
        _ => panic!("Opponent move not recognised: {}", round.0)
      }
    },
    'Y' => {
      move_score = 2;
      match round.0 {
        'A' => outcome_score = 6,
        'B' => outcome_score = 3,
        'C' => outcome_score = 0,
        _ => panic!("Opponent move not recognised: {}", round.0)
      }
    },
    'Z' => {
      move_score = 3;
      match round.0 {
        'A' => outcome_score = 0,
        'B' => outcome_score = 6,
        'C' => outcome_score = 3,
        _ => panic!("Opponent move not recognised: {}", round.0)
      }
    },
    _ => panic!("My move not recognised: {}", round.1)
  }
  
  return move_score + outcome_score;
}

fn get_total_score(rounds: &Vec<(char, char)>) -> u32 {
  return rounds.iter().map(|round| get_score(round)).sum();
}

fn get_score_updated(round: &(char, char)) -> u32 {
  let move_score: u32;
  let outcome_score: u32;

  match round.1 {
    'X' => {
      outcome_score = 0;
      match round.0 {
        'A' => move_score = 3,
        'B' => move_score = 1,
        'C' => move_score = 2,
        _ => panic!("Opponent move not recognised: {}", round.0)
      }
    },
    'Y' => {
      outcome_score = 3;
      match round.0 {
        'A' => move_score = 1,
        'B' => move_score = 2,
        'C' => move_score = 3,
        _ => panic!("Opponent move not recognised: {}", round.0)
      }
    },
    'Z' => {
      outcome_score = 6;
      match round.0 {
        'A' => move_score = 2,
        'B' => move_score = 3,
        'C' => move_score = 1,
        _ => panic!("Opponent move not recognised: {}", round.0)
      }
    },
    _ => panic!("My move not recognised: {}", round.1)
  }
  
  return move_score + outcome_score;
}

fn get_total_score_updated(rounds: &Vec<(char, char)>) -> u32 {
  return rounds.iter().map(|round| get_score_updated(round)).sum();
}

fn main() {
  let contents = fs::read_to_string("./data/q2.txt")
    .expect("Should have been able to read file");
  let lines = contents.split('\n').collect::<Vec<&str>>();
  let rounds = lines.iter().map(|line| {
    let curr_line = line.chars().collect::<Vec<char>>();
    return (curr_line[0], curr_line[2]);
  }).collect::<Vec<(char, char)>>();

  println!("Part 1: {}", get_total_score(&rounds));
  println!("Part 2: {}", get_total_score_updated(&rounds));
}