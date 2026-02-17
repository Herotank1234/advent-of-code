use std::{fs, ops::{Sub, Add}};
use regex::Regex;
use num_bigint::{ToBigInt, BigInt};

const LOWER: f64 = 200000000000000.0;
const UPPER: f64 = 400000000000000.0;

#[derive(Debug)]
struct Hailstone {
  x: i64,
  y: i64,
  z: i64,
  dx: i64,
  dy: i64,
  dz: i64,
  m: f64,
  c: f64
}

#[derive(Clone, Debug)]
struct Vec3 {
  x: BigInt,
  y: BigInt,
  z: BigInt
}

impl Vec3 {
  fn new(x: BigInt, y: BigInt, z: BigInt) -> Self {
    return Self{x, y, z};
  }
}

impl Sub for Vec3 {
  type Output = Vec3;
  fn sub(self, rhs: Self) -> Vec3 { 
    return Vec3::new(&self.x - &rhs.x, &self.y - &rhs.y, &self.z - &rhs.z);
  }
}

impl Add for Vec3 {
  type Output = Vec3;
  fn add(self, rhs: Self) -> Vec3 {
    return Vec3::new(&self.x + &rhs.x, &self.y + &rhs.y, &self.z + &rhs.z);
  }
}

impl Hailstone {
  fn new(x: i64, y: i64, z: i64, dx: i64, dy: i64, dz: i64) -> Self {
    let m = dy as f64 / dx as f64;
    let c = y as f64 - (m * x as f64);
    return Self{x, y, z, dx, dy, dz, m, c};
  }
}

fn count_intersecting(hailstones: &Vec<Hailstone>) -> u32 {
  let mut intersecting = 0;

  for i in 0..hailstones.len() - 1 {
    for j in i + 1..hailstones.len() {
      let stone_1 = &hailstones[i];
      let stone_2 = &hailstones[j];

      if stone_1.m == stone_2.m { continue; }
      /* 
        m1x + c1 = m2x + c2
        x(m1 - m2) = c2 - c1
        x = (c2 - c1) / (m1 - m2)
        y = m1x + c
      */
      let x = (stone_2.c - stone_1.c) / (stone_1.m - stone_2.m);
      let y = (stone_1.m * x) + stone_1.c;
      if x < LOWER || x > UPPER || y < LOWER || y > UPPER { continue; }
      if x < stone_1.x as f64 && stone_1.dx > 0 { continue; }
      if x > stone_1.x as f64 && stone_1.dx < 0 { continue; }
      if x < stone_2.x as f64 && stone_2.dx > 0 { continue; }
      if x > stone_2.x as f64 && stone_2.dx < 0 { continue; }
      intersecting += 1;
    }
  }

  return intersecting;
}

fn cross_product(left: &Vec3, right: &Vec3) -> Vec3 {
  let x = &left.y * &right.z - &left.z * &right.y;
  let y = &left.z * &right.x - &left.x * &right.z;
  let z = &left.x * &right.y - &left.y * &right.x;
  return Vec3::new(x, y, z);
}

fn dot_product(left: &Vec3, right: &Vec3) -> BigInt {
  return &left.x * &right.x + &left.y * &right.y + &left.z * &right.z;
}

fn scalar_mul(s: &BigInt, v: &Vec3) -> Vec3 {
  return Vec3::new(&v.x * s, &v.y * s, &v.z * s);
}

fn scalar_div(s: &BigInt, v: &Vec3) -> Vec3 {
  return Vec3::new(&v.x / s, &v.y / s, &v.z / s);
}

fn sum_initial_rock_pos(hailstones: &Vec<Hailstone>) -> BigInt {
  let stone0 = &hailstones[0];
  let stone1 = &hailstones[1];
  let stone2 = &hailstones[2];

  let p0 = Vec3::new(stone0.x.to_bigint().unwrap(), stone0.y.to_bigint().unwrap(), stone0.z.to_bigint().unwrap());
  let v0 = Vec3::new(stone0.dx.to_bigint().unwrap(), stone0.dy.to_bigint().unwrap(), stone0.dz.to_bigint().unwrap());

  let p1 = Vec3::new(stone1.x.to_bigint().unwrap(), stone1.y.to_bigint().unwrap(), stone1.z.to_bigint().unwrap());
  let v1 = Vec3::new(stone1.dx.to_bigint().unwrap(), stone1.dy.to_bigint().unwrap(), stone1.dz.to_bigint().unwrap());

  let p2 = Vec3::new(stone2.x.to_bigint().unwrap(), stone2.y.to_bigint().unwrap(), stone2.z.to_bigint().unwrap());
  let v2 = Vec3::new(stone2.dx.to_bigint().unwrap(), stone2.dy.to_bigint().unwrap(), stone2.dz.to_bigint().unwrap());

  let p1_rel = p1.clone() - p0.clone();
  let v1_rel = v1.clone() - v0.clone();

  let p2_rel = p2.clone() - p0.clone();
  let v2_rel = v2.clone() - v0.clone();

  let numerator1 = -dot_product(&cross_product(&p1_rel, &p2_rel), &v2_rel);
  let denominator1 = dot_product(&cross_product(&v1_rel, &p2_rel), &v2_rel);
  let t1 = numerator1 / denominator1;

  let numerator2 = -dot_product(&cross_product(&p1_rel, &p2_rel), &v1_rel);
  let denominator2 = dot_product(&cross_product(&p1_rel, &v2_rel), &v1_rel);
  let t2 = numerator2 / denominator2;

  let collision1 = p1.clone() + scalar_mul(&t1, &v1);
  let collision2 = p2.clone() + scalar_mul(&t2, &v2);

  let v_rock = scalar_div(&(t2 - t1.clone()), &(collision2 - collision1.clone()));
  let p_rock = collision1 - scalar_mul(&t1, &v_rock);

  return &p_rock.x + &p_rock.y + &p_rock.z;
}

fn main() {
  let contents = fs::read_to_string("./data/q24.txt")
    .expect("Should have been able to read file");
  let lines = contents.split('\n').collect::<Vec<&str>>();
  let re = Regex::new(r"([\d]+), +([\d]+), +([\d]+) +@ +(-?[\d]+), +(-?[\d]+), +(-?[\d]+)").unwrap();
  let mut hailstones = Vec::new();

  for line in lines.iter() {
    let Some(caps) = re.captures(&line) else { panic!("Bad line {}", line); };
    let vals = caps.iter().skip(1).map(|cap| cap.unwrap().as_str().parse::<i64>().unwrap())
      .collect::<Vec<i64>>();
    let [x, y, z, dx, dy, dz] = vals[..] else { panic!("Unable to pattern match {:?}", vals); };
    hailstones.push(Hailstone::new(x, y, z, dx, dy, dz));
  }

  println!("Part 1: {}", count_intersecting(&hailstones));
  println!("Part 2: {}", sum_initial_rock_pos(&hailstones));
}