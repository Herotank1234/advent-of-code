use std::fs;

const IMPORTANT_CYCLES: &[i32; 6] = &[20, 60, 100, 140, 180, 220];
const SCREEN_WIDTH: usize = 40;

enum Instruction {
  Noop,
  Addx(i32)
}

enum State {
  Free,
  MidAdd,
}

struct CPU {
  reg: i32,
  cycle: i32,
  state: State,
  signal_strengths: Vec<i32>,
  pixel_data: Vec<char>
}
 
impl CPU {
  fn new() -> Self {
    return CPU{reg: 1, cycle: 0, state: State::Free, signal_strengths: Vec::new(), pixel_data: Vec::new()};
  }

  fn simulate(&mut self, instructions: &Vec<Instruction>) {
    let mut index = 0;
    while index < instructions.len() {
      let sprite_pos = self.cycle % SCREEN_WIDTH as i32;
      if sprite_pos >= self.reg - 1 && sprite_pos <= self.reg + 1 {
        self.pixel_data.push('#');
      } else {
        self.pixel_data.push('.');
      }

      self.cycle += 1;
      if IMPORTANT_CYCLES.contains(&self.cycle) { self.signal_strengths.push(self.cycle * self.reg); }

      match instructions[index] {
        Instruction::Noop => index += 1,
        Instruction::Addx(val) => {
          match self.state {
            State::Free => self.state = State::MidAdd,
            State::MidAdd => {
              self.state = State::Free;
              self.reg += val;
              index += 1;
            }
          }
        }
      }
    } 
  }

  fn get_signal_strengths(&self) -> &Vec<i32> {
    return &self.signal_strengths;
  }

  fn get_pixel_data(&self) -> &Vec<char> {
    return &self.pixel_data;
  }
}

fn main() {
  let contents = fs::read_to_string("./data/q10.txt")
    .expect("Should have been able to read file");
  let lines = contents.split('\n').collect::<Vec<&str>>();
  let instructions = lines.iter().map(|line| {
    if *line == "noop" {
      Instruction::Noop
    } else {
      let parts = line.split(' ').collect::<Vec<&str>>();
      Instruction::Addx(parts[1].parse::<i32>().unwrap())
    }
  }).collect::<Vec<Instruction>>();

  let mut cpu = CPU::new();
  cpu.simulate(&instructions);
  
  let total_signal_strengths: i32 = cpu.get_signal_strengths().iter().sum();
  let pixel_data = cpu.get_pixel_data();
  let mut rows = Vec::new();
  for i in (0..pixel_data.len()).step_by(SCREEN_WIDTH) {
    rows.push(pixel_data[i..i + SCREEN_WIDTH].iter().collect::<String>());
  }

  println!("Part 1: {}", total_signal_strengths);
  println!("Part 2:");
  for row in rows.iter() {
    println!("{}", row);
  }
}