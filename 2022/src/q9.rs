use std::{collections::HashSet, fs};

const HEAD: usize = 0;

struct Move {
  direction: (i32, i32),
  iterations: u32
}

struct Snake {
  sections: Vec<(i32, i32)>,
  unique_tail_tiles: HashSet<(i32, i32)>
}

impl Snake {
  fn new(len: u32) -> Self {
    return Snake{
      sections: vec![(0, 0); len as usize],
      unique_tail_tiles: HashSet::new()
    };
  }

  fn get_unique_tail_tiles_len(&self) -> usize {
    return self.unique_tail_tiles.len();
  }

  fn move_snake(&mut self, m: &Move) {
    for _ in 0..m.iterations {
      self.sections[HEAD].0 += m.direction.0;
      self.sections[HEAD].1 += m.direction.1;

      for i in 0..self.sections.len() - 1 {
        let new_position = {
          let sec1 = &self.sections[i];
          let mut sec2 = self.sections[i + 1];

          let mut di = sec1.0 - sec2.0;
          let mut dj = sec1.1 - sec2.1;

          if di.abs() == 2 || dj.abs() == 2 {
            if di.abs() == 2 { di /= 2; }
            if dj.abs() == 2 { dj /= 2; }
            sec2.0 += di;
            sec2.1 += dj;
          }
          sec2
        };
        if self.sections[i + 1] == new_position { break; } 
        self.sections[i + 1] = new_position;
      }

      self.unique_tail_tiles.insert(*self.sections.last().unwrap());
    }
  }
}

fn count_tail_unique_tiles(moves: &Vec<Move>, snake_len: u32) -> usize {
  let mut snake = Snake::new(snake_len);
  for m in moves.iter() {
    snake.move_snake(m);
  }
  return snake.get_unique_tail_tiles_len();
}

fn main() {
  let contents = fs::read_to_string("./data/q9.txt")
    .expect("Should have been able to read file");
  let moves = contents.split('\n').map(|line| {
    let parts = line.split(' ').collect::<Vec<&str>>();
    let direction = match parts[0] {
      "R" => (0, 1),
      "D" => (1, 0),
      "L" => (0, -1),
      "U" => (-1, 0),
      _ => panic!("Not a valid move: {}", parts[0])
    };
    let iterations = parts[1].parse::<u32>().unwrap();
    return Move{direction, iterations};
  }).collect::<Vec<Move>>();

  println!("Part 1: {}", count_tail_unique_tiles(&moves, 2));
  println!("Part 2: {}", count_tail_unique_tiles(&moves, 10));
}