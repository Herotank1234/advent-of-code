use std::{fs, collections::HashSet};
use regex::Regex;

struct Sensor {
  x: i64,
  y: i64,
  beac_x: i64,
  beac_y: i64,
  man_dist: i64,
}

impl Sensor {
  fn new(x: i64, y: i64, beac_x: i64, beac_y: i64) -> Self {
    let man_dist = (x - beac_x).abs() + (y - beac_y).abs();
    return Sensor{x, y, beac_x, beac_y, man_dist};
  }
}

fn get_beacon_ranges(sensors: &Vec<Sensor>, row: &i64) -> Vec<(i64, i64)> {
  let mut ranges = Vec::new();

  for sensor in sensors.iter() {
    let y_dist = (sensor.y - *row).abs();
    let rem_dist = sensor.man_dist - y_dist;
    if rem_dist <= 0 { continue; }
    ranges.push((sensor.x - rem_dist, sensor.x + rem_dist));
  }

  ranges.sort_by(|(l1, _), (l2, _)| l1.cmp(l2));

  let mut index = 0;
  while index < ranges.len() - 1 {
    let (l1, u1) = &ranges[index];
    let (l2, u2) = &ranges[index + 1];
    if l1 <= l2 && u2 <= u1 {
      ranges.remove(index + 1);
    } else if l1 <= l2 && l2 <= u1 {
      ranges[index] = (*l1, *u2);
      ranges.remove(index + 1);
    } else {
      index += 1;
    }
  }

  return ranges;
}

fn count_non_beacon_tiles_on_row(sensors: &Vec<Sensor>, row: &i64) -> i64 {
  let ranges = get_beacon_ranges(sensors, row);
  let beacons_on_row = sensors.iter().filter_map(|sensor| {
    if sensor.beac_y == *row { Some((sensor.beac_x, sensor.beac_y)) } else { None }
  }).collect::<HashSet<(i64, i64)>>();
  return ranges.iter().map(|(lower, upper)| upper - lower + 1).sum::<i64>() - (beacons_on_row.len() as i64);
}

fn find_distress_beacon(sensors: &Vec<Sensor>) -> i64 {
  for i in 0..=4_000_000 {
    let ranges = get_beacon_ranges(sensors, &i);
    if ranges.len() != 1 {
      return (ranges[0].1 + 1) * 4_000_000 + i;
    }
  }
  return 0;
}

fn main() {
  let contents = fs::read_to_string("./data/q15.txt")
    .expect("Should have been able to read file");
  let re = Regex::new(r"Sensor at x=(-?[\d]+), y=(-?[\d]+): closest beacon is at x=(-?[\d]+), y=(-?[\d]+)").unwrap();
  let sensors = contents.split('\n').map(|line| {
    let caps = re.captures(line).unwrap();
    let x = caps[1].to_string().parse::<i64>().unwrap();
    let y = caps[2].to_string().parse::<i64>().unwrap();
    let beac_x = caps[3].to_string().parse::<i64>().unwrap();
    let beac_y = caps[4].to_string().parse::<i64>().unwrap();
    Sensor::new(x, y, beac_x, beac_y)
  }).collect::<Vec<Sensor>>();

  println!("Part 1: {}", count_non_beacon_tiles_on_row(&sensors, &2_000_000));
  println!("Part 2: {}", find_distress_beacon(&sensors));
}