use std::collections::HashMap;

type Position = (u64, u64);

static DEPTH: u64 = 10689;
static TARGET: Position = (11, 722);

fn geologic_index(p: Position, target: Position, lut: &HashMap<Position, u64>) -> u64 {
  if p.0 == 0 && p.1 == 0 {
    0
  } else if p.0 == target.0 && p.1 == target.1 {
    0
  } else if p.0 == 0 {
    p.1 * 48271
  } else if p.1 == 0 {
    p.0 * 16807
  } else {
    lut[&(p.0 - 1, p.1)] * lut[&(p.0, p.1 - 1)]
  }
}

fn erosion_level(p: Position, depth: u64, target: Position, lut: &mut HashMap<Position, u64>) -> u64 {
  let index = (geologic_index(p, target, lut) + depth) % 20183;
  lut.insert(p, index);
  index
}

fn risk_level(depth: u64, target: Position) -> u64 {
  let mut risk_level = 0;
  let mut lut = HashMap::new();
  for x in 0..target.0+1 {
    for y in 0..target.1+1 {
      risk_level += erosion_level((x, y), depth, target, &mut lut) % 3;
    }
  }

  risk_level
}

pub fn problem1() {
  let result = risk_level(DEPTH, TARGET);
  println!("Risk level: {}", result);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_example() {
    assert_eq!(risk_level(510, (10, 10)), 114);
  }
}